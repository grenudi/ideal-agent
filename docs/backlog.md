<!-- Generated from data/ by tad. Do not edit by hand. -->

# Backlog

<a id="release-binaries"></a>
### Prebuilt ideal-agent binaries, so the hooks hold on a machine without cargo

Status
: open

Note
: The plugin builds ideal-agent from its source with cargo at the first session start (scripts/ideal-agent); a machine or cloud session without Rust gets no guard and is told so. Next: a release workflow (release-plz and musl builds, as tad-engine has) and scripts/ideal-agent fetching the pinned release with its checksum before falling back to cargo.

About
: hooks

Opened
: 2026-11-17


<a id="guard-stop-unlanded"></a>
### A Stop hook: a turn does not end with unlanded changes in a place the session worked in

Status
: open

Note
: Was lab-tools' backlog guard-stop-unlanded (devlab guard); general to every project, so it belongs to the plugin's hooks. Next: decide what counts as a place the session worked in (the cwd's repository and the worktrees it made) and prove it against its own event.

About
: hooks

Opened
: 2026-11-17


<a id="session-start-size"></a>
### The session start says less: its output is about 20 KB

Status
: done

Note
: Done 2026-11-17: the session start prints each practice's words without its reason (22.5 KB to 13.9 KB, measured on main with scripts/start-weight); the reasons are one command away (lab-agent guide, scripts/ideal-agent practices). Kept by practice 45-keep-the-start-light.

About
: housekeeping

Opened
: 2026-11-17


<a id="devlab-reads-commands-here"></a>
### devlab's guard reads shell commands with ideal-agent's reader instead of its own copy

Status
: done

Note
: Done 2026-11-17 by orchestration: devlab's guard uses ideal_agent::shell::{simple_commands, unwrapped}, pinned in devlab/Cargo.toml at rev 5967ea8 (read back from lab-tools' main); its own copy is gone. A quoted here-document naming the phone's address passes, a real ssh there is refused. When the reader changes here, devlab takes it by bumping the rev.

About
: hooks

Opened
: 2026-11-17


<a id="move-in"></a>
### Move the general agent know-how here and retire the old places with a pointer

Status
: done

Note
: Done 2026-11-17, each read back from main: ideal-agent #1 and #2; lab-tools #65 (devlab guard keeps only the phone's rule; installed; the lab's settings keep only its PreToolUse hook); lab-agent #218 (guide reads the practices here, built-in copy refreshed, oco-agent/skills retired with a pointer; installed); docs #159 (briefs keep their domain and name the skills; decisions for lab-tools and docs); marshall-oco #32 and ecosystem #159 gather ideal-agent; ocoeco #456 and ecosystem #160 repoint the links to the practices; the observatory's design/data/practices.json retired with a pointer. The plugin is installed on the lab (user scope) and proven live in a fresh session: practices and inbox at start, push to main and pip refused by ideal-agent guard, a branch push and grep python allowed, ssh to the phone refused by devlab guard, a quoted here-document allowed, all eight skills and the researcher listed. The old ~/.claude/skills links are in ~/.claude/backups/skills-links-before-ideal-agent.

About
: ideal-agent

Opened
: 2026-11-17


<a id="follow-ideal-knowledge-base"></a>
### ideal-agent follows grenudi/ideal-knowledge-base, as its second follower

Status
: done

Note
: Done 2026-11-17: follows idealkb 0.1.0 (idealkb.toml), about on every record, subjects table with references rules (decision 2026-11-17-follows-ideal-knowledge-base).

About
: knowledge-base

Opened
: 2026-11-17


<a id="sources-by-back"></a>
### Move the sources' 'Made by ...' back into by, once idealkb v0.3.0 is pinned

Status
: done

Note
: Done 2026-11-17: idealkb 0.3.0 is pinned and declares by; the three 'Made by ...' prefixes are back in by (lava-scheduler, ci-tron-introduction, pmos-hw-ci-status).

Waits
: the idealkb pin moving to 0.3.0

About
: knowledge-base

Opened
: 2026-11-17


<a id="practices-short-at-the-start"></a>
### Proposed to the author: the session start shows each practice's first sentence only

Status
: done

Note
: Done 2026-11-17 at the author's word: the session start prints first sentences (decision 2026-11-17-the-start-reads-first-sentences).

Waits
: the author

About
: housekeeping

Opened
: 2026-11-17


<a id="measure-start-without-connectors"></a>
### Measure the next keeper's first turn without Gmail and Claude Docs

