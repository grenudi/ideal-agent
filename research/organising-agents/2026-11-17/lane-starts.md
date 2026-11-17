# Who starts a lane's sessions (2026-11-17)

## The pauses today (lane coordinators' transcripts 97ce3361 surya, 6b97b7cb walleye)
| card | lane | what | first sign it had started | waited (upper bound) |
|---|---|---|---|---|
| 13:08 | surya | keeper: SIM skips | 13:48 | ~40 min |
| 15:21 | walleye | three sessions refused at once | 15:52 (first research) | ~30 min |
| 15:53 | walleye | builder v7.3 | started by the top coordinator | ~ |
| 15:58 | surya | keeper: cluster sleep | 16:28 | ~30 min |
| 16:29 | surya | keeper FastRPC + research floor | 16:58 (research) | ~29 min |
| 16:58 | surya | keeper: subtract the floor holders | running at 17:02 | ~4 min |
Every one is start_session refused on a turn a message or event began - a lane coordinator wakes on its
keepers' reports, so nearly every turn of its is such a turn - and a card waiting for the author's click.

## What the app allows (checked 2026-11-17)
- start_session: refused on a message's or event's turn ("start sessions only on the user's own words").
- run_scheduled_task: started a new session from a message's turn (probe, 17:01 UTC); refused while the task
  has a run in progress; the run keeps the permission mode saved with the task, which is saved only when the
  task is created on a turn the author typed; its prompt is wrapped in <scheduled-task ...> with a line that
  the user is absent and to make only the writes the task asks for.
- SendMessage to an idle session: works on any turn (a standing keeper could take work by message).
- clear_session: the app asks the author to approve each call outside auto or bypass mode.
- Agent tool subagents: within the turn, any turn.
- A headless claude: not logged in on the lab; a run outside the app is not seen, steered or archived.

## Launcher prompts for the author's one-time setup (no schedule; created on his own turn)
surya-keeper:
  Role: keeper (ideal-agent) / Name: surya-keeper / Also practices: 42
  Your brief: ~/Projects/linux-phone-oco/docs/ideal-agent-linux-phone-oco/phone-keeper.md (its first
  paragraph). Take the top of `tad q phone-next` in docs; work it through devlab; record and land in docs;
  report to the surya coordinator. You may: change the phone through devlab, land in linux-phone places,
  message the surya coordinator.
surya-research:
  Role: research-lead (ideal-agent) / Name: surya-research / Also practices: 42
  Your brief: .../feature-research.md. Your question is the surya lane's open question in docs (`tad q next
  --about surya-question`); answer it as research and a decision, land in docs, report to the surya
  coordinator. You may: land in docs, message the surya coordinator. You never change the phone.
walleye-keeper, walleye-research, walleye-builder: the same, with walleye's brief, queue and question.
