# Attended steps: how others spend a person's presence (2026-11-19)

Asked by the author through the Top Coordinator, after a session at the phones. A walleye keeper got its
arguments wrong while he waited. A surya go reached a scheduled keeper that wasn't the one attending, because
the attended keeper had been archived mid-session. His rules (2026-11-19):

1. "please, ask me to attent only when everything is preped 1 launch sequence away with test sequences being
   prepered as rust-script or something and they test them before asking me to attend".
2. "make sure the rust program reads cleanly and simply on the top entry file level, like any other simple
   scripted procedure would read".

The decision resting on this is 2026-11-19-attended-one-command-away.

## What let it happen (our side)

- **Built at his arrival.** The program for the step was written and started while he sat there, not before.
  The keeper was launched when he came.
- **The go went to a keeper, not a run.** The go was open on the item, so whichever keeper was alive took it.
  Nothing tied it to a prepared program.
- **Fixes were typed live.** When a step failed, commands were improvised while he waited. The flow had no
  hold that sends a fix back to the program.

## What others do

- **Hardware-in-the-loop rigs.** Runs are scripted, start from a known state, and pin firmware and wiring as
  one baseline. Sources: [OPAL-RT](https://www.opal-rt.com/blog/what-engineering-teams-need-before-launching-a-motor-drive-hil-program/),
  [DMC](https://www.dmcinfo.com/blog/51793/hardware-in-the-loop-testing-aerospace-risk-reduction/).
- **Launch countdowns.** Holds are built into the countdown, and a go/no-go poll is taken during a hold.
  After the final go, the sequencer runs the terminal count automatically: people may call a hold, but they
  don't improvise steps. Sources: [NASA Artemis II countdown, 2026](https://www.nasa.gov/general/nasa-releases-artemis-ii-moon-mission-launch-countdown/),
  [Artemis Countdown 101](https://ntrs.nasa.gov/api/citations/20220004992/downloads/Artemis%20countdown%20101_aj_CLEAN.docx).
- **Aviation and medical checklists.** Pilots use do-verify for normal procedures and challenge-do-verify for
  abnormal ones ([FAA AC 120-71B, 2017](https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_120-71B.pdf)).
  Gawande's pause points hold 5 to 9 items and take 60 to 90 seconds (The Checklist Manifesto, 2009).
- **Runbooks as code.** The Google SRE book reports roughly a 3x faster recovery with a playbook than with
  "winging it" ([SRE book, introduction](https://sre.google/sre-book/introduction/)). Dan Slimmon's
  "do-nothing scripting" (2019) makes each step a function that prints its instruction and waits; steps are
  then automated one at a time ([blog post](https://blog.danslimmon.com/2019/07/15/do-nothing-scripting-the-key-to-gradual-automation/)).
- **Human-gated pipeline steps.** GitHub's pending-deployment approval carries the run id in its path. A
  Jenkins `input` step has its own id under one build. Spinnaker's Manual Judgment pauses one execution.
  GitLab manual jobs are protected by environment. All of them bind an approval to one run, never to a
  message. Sources: [GitHub REST, workflow runs](https://docs.github.com/en/rest/actions/workflow-runs),
  [Jenkins input step](https://www.jenkins.io/doc/pipeline/steps/pipeline-input-step/),
  [Spinnaker safe deployments](https://spinnaker.io/docs/guides/tutorials/codelabs/safe-deployments/),
  [GitLab job control](https://docs.gitlab.com/ci/jobs/job_control).
- **An entry that reads like a procedure.** Kent Beck's Composed Method: a body made of calls to well-named
  methods at one level of detail ([c2 wiki](https://c2.com/ppr/wiki/WikiPagesAboutRefactoring/ComposedMethod.html)).
  Martin's stepdown rule: each function is followed by the ones a level below it (Clean Code, 2008). matklad's
  cargo xtask: one binary of named subcommands ([2018 post](https://matklad.github.io/2018/01/03/make-your-own-make.html)).

## What we take

1. **Rehearse before asking.** Ask him only after a full dry run, with every safe part run for real, and name
   the one command.
2. **`main` is the checklist.** The program first checks the resource is in its known state, and refuses to
   start if it isn't.
3. **Attended steps are pause points.** Each says what to do or see, then waits. Keep each short.
4. **A go binds to a prepared run.** In our harness, the go on the item names the program, so any keeper runs
   only that command.
5. **A failure is a hold, never a live fix.** The fix is made to the program, without him, and then a new go
   follows.
6. **A runner that dies ends its run.** The same program starts again from its start or from a safe point,
   and he is told first.
7. **Each attended run leaves one step fewer** that needs him (the do-nothing pattern).

## Ours already

- linux-phone-oco lanes: surya-lab #388 and #390 (surya-attended-one-command), google-walleye-lab #411 (one
  program per attended try).
- docs #634 (lpoco-attended): the shared runner for attended sequences, which carries how a run is driven.
  ideal-agent holds the general rule and its place in each role.

## Not known yet

Whether a run id stamped into the go (as GitHub and Jenkins bind theirs) is needed, or whether naming the
program on the item suffices. That is for the shared runner to show.
