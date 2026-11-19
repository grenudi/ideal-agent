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
   notes - an attended proof is one prepared command (keep-a-shared-resource, "An attended step"). Then
   look at the face.
8. **Upstream.** A fix that works becomes a series ready to send. Sending is the author's decision, in his
   name: say it waits for him; never send mail yourself.
9. **Recorded as you go**, in the project's data: decisions with what was rejected, research, the backlog
   item's next step, the session log, the DOMAIN-how-it-stands row. Where the ecosystem itself shows a gap,
   refine it first and record the refinement.

## Small steps that compose

A test or bring-up procedure is written so that its steps outlive the device it was made for (the author's
question, decision 2026-11-19-small-steps-compose):

- **One job per step**, a step a named call; a procedure is a chain of steps, never one long script. A step that
  changes the device says so in its name and is never hidden inside another.
- **Device values are data.** Addresses, partitions, prompts, timeouts, image names: in the device's own data,
  over its family's defaults. Steps read the names, never the values.
- **The board's own path in one place.** What only this board does - its boot path, its way back - is one
  marked unit. The steps above it are the same for every device.
- **Lifted on the second device.** A step proven on one device moves into the shared tool when a second
  device needs it as it is, or on its third copy if the copies differ. A shared step that starts collecting a
  device's flags is copied back out and extracted again.

Where the shared steps live is the project's: its brief names the tool (linux-phone-oco: lpoco's step library,
docs items `lpoco-attended` and `bringup-step-runner`).

## A device from nothing

Before the first part, the device itself - in this order (decision 2026-11-17-a-device-from-nothing):

1. **A way back.** Keep the stock image and a way to restore it; read the image's own facts from it (header,
   offsets, command line, ids).
2. **A feedback channel first**: what tells you the kernel ran - a serial console, a display the bootloader
   left on, a USB gadget shell where there is no display, a persistent log (pstore, ramoops) read on the next
   boot. Nothing is learned from an image that is accepted and then says nothing.
3. **A known-good boot**: the vendor's kernel - or the accepted stock image itself - with your own initramfs,
   reached over that channel: it proves the image format and offsets, and lets you read, read-only, what
   mainline needs (the command line, ids, reserved memory, regulators).
4. **Then mainline**, from the nearest supported sibling, one change at a time.
- **When stock is accepted and yours is not**: start from the accepted image and swap one part of yours in at a
  time - ramdisk, then kernel, then device tree, then header fields - until it fails; the last swap names it.