Status
: open

Note
: Before: surya-keeper-2 (started by hand) first turn 81,280 tokens, 33.5 KB read before acting. A first 'after' (waker run 45833ff5, 12:33 UTC): first turn 75,649, 39.1 KB read before acting (devlab start 10.8 KB, the phone queue asked 5 times ~13 KB, a tad sql 5.1 KB), first act 26 min in (a permission prompt). Not yet comparable: that run began before Gmail and Claude Docs were turned off, and the scheduled runs measured earlier started at 73.5-75.6k too - a scheduled run's system part is smaller than a hand-started session's. Next: a keeper started after the connector change, by the same route as its before, measured with read-weight.

About
: housekeeping

Opened
: 2026-11-17


<a id="measure-a-role-start"></a>
### Measure a role-named session's first turn with read-weight on plugin 0.3.0

Status
: in-progress

Note
: keeper-6 had no Role line (every practice at the first prompt): 80,864 tokens, as before. The clean after is surya-keeper-7, started with Role: keeper; orchestration measures it.

Waits
: plugin 0.3.0 installed on the lab and a session started with a Role line

About
: housekeeping

Opened
: 2026-11-17


<a id="briefs-name-their-role"></a>
### linux-phone's briefs name their role and drop what the card says

Status
: done

Note
: docs #230: every brief names its role, Name and Also practices; trimming is orchestration's.

Waits
: orchestration's word

About
: organising-agents

Opened
: 2026-11-17


<a id="briefs-drop-oco-work"></a>
### linux-phone's briefs drop . oco-work and OCOECO_BY for Role and Name lines

Status
: done

Note
: docs #230: no brief says . oco-work, OCOECO_BY or landit.

Waits
: orchestration's word

About
: organising-agents

Opened
: 2026-11-17


<a id="skills-listed-per-role"></a>
### Which skills and plugins a role's session lists at its start

Status
: done

Note
: The author chose skillOverrides name-only for the 33 unused skills (research claude-code-skill-overrides); the after is backlog measure-skill-overrides.

Waits
: the author's word on his settings

About
: housekeeping

Opened
: 2026-11-17


<a id="lab-agent-1-token-neutral-path"></a>
### lab-agent retires 1: the token to a neutral path

Status
: done

Note
: Done 2026-11-17: token at ~/.config/ideal-agent/token (600), old path a symlink to it; land, agent-env and the 2 pins units read the new path (lab-machine PR 29, this change). oco-agent and oco-* tools stay on the symlink until step 2. lab-agent-free count 34 -> 28 (the rest are other steps: src, skills, units, setup, devlab).

About
: organising-agents

Opened
: 2026-11-17


<a id="lab-agent-2-ocoeco-timers-stop"></a>
### lab-agent retires 2: stop the oco timers

Status
: done

Note
: Done 2026-11-18 by the author's word (turn-2026-11-18-lab-agent-timers-and-keep-awake): oco-agent-shadow, oco-say-what-is-new and oco-ask-to-observe timers disabled and stopped (systemctl --user disable --now; the unit files stay). list-timers shows no oco-* timer. lab-agent-update.timer goes with the daemon (step 7).

About
: organising-agents

Opened
: 2026-11-17


<a id="lab-agent-5-inbox-guide-out-of-the-start"></a>
### lab-agent retires 5: the session start without lab-agent inbox

Status
: done

Note
: Done 2026-11-18 (author: the ntfy inbox is not carried anymore): the plugin's session start no longer runs lab-agent inbox, its practices line and the skill name scripts/ideal-agent practices, practice 04 says messages arrive in the conversation. The linux-phone-oco briefs' 'lab-agent guide' words are lab-agent-7. lab-agent-free 28 -> 22.

About
: organising-agents

Opened
: 2026-11-17


<a id="lab-agent-6-root-exec-grant"></a>
### lab-agent retires 6: root for devlab and setup without lab-agent

Status
: in-progress

Note
: Grant revoked 2026-11-18 ~14:49 by the author (lpoco-host revoke now): /etc/sudoers.d/lab-agent is gone and sudo -n lab-agent root-exec asks for a password (checked). No standing root: lpoco-host prepare now runs each root command behind sudo with his password. The guard no longer unwraps lab-agent root-exec (ideal-agent 0.3.43). Waits only on the author's rerun of lpoco-host prepare now for the per-device helper (lpoco 0.14.2).

About
: organising-agents

