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
3. **Up to the author**, through the top coordinator, go only:
   - what needs his hands or his witness;
   - a write to a resource that cannot be undone or risks its recovery (a partition flashed with no known way
     back, an unlock, efuses, erasing what cannot be restored);
   - acting on the world in his name: sending a message or mail, posting, publishing, a call, a pull request or
     patch to someone else's project, an account made, terms accepted (a licence to download a blob), anything bought or paid;
   - his settings, and the agents' permissions, credentials and keys.
4. **The lane's own, asking no one** - recorded either way: everything technical that is not on that list.
   Reading from the world is not acting on it: a public file from a project's own release site (an official
   image, a vendor's published firmware), its checksum or signature checked against what the site publishes,
   kept in the work's own folder with its source and checksum recorded, and used without being run on the lab
   host (a device may boot it from memory) needs no one's word. A file without a published checksum or
   signature, from a mirror, or a program to run on the host is the lane coordinator's decision - research
   first if needed - not the author's.

## Starting the lane's sessions

A lane starts its own sessions without anyone's click (decision 2026-11-17-a-lane-starts-its-own).

- **Launchers**: one stored, on-demand task per role of the lane - its keeper, its research lead, its
  builder - set up once by the author, so that it runs with his permission mode. Its prompt never changes:
  the role's head (`Role:`, `Name:`, `Also practices:`), the brief's path, and where its work is read - the
  top of the lane's queue, or the lane's open question in its base. It says plainly the writes it may make
  (land, records, the resource through its tool), since a launched run is told to make no other.
- **Where a launcher reads its work**: one named query, the lane's next item of a kind - each item names the
  device it is for and what it needs (the resource, research, a build); linux-phone: `tad q lane-next
  --device D --needs phone|research|build`.
- **The lane coordinator** writes the work into the base first - the queue item, or the question - and then
  runs the launcher, on any turn. A launcher runs once at a time, so a lane never has two keepers.
- **A run's end wakes the lane**: a launched session is not its coordinator's child, so its end says nothing by
  itself (surya's lane sat idle from 18:16 on 2026-11-17). At the lane's start - and after its own renewal -
  the lane coordinator subscribes itself to each launcher's completion (one subscriber per launcher, the newest
  replacing the last) and re-runs on the wake; and a launched session's last act, after its final landing, is
  two lines to its lane coordinator, so the lane wakes even where a subscription was lost.
- **Within its own turn** the lane coordinator still uses subagents for a short question (one round, a
  lighter model) or a read-only look; a launcher is for work that changes something or takes long.
- How a launcher is made and run in your harness: ideal-agent's base, `tad q about --subject claude-code`.
