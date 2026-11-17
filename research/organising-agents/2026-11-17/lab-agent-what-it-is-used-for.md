# lab-agent: what it is used for today, what depends on it, what supersedes each part (2026-11-17)

Measured on the lab (grig) 2026-11-17 ~17:30-18:20 CET, nothing changed. A first look by orchestration
(side-session-notes lab-agent-what-it-is-2026-11-17.md) was the lead; every claim below was re-checked.

## What it is
creation-guidelines/lab-agent (Rust, 0.1.53; /usr/bin is 0.1.52, ~/.local/bin 0.1.53 is what runs - a drop-in
`use-the-fixed-build.conf` points the unit there). Begun 2026-11-04 so a claude.ai sandbox that could not reach
the lab could push numbered shell scripts to grenudi/lab-agent-jobs; the lab polls (every 5 s, GitHub API), runs
each, pushes results to grenudi/lab-agent-results. Around that grew: root-exec, notify/notify-setup, board,
inbox, ask, guide, self-update, connect, monitor, submit/wait/logs/jobs. The same repo holds oco-agent
(workspace dir oco-agent/), the redesign after the 2026-11-11 stall (decision 2026-11-12-oco-agent).

## 1. Use today (measured)
Jobs, lab-agent: 1165 job files ever added to the jobs repo (git history). By the day in the file name's epoch:
11-09 184, 11-10 526, 11-11 293, 11-12 81, 11-15 2 (two probes, "probe-the-way-in", "a-driver-speaks-with-git-alone"),
11-13/14/16/17 none. Last real job 2026-11-12 04:09 (the-switch-1-proven). The daemon is idle since: its journal
since 11-16 is only DNS failures reading board and inbox, two self-switches (10:58, 13:03 today) and a
"1137 jobs already have results" line. Real work (phones, builds, landing) runs in Claude sessions on the lab.
Jobs, oco-agent (same jobs repo, oco-agent/jobs): 956 files 11-12..11-16 (133, 135, 188, 203, 305 per day), 228 of
them "window" (`. oco-work; said ...` - ocoeco speaking), 62 of the 400 newest carry `by=claude`; last 11-16 23:50.
That traffic is ocoeco's own. TODAY, one non-ocoeco use: walleye-builder (session 04b24088) submitted
walleye-kernel-build-2..4, walleye-busybox-1, walleye-images-1, walleye-sleep-test through `oco-agent submit/run/wait/log`;
`oco-agent standing`: kernel-build-1 failed, -2 and -3 "parked: the agent restarted while doing it", -4, busybox, images,
sleep-test finished. That is the only job runner use measured today, and it is a long build that must outlive a session.
root-exec: used today by three sessions (220b014f fastboot devices; 7007ce97 dmesg, /proc/PID/stack, `pacman -S`
perl-io-socket-ssl; 8b91886a only my own greps) and by lab-machine setup (nix.sh, locale.sh) and devlab. In use.
board: board.json has 42 tasks; 40 last touched 2026-11-10/11 (surya-audio, oco, ocoeco, "rust-toolset": stale). Live: `sim`
(touched 11-16 23:52 by a session) and health-xiaomi-surya (set/done by `devlab watch` 11-17 09:35/14:50). Each change
is pushed to the author's phone on the ntfy info topic: 10 pushes today (cache window ~12 h), titled "lab-agent:
what I'm doing", body "You received a file: attachment.txt".
inbox: runs at every session start (the plugin hook; 33 times in today's transcripts, always empty at the start of this one).
The ntfy inbox topic holds nothing in its cache window (~12 h); older traffic is NOT KNOWN. 4 `inbox --ack` today.
guide: 33 calls today = the session-start hook's neighbours, plus the plugin's own start prints practices; the plugin's
`scripts/ideal-agent practices` is the same table. notify: one `notify --urgent` today (220b014f). ask: none today. self-update:
lab-agent-update.timer daily (last 11-17 00:58). Shadow timer (10 min), oco-ask-to-observe (15 min, ~44 GitHub requests each)
and oco-say-what-is-new (5 min): ocoeco-era timers, all running.
Units active: lab-agent.service (inhibits idle), oco-agent.service (`run --listen grenudi/lab-agent-jobs`, inhibits sleep),
devlab-watch@surya-lab, 4 oco timers + lab-agent-update.timer.

## 2. Dependents (grep of ~/Projects, ~/.local/bin, the plugin, units)
- Token file ~/.config/lab-agent/token: lab-machine/bin/land:31; ideal-agent scripts/agent-env:19 (GH_TOKEN for every session);
  lab-machine/systemd tad-update-pins.service:10, idealkb-update-pins.service:12; oco-agent command-line/src/main.rs:31-40
  (falls back to it). ~/.config/lab-agent/env holds a second token (LAB_AGENT_GITHUB_TOKEN) for the daemon. Only the FILE.
- root-exec: sudoers /etc/sudoers.d/lab-agent (written by `lab-agent connect`, connect.rs:127-): `mll ALL=(root) NOPASSWD:
  /usr/bin/lab-agent` (`sudo -n -l` confirms; /usr/bin/lab-agent is root-owned; ~/.local/bin copy is NOT granted). root_exec
  (main.rs:119) runs ANY argv as root: root for anything, by design. Callers: devlab flash.rs:12, provision.rs:57,95
  (DEVLAB_ROOT overrides), guard.rs:179 (a rule's text); lab-machine setup/nix.sh:15, locale.sh:6; sessions ad hoc.
- board: devlab watch.rs:22-24 (LAB_AGENT overrides; unit devlab-watch@.service sets LAB_AGENT=/usr/bin/lab-agent).
- inbox/guide: plugin src/main.rs:198 runs `lab-agent inbox` at session start; skill lab-session-start names it; practice 04
  says "lab-agent inbox"; practices.rs:92 and tables.toml:18 name `lab-agent guide`. (Practice 06 now says "through a lab job", not oco-agent.)
- self-update: lab-machine/systemd/lab-agent-update.{service,timer}.
- units: lab-agent.service, stay-awake.conf, use-the-fixed-build.conf, oco-agent.service (+ shadow, observe, say timers).
- oco-agent: walleye-builder's jobs (today); marshall-oco/tad-engine oco/ tables and ocoeco (`. oco-work`) - legacy.
- Not found: any cloud/sandbox driver submitting after 11-15; any caller of `ask`, `submit`, `wait` outside the jobs themselves.

## 3. Parts, what supersedes, what would be lost, cost
See the decision 2026-11-17-lab-agent-retires-by-parts for the table and the order.

## Sources
repo creation-guidelines/lab-agent (src/main.rs, connect.rs, notify.rs, oco-agent/README.md, oco/decisions.json),
grenudi/lab-agent-jobs (git history, board.json, oco-agent/jobs), lab journal (journalctl --user -u lab-agent),
`sudo -n -l`, `systemctl --user list-units/list-timers`, ntfy topic caches, ~/.claude/projects transcripts of 2026-11-17.