Opened
: 2026-11-17


<a id="lab-agent-4-board-notify-to-curl-and-backlog"></a>
### lab-agent retires 4: devlab watch to curl and a backlog row

Status
: done

Note
: Done 2026-11-18, by the author's word (turn-2026-11-18-lab-agent-board-alerts-not-needed: not needed, nothing replaces it, the concept is not carried): devlab watch files no board task and sends no alert, its verdict is a journal line (lab-tools PR 152, merged; in use after the next lpoco release and prepare). The board's 40 stale items are left to die with the daemon. Replaces the curl-and-backlog-row plan.

About
: organising-agents

Opened
: 2026-11-17


<a id="measure-skill-overrides"></a>
### Measure a fresh session's skill listing after the name-only overrides

Status
: open

Note
: read-weight on the first session started after 2026-11-17 17:00 UTC: skills listed, bytes, first-turn tokens; compare with 44 skills, 22,455 B, ~80k tokens. Then supersede claude-code-skill-overrides with the number.

About
: claude-code

Opened
: 2026-11-17


<a id="canon-one-source"></a>
### The author's canons as one source: software-engineering-canon, the playbook, design-canon

Status
: open

Note
: About ten rules repeat across software-engineering-canon, ideal-project-playbook and design-canon (likely an earlier version); ideal-agent practices 22, 23, 43 repeat canon ids too. Next: the author's word on retiring design-canon; link overlaps by id rather than restate (upstream items to the canon's owner).

Waits
: the author's word on design-canon

About
: practices

Opened
: 2026-11-17


<a id="ideal-agent-follows-idealkb"></a>
### ideal-agent's base follows idealkb past 0.5.0

Status
: done

Note
: Followed 0.5.0 -> 0.14.0 (2026-11-18); the class lines are in session log 2026-11-18-ideal-agent-follows-idealkb-0-14.

About
: knowledge-base

Opened
: 2026-11-18


<a id="lab-agent-0-five-words-from-the-author"></a>
### lab-agent retires 0: the five words only the author can give, asked once

Status
: done

Note
: Closed 2026-11-18: all five answered. 2 (oco timers): unwanted, stopped. 3 (keep-awake): only the build's own unit holds the lab awake, designed from lpoco's one roof. 4: board and phone alerts not carried. 5: the ntfy inbox not carried. 6: the root grant viewed from lpoco's one roof (research 2026-11-18-lpoco-root-from-one-roof), direction waits on his word. His words: marshall-oco/conversation turns turn-2026-11-18-lab-agent-*.

Waits
: the author

About
: organising-agents

Opened
: 2026-11-18


<a id="lab-agent-7-briefs-name-no-lab-agent"></a>
### lab-agent retires 7: linux-phone-oco briefs and docs stop naming lab-agent guide and runs

Status
: open

Note
: Proof: in linux-phone-oco main checkouts 'lab-agent guide' and 'lab-agent runs' find nothing outside dated records; the root prefix constant is the only live string left until lab-agent-6.

Blocked by
: lab-agent-5-inbox-guide-out-of-the-start

About
: organising-agents

Opened
: 2026-11-18


<a id="landed-by-trailer"></a>
### land keeps who landed: a Landed-by: trailer

Status
: done

Note
: Not needed: GitHub's squash keeps the run's name as Co-authored-by; who-lands reads it (decision 2026-11-18-a-run-signs-as-role-and-run).

Waits
: land's owner

About
: organising-agents

Opened
: 2026-11-18


<a id="land-refuses-a-dirty-rebase"></a>
### land's rebase refuses while the clone holds uncommitted work

Status
: open

Note
: From the class question 2026-11-18-uncommitted-work-reset-away: as agent-env's branch does. land is lab-machine's (lab-agent-token-path keeps it).

Waits
: land's owner

About
: organising-agents

Opened
: 2026-11-18


<a id="canon-attention-principle"></a>
### software-engineering-canon: a principle for converging tools that ask only on a change

Status
: done

Note
: Landed 2026-11-18 in software-engineering-canon #36: principle converge-and-ask-only-on-change, three sources opened; the canon no longer exports ocoeco glimpses (which made land ask a retired route). write-code-by-the-canon names the id.

Waits
: the author, who keeps the canon

About
: practices

Opened
: 2026-11-18


<a id="shell-past-glue-into-rust"></a>
### Move ideal-agent's scripts that are past glue into the Rust program

Status
: done

