# How lab-agent is still involved, and what the job route is worth (2026-11-18)

Asked by the author through the Top Coordinator: "how is lab-agent still involved? ... a separate research,
lookaround too". Builds on research/organising-agents/2026-11-17/lab-agent-what-it-is-used-for.md (what it is,
measured use) and decision 2026-11-17-lab-agent-retires-by-parts. Measured 2026-11-18 on the lab (grig).

## 1. The counts, corrected
A grep of ~/Projects/linux-phone-oco counts the 83 worktree clones (wt-*, docs-*, lab-tools-*, surya-lab-*) too.
In the main checkouts only (no wt-*, no *-wt, no clones): `lab-agent root-exec` 90 lines in 78 files; `lab-agent guide`
54 lines; `lab-agent runs` 18; `lab-agent submit` 5 (2 files); `lab-agent job` 9 (6 files); `lab-agent inbox` 0. Of the
`root-exec` lines only 5 are code: devlab cli.rs `DEFAULT_ROOT`, host lib.rs `DEFAULT_ROOT`, devlab provision.rs
(`ufw status`), guard.rs (a rule's example text), lab-tools README. The rest are records (decisions, backlog, research,
session log data and their rendered pages) and briefs. `guide` is briefs and docs telling a session to read the
practices ("when a choice turns on one: `lab-agent guide`"); `runs` is prose ("as lab-agent runs").
Meaning: the live dependence of linux-phone-oco is one string in two programs (the root prefix) plus the briefs'
words. Records stay as history; they are not retired, they are dated.

## 2. What each use is for, still needed, where it lives instead
- root prefix (`sudo -n lab-agent root-exec --`, devlab flash/provision/guard, lpoco-host, setup/nix.sh, locale.sh):
  passwordless root for fastboot, ufw, pacman, udev, dmesg. Needed. One sudoers line, /etc/sudoers.d/lab-agent, to the
  root-owned /usr/bin/lab-agent (the ~/.local/bin copy is not granted). Replacement: see 5.
- `lab-agent guide` in briefs: prints the practices. Not needed: the session start already prints them
  (plugin) and `scripts/ideal-agent practices` is the same table. Move: the briefs say "the practices at the session start".
- `lab-agent inbox` at session start (plugin src/main.rs:198): reads the ntfy inbox topic. Empty for 12 h and in
  every session start measured; whether the author still writes to the lab from the ntfy app is NOT KNOWN: his word.
- `lab-agent runs`: prose only. Reword.
- job / submit / wait / logs: the job route; used today only by walleye-builder (kernel build, busybox, images, sleep test)
  through `oco-agent submit/run/wait/log`. See 4.
- board and notify: devlab watch's alert to the phone. Needed in some form (an unattended unit has no session to tell).
  Replacement: `curl` to the ntfy topic from a mode-600 file, a backlog row for the standing alert (decision part 4).
- token: moved (step 1, done), the daemon reads the same file through the symlink.
- oco timers and self-update: ocoeco legacy; step 2.

