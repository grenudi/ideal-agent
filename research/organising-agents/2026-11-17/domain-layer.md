# A domain layer over ideal-agent (2026-11-17)

## What is (docs origin/main cfbce35, ideal-agent a27b543)
linux-phone-oco/docs/ideal-agent-linux-phone-oco/: 8 briefs, 26,018 B. Read paragraph by paragraph: 49% general
(already a card, skill or practice of ideal-agent), 36% linux-phone domain, 15% one lane (surya or walleye).
Two contradict ideal-agent now: phone-keeper sends a turning point to orchestration (the cards: to the lane
coordinator); surya-coordinator starts keepers with start_session (the cards: launchers and a completion
subscription). overlay-check (runs of 6 words shared with ideal-agent): feature-research 27%, orchestration
22%, README 17%, the rest 5-11% - most restating is paraphrase, not copy. No device repository has a base or an
agent folder yet; device-template has a base and no release. Decisions: commons-and-device-bases (device work in
each device's base; lanes and launchers ask their device's base), chain-follow-by-slice (pin a release, follow
by slice, improvements back as upstream items).

## How others layer
Claude Code plugins: dependencies with a semver range, namespaced names; no agent extends another agent's body.
CLAUDE.md/AGENTS.md: additive, nearest wins, @-imports unpinned. Copilot: repository plus path-specific
instructions, additive. Helm library charts: a pinned dependency, templates included by name, a merge helper
for overrides. Kustomize: an overlay over a pinned base, patches only. Debian sub-policies declare the main
policy's version (Standards-Version, checked by lintian). Nobody checks that an overlay does not restate its base.
