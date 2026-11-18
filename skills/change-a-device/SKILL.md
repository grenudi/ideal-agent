---
name: change-a-device
description: Changing a shared test device safely: only through the tool that holds it, one change at a time, tests first, each change ending in its health report. Use before any reboot, kernel, flash, package or config change.
---

# Changing a shared device

The device is the reference test, shared by every session; a stranded device costs a person's hands.

- **Only through the tool that holds it.** One tool holds the device for every change - one at a time,
  across every session - and a second change waits. Never a hand-written ssh to the device; never wrap the
  tool in a lock of your own (it would wait on itself). Reading needs no hold. The project's brief names the
  tool and its words (linux-phone-oco: devlab - `devlab phone sh|root`, `kernel install|try`, `suite run`,
  the MCP door `devlab mcp`; its own guard refuses ssh/scp to a phone outside it).
- **Keep it known-working.** Experiments are one-off tries that fall back by themselves - a test kernel is a
  second boot entry tried once by boot counting, falling back to stock; never an initramfs debug shell on an
  unattended try, and never the device's own fallback - its watchdog, its boot counting -
  disabled on one - and are cleaned up after. A reboot, a try and a provision end with the new boot's health
  report: read it.
- **Wait on the condition, not the clock.** Wait for a boot or a result with the tool's own wait, or by watching
  until the condition holds - never a blind sleep (how, in your harness: ideal-agent's base, subject of the
  harness, e.g. `tad q about --subject claude-code`).
- **Test first.** A check that fails for the missing part, before the fix; generic checks where any device
  runs them, the device's own beside its data. Record what a pass proves (`present` - it is there;
  `works` - it does its work): only a passing `works` test turns a part `works`.
- **Results file themselves.** The device measures itself after every boot and the conveyor files the
  report. Never type a status a machine can measure; a person's witness goes in the judged field and notes.
  What is not measured is not known yet - never broken.
- **Witnessed tests.** Before anything the author must hear or see, say what to expect and what to check,
  and wait for his go. Keep sounds quiet. Notify urgently only when his hands or eyes are needed.
