# What root the lpoco tools need, seen from lpoco's one roof (2026-11-18)

The author, 2026-11-18 (turn-2026-11-18-lab-agent-root-grant-lpoco-roof): the root grant "needs to be viewed from
lpoco one roof of tools perspective, not in general like it was for lab-agent". This redoes option A/B/C of
lab-agent-still-involved.md section 5 from the tools' own needs. Read from lab-tools main (host/src, devlab/src) on
the lab 2026-11-18; nothing run as root, sudoers untouched.

## 1. What each tool asks root for (read from the source)
- **lpoco-host** (`prepare now`): write /etc/udev/rules.d/70-lpoco-CODENAME.rules and /etc/modules-load.d/lpoco.conf
  (`install -m`), `udevadm control --reload` and `trigger`, `modprobe MODULE`, a link file for a device's USB network
  (NetworkManager keyfile + `nmcli connection reload`, or a systemd-networkd file + `networkctl reload`). It prints
  the exact list (`plan`, "root commands: none" when the host already matches). Rare: when a device or a rule is new.
  Watchers are user units: no root.
- **lpoco-lab / devlab flash**: bakes files and masked services into the rootfs image through a loop device:
  `losetup --show -f`, `mount` (esp, root), `cp`, `install -m`, `ln -sf`, `sha256sum` of a root-only file,
  `umount`, `losetup -d`. Then fastboot, which runs as the user (udev rules by lpoco-host).
- **lpoco-lab / devlab provision**: the phone's USB network from the lab: `sysctl -qw net.ipv4.ip_forward=1`,
  `nft add table/chain/rule` (a table of its own), `ufw route ...` when ufw is active (`ufw status` is read as root
  today), and a guard that undoes all of it (`nft delete table`, `sysctl ... =0`). Repeated: every provisioning.
- **lpoco-bringup**: none. It builds boot images in Rust, in userspace.
- **lpoco-health, tone, lpoco**: none.
- Not lpoco's: lab-machine setup (`nix.sh`, `locale.sh`: pacman, nix) and what sessions do ad hoc (dmesg,
  /proc/PID/stack, `pacman -S`, `fastboot devices` before the udev rule). Those are the lab's setup and the
  author's hands, not the tools'.

So from the roof the root need is three kinds: (a) files and a reload under /etc, rare, author-present; (b) a loop
mount to bake an image, per flash; (c) host networking for the phone, per provisioning. (b) and (c) are not
"root for anything": each is a fixed, typed set of commands.

## 2. How lpoco should declare it once (argenv-style, one schema)
The manifest decision (2026-11-18-device-manifest-schema-from-rust) already says what a device needs is Rust in the roof,
an enum with tokens, and a device only names tokens. Root needs are the same kind of thing: one enum in the manifest
crate, `RootNeed`, with a variant per need and typed parameters (codename and module tokens are already refused unless
letters, digits, `.`, `_`, `-`; paths are built from them, never typed): `UdevRule{codename}`, `ModulesLoad`,
`UdevReload`, `Modprobe{module}`, `LinkConfig{codename}`, `BakeImage{image}`, `PhoneNet{codename}`. Every program that
needs root asks the crate for the argv of a variant (`argv()`), and the same variants render the plan, the sudoers
fragment and the check: one declaration, three readers, so what lpoco-host prints as "root commands" and what the grant
allows cannot drift. The invocation surface stays one argenv input, `LPOCO_ROOT` (the prefix), as today.

## 3. What grant shape follows
Options from this declaration, narrowest first:
1. **No standing grant for (a); a unit for (c); no root for (b).** (a) is `sudo lpoco-host prepare now` typed by the
   author with his password, when a device or rule is new (a few times a month at most): no passwordless line.
   (c) becomes a system unit installed by (a) (`lpoco-net@CODENAME.service`, started by the device's udev rule, undone
   when it goes): the phone's network needs no sudo at runtime. (b) is done without root: build the rootfs with
   `mkfs.ext4 -d DIR` or a Rust writer in lpoco-bringup (no loop, no mount), the way bringup already builds boot images.
   End state: no passwordless root for lpoco at all. Cost: three changes in lab-tools (a unit, a rootfs writer, a prepare
   that installs the unit) before the old line can go. Not proven here: that `mkfs.ext4 -d` reproduces what the bake
   does (symlinks, masked services, modes); that udev can start the network unit when the phone appears (a `SYSTEMD_WANTS`
   rule); both to be tried first (a probe each, nothing installed).
2. **One root-owned helper, one sudoers line, the enum only.** `lpoco-root` accepts a variant name and typed arguments,
   builds the argv itself from the same crate, runs nothing else. Narrow because the helper validates, not sudoers.
   Cost: a root-owned binary that must be installed and updated as root at each release (the lab's one root-owned file
   outside pacman), and a sudoers line the author writes once.
3. **Sudoers per fixed command.** Rejected from the roof: `install`, `mount` and `losetup` need arguments that sudoers
   cannot narrow without wildcards (a wildcard on `install` can write anywhere), and sudo-rs allows a wildcard only as the
   last argument (read 2026-11-18).
4. **The general wrapper (today's `lab-agent root-exec`, or a renamed `lab-root`).** What the author said not to do.
Recommendation: 1 as the direction, 2 as the bridge only if the author wants a grant while 1 is built, 3 and 4 not.
Whichever he picks, the code change in lab-tools is the same first step: the `RootNeed` enum and `LPOCO_ROOT` as the only
surface, so the default `sudo -n lab-agent root-exec --` (two constants in devlab and host, one `ufw status` call in
provision.rs, one guard example, one README line) goes away without a sweep.

## 4. What the author decides, and what is not known yet
His word: direction 1 (no standing passwordless root for lpoco; he types the password for `prepare now`), or 2 (a
helper and one line), and whether the lab's own setup (pacman, nix) stays his hands. Sudoers stays untouched until then.
Not known yet: the two probes in 3.1; whether udev starts a root unit on the phone's USB network device on this
kernel; whether `fastboot devices` works as the user today (lpoco-host check says "not measured, not in fastboot now").