Note
: Closed without doing: dropped on the author's word (decision 2026-11-19-existing-shell-stays) - existing shell stays as it is unless a real need comes.

About
: practices

Opened
: 2026-11-19

Rank
: 20


<a id="role-start-brief-whole"></a>
### The role start hands the brief over whole from origin/main: a per-session file with its commit, the start's output under the app's cap

Status
: done

Note
: Done in 0.3.63: the brief is written to ~/.local/state/ideal-agent/sessions/SESSION.brief.md, headed by its path and main's commit; the start prints a pointer and the first paragraph past the title and head. Measured with linux-phone-oco's briefs: builder 5.7 KB, keeper 6.7, coordinator 8.3, patrol 7.4, orchestrator 7.5 (cap 10,000); each file equals origin/main's. From linux-phone-oco/docs upstream item role-start-brief-whole.

About
: hooks

Opened
: 2026-11-19

Proof
: brief test: a 20 KB brief prints under 2 KB and the file holds it whole; live: five roles' starts under the cap, the file equal to origin/main


<a id="guard-refuses-stale-clone-reads"></a>
### The guard refuses a command in, or a read from, a shared clone's main checkout behind its local origin/main

Status
: done

Note
: Done in 0.3.63: PreToolUse matches Bash|Read|Grep|Glob; in a shared clone (shared_main) whose HEAD..origin/main counts more than 0, a command other than git, branch and a few that only look is refused, and so is a path there named in a command or read by Read/Grep/Glob; the refusal names the worktree line, git show and git pull --ff-only; SHARED_CLONE_OK=1, in the command or the environment, lets a deliberate one through. 17 clones under ~/Projects were behind at landing (lab-agent 213, linux-phone-oco/docs 25, surya-lab 23). From linux-phone-oco/docs upstream item guard-refuses-stale-clone-reads.

About
: hooks

Opened
: 2026-11-19

Proof
: guard test reading_in_a_stale_shared_clone_is_refused_and_main_is_not; live against a real clone one behind: cat, Read, Grep and tad -C refused (rc 2), git pull, a worktree Read, SHARED_CLONE_OK=1 and the current clone allowed


<a id="stale-worktrees-pruned"></a>
### Agent worktrees left behind are pruned: a clean one nothing wrote to for 3 days is removed, a dirty one named

Status
: done

Note
: Done in 0.3.64: ideal-agent prune now|show (src/prune.rs), started once a day by the session start in its own process group, its report in ~/.local/state/ideal-agent/pruned.log. Worktrees: every linked worktree of the clones under ~/Projects, locked ones never; git worktree remove keeps the branch. Build runs moved here from agent-env, whose loop used find -newermt '-3 days': in a Claude Bash shell find is the app's bfs wrapper, which rejects that date, so every other run folder was removed whatever its age (one left on 2026-11-19); now find -mtime -3, from Rust. At landing: 138 worktrees, 70 untouched for a day, 16 for two, none for three. From linux-phone-oco/docs upstream item stale-worktrees-pruned.

About
: housekeeping

Opened
: 2026-11-19

Proof
: prune test an_old_clean_worktree_is_removed_and_a_dirty_one_kept (live git: show removes nothing, now removes the clean old one, keeps the dirty and the fresh, the branch stays); prune show on the lab machine: nothing 3 days old yet


<a id="lab-agent-3-jobs-by-systemd-run"></a>
### lab-agent retires 3: long jobs by systemd-run --user

Status
: done

Note
: Done 2026-11-20 (lab-agent-token-path's session): lab-agent.service, oco-agent.service, lab-agent-update.timer and the four oco timers are off and their unit files removed (lab-machine PR 83); long jobs run as lpoco-lab job. Read on the lab machine: no lab-agent or oco unit in systemctl --user, no lab-agent or oco-agent process; scripts/lab-agent-free: the job route has 0 parts up. Left of the plan: step 7, the daemon and package (the author removes /usr/bin/lab-agent as root; then ~/.local/bin/lab-agent, ~/.config/lab-agent and the two GitHub repos). Still named by what the lab runs, read from origin/main: lab-machine bin/land (a fallback author name lab-agent-session) and setup/ (two comments), lpoco host's grant revoke (names it by purpose).

About
: organising-agents

Opened
: 2026-11-19

Proof
: walleye-c3 and walleye-c4 built through lpoco-lab job on 2026-11-18; systemctl --user list-units and pgrep find nothing of lab-agent or oco-agent on 2026-11-20
