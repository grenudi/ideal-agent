---
name: bring-up-a-part
description: The bring-up method: make one part of a device work on mainline, proven by the device's tests - what is, others' work, clean-room, test first, the change, upstream. Use when your brief makes you bring a part up.
---

# Bringing up a part

Your brief names the device, its tool, the places you own and the parts that wait. The first device is not
the mould: write what you learn so the next of its family starts further ahead.

1. **What is.** The part's summary - `tad q about --subject S`: how it stands, then each record by its
   brief. A record whole only where its brief leaves the step in hand open. Say in a sentence what is
   known, what is not, and why it is so.
2. **What others learned.** Inside the commons first, then outside: mainline (lore.kernel.org, the
   subsystem's tree), the SoC's mainline effort and its siblings, the distributions, the vendor's GPL kernel
   (a reference for register sequences, regulators, GPIOs, device-tree values). Record it as research, so
   decisions can cite it.
3. **Reverse engineering, clean room, when nothing open exists.** Observe behaviour, never copy proprietary
   code: register traffic, traces and dynamic debug, the messages between processors, the stock system's own
   logs. Write what you observe as a specification; the driver is written from it. GPL code may be ported
   with its copyright and licence kept; disassembled or decompiled proprietary code never enters a patch, a
   note or a commit. Firmware is never redistributed beyond its licence.
4. **The test first** - a check that fails for the missing part (change-a-device).
5. **The change.** One branch per fix, each commit clean enough to go upstream as it is; userspace
   configuration beside the device's data, each mask or overlay with its reason beside it.
6. **Through the conveyor, never by hand.** Tries fall back by themselves and end in the health report; once
   proven, the change goes where every flash carries it. A step done a second time by hand is a tool to
   build.
7. **Proven by the device.** Its own report turns the part's status; a person's witness goes in judged and
   notes. Then look at the face.
8. **Upstream.** A fix that works becomes a series ready to send. Sending is the author's decision, in his
   name: say it waits for him; never send mail yourself.
9. **Recorded as you go**, in the project's data: decisions with what was rejected, research, the backlog
   item's next step, the session log, the DOMAIN-how-it-stands row. Where the ecosystem itself shows a gap,
   refine it first and record the refinement.