## 3. The plan, where it is, why it stalled
Decision 2026-11-17-lab-agent-retires-by-parts: 1 token, 2 oco timers, 3 jobs, 4 board and notify, 5 inbox and guide,
6 root-exec, then daemon and package. Step 1 landed 2026-11-17 (lab-machine PR 29, ideal-agent PR 45); the other
five items are open and each waits on a word of the author (2: are ocoeco's lab lenses unwanted; 3: keep-awake during
builds; 4: ntfy use and the board's 40 stale items; 5: does he write through ntfy; 6: the root grant shape). Step 1 was
the only part needing no decision, so it was the only one started: the plan is not stuck on work, it is stuck on
five questions nobody has put to the author in one list. The same day the route kept costing: 15 hours of silent 401s
(a daemon holding a token from before a rotation); fixed as lab-agent 0.1.54 (rereads the token file, loud after six
refused polls) and a patrol check of the route's 401 count.

## 4. Should the job route exist at all
Reasons it was made (2026-11-04): a claude.ai sandbox that could not reach the lab pushed scripts to a git repo and the
lab polled. Since 2026-11-15 no such driver exists; sessions run on the lab (Claude Code, same shell, same clones). Measured
today: from a session on the lab, `systemd-run --user --unit=research-probe-1 --collect --wait sh -c 'echo probe'` ran as
mll, logged to the journal (`journalctl --user -u research-probe-1`), and `loginctl show-user mll` says Linger=yes: a
unit started from a session outlives it and a restart of anything. That is everything the walleye builds need from the
route: outlive the session, a name, standing (`systemctl --user status`), stop (`systemctl --user stop`), a log (journal).
What the route has that this lacks: submission from a place that cannot run a command on the lab (none exists today),
and the 5-second polling of the jobs repo (17,280 requests a day; unchanged listings answer 304 and are free of the budget; oco-agent's observe timer
adds about 4,200 requests a day). Answer: no, retire it; the sandbox limit "a session may not hold a long process" is met by a transient
unit. Not known yet: whether a sandboxed (restricted-permission) session can reach the user bus; this session could.

## 5. What replaces root-exec (the author's decision; not changed here)
Options, same power or narrower:
A. A root-owned wrapper `lab-root` (lab-machine) with one sudoers line to it: today's power, renamed, no lab-agent. Cheapest;
   removes the dependency but not the width of the grant.
B. One grant per fixed command (fastboot, ufw status, pacman -S, nix, dmesg, cat /proc/PID/stack) at fixed paths without
   wildcards. devlab already has a fixed command list (lab-tools README). Narrowest; more lines to keep; ad hoc session
   needs (a new pacman package) then ask the author, which is today's "left to the author's grant" (lab-agent-6).
C. polkit rules for unit management only, plus A or B for the rest.
Recommendation: A first (a mechanical rename, proven by devlab flash dry-run and provision), B later, one command at a time,
as each stops being ad hoc. The root prefix is already one constant (`LPOCO_ROOT`, then `DEVLAB_ROOT`, then the default),
so switching is one pin change, not a sweep of docs.

## 6. How others do it (read 2026-11-18; the LAVA worker source and buildbot's failed-auth behaviour were not read)
- GitHub Actions self-hosted runner: dials out over HTTPS; registration tokens last one hour; `--ephemeral` and just-in-time
  config (`generate-jitconfig`) so no standing credential sits on the box; a persistent runner that misses updates 63 days stops
  getting jobs. Take: short-lived credentials and an auth failure that stops the runner at registration, not silently mid-run.
- buildbot worker: one long-lived outbound connection, name and password in buildbot.tac, reconnect with capped back-off (300 s).
  Rotation is an edit on both sides. Take: nothing for loudness (not known yet what a refused password looks like).
- LAVA lava-worker/dispatcher: `--url --token` in /etc/default/lava-worker; the server marks a worker Online/Offline from pings
  and a device Idle/Reserved/Running; a failed health check takes the device offline. Take: a bad state is a visible status of
  the device, set by the thing that sees it (the patrol's route check and devlab's watch are that here).
- systemd-run --user / timers: a transient unit is not a child of the caller; with linger it survives logout. Take: it is the long-job route.
- sudo-rs / polkit: a fixed path with no wildcards is the safe grant; polkit fits "manage this one unit". Take: option B/C above.
None of them has a session on the machine itself; all exist because the controller is remote. Here it is not.

## 7. The 401 fix: worth it, and not more of it
The reread (lab-agent 0.1.54) was cheap (the client re-stats one file), stops the failure that cost 15 hours, and is the loud-failure
pattern the others use. It does not change the order: retire the route; do not extend it (no board state, no new ntfy plumbing).
Order change: step 3 (jobs) moves ahead of 2, 4, 5 and 6 because it needs one word (keep-awake) and probe above shows
the replacement works; retiring it also stops two units (lab-agent, oco-agent).
