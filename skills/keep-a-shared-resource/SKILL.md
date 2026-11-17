---
name: keep-a-shared-resource
description: The keeper's method: the one session that changes a shared resource (a device, a lab machine, a queue), serving a queue others prepare. Use as a keeper, when you need such a resource changed, or setting a keeper up.
---

# Keeping a shared resource

One owner per shared resource (practice 44-one-keeper-per-shared-resource; why: research
keeper-of-a-shared-resource - LAVA's scheduler and CI-tron's gateway for test boards, Gas Town's Refinery for
the merge queue, the actor model's single writer). The keeper is a **lasting role, not an immortal session**:
the expertise lives in the resource's records (decision 2026-11-17-experts-written-not-standing).

## What a project supplies

Your brief names them; this skill is the rest. A project keeps only these:

- **the resource** and **its tool**, which holds the lock for every change (linux-phone: the phone, devlab).
  The keeper adds flow, not safety: never wrap the tool in a lock of your own.
- **the queue**: a named query over the project's backlog - the items marked as needing the resource - shown
  on the project's face (`tad q NAME` in a tad place). Not a list of its own.
- **the lease**: what says a keeper is live, so two never run.
- **the waker**: a scheduled run, or the patrol, that starts the next keeper session whenever the queue has
  work and no keeper is live.
- **the how-it-stands record** of the resource (a research row, RESOURCE-how-it-stands).

## If you need the resource and are not its keeper

Prepare, do not touch: research, write the patch, the test and its proof in parallel. Then put an item on
the resource's queue - a backlog item marked as needing it, saying what to do, how it is proven (which test,
present or works), and what it risks. The keeper takes it; you are told through the record.

## If you are the keeper

1. **Take the lease** (the project's way); if a keeper is live, end - there is one.
2. **Read the record, in summary**: the resource's how-it-stands (`tad q about --subject RESOURCE`, its
   summary first), its health, and the top of the queue (its named query). Nothing else until a step needs it.
3. **Take the next item by the weakest link** - the item a milestone rests on, whose proof is clearest - not
   by arrival. Say on the item that you took it.
4. **Execute it through the resource's tool**, one change at a time, keeping the resource known-working:
   tries fall back by themselves and end in its health report (change-a-device for a device).
5. **Record as you go**: the item's result and proof, its status (done only when read back), what was
   learned in the how-it-stands row - item by item, never rewritten whole.
6. **End** when your piece lands or your context grows large: write the hand-over into the how-it-stands row
   (where it stands, what is next on the queue, dead ends, what never to do again), release the lease, and
   end the session. The waker starts the next keeper fresh from the record and the queue.

A question about the resource is answered from its record first, then by the live keeper directly, and the
answer is written back as a row.

## When a keeper cannot decide

One decider per lane: its coordinator, who holds the records and the lane's knowledge (decision
2026-11-17-decisions-go-to-the-lane).

1. **The keeper** leaves the resource known-working - nothing half-done - writes the question as a backlog
   item of the lane (what it saw, the ways it could go, the way back of each), sends it to the lane
   coordinator, and takes the next queue item that waits on nothing. It never waits on the author itself.
2. **The lane coordinator decides** what can be undone - a two-way door - once the records show the way back
   (for a bootloader slot: the other slot still boots and fastboot is reachable). Where the records do not
   say, it asks a read-only researcher subagent on a lighter model, briefed with the question, the answer's
   form and a bound (one round; a second only for what the first left open); a question too long for that
   is a research-lead session, started on its own turn or proposed. Then it records the research and the
   decision in the lane's base - before anyone acts on it - tells the keeper what to do with the decision's
   id, and sends the top coordinator two lines with the id. What the research cannot settle goes up.
3. **Up to the author**, through the top coordinator, go only: what needs his hands or his witness; a write
   that cannot be undone or that risks the resource's recovery (a partition flashed with no known way back,
   an unlock, efuses, erasing what cannot be restored); money; anything outward (publishing, messages,
   accounts); and changes to permissions, security or an agent's own settings. The rest is the lane's,
   recorded either way.
