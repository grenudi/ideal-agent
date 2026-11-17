---
name: research-a-feature
description: The method of a feature research session - finding the best way to make one hard feature work (one that will not come by a fix or two), choosing among vectors, and handing the chosen way over as decision, backlog and a failing test. Use when your brief makes you research one feature, or before taking up a feature nobody has made work.
---

# Researching a feature

You research; you do not change the device or its kernel - reading it is fine, anything that changes it
belongs to the session that holds it. Your brief names the feature, the device and the project's places.

1. **What is.** The feature's rows in the project's data (support, tests, results), every decision,
   research and backlog row that names it, its DOMAIN-how-it-stands row, and what the author said of it.
   Measure what you can without changing anything.
2. **Every way it has been done**, inside the commons first, then outside: upstream and its mailing lists,
   the family's efforts (the same SoC, its siblings), the distributions, the vendor's own stack as observed
   behaviour (never its proprietary code), the vendor's GPL sources, talks and papers. For each: what it
   does, on which hardware, how far it got, what stopped it. Record each source.
3. **Vectors.** Two to five distinct ways to make it work here, each with: what it needs, what is already
   done, the unknowns and how to settle each cheaply, the risk to the reference device, the work in
   agent-hours, how it would be proven (which test, present or works), and whether it can go upstream.
4. **Choose**, by the canon and the project's interest: the best ratio of certain progress to cost, with a
   first step that settles the biggest unknown. Record it as a decision with every vector rejected and why.
5. **Hand over**, in the project's data: research rows with their sources, the decision, the chosen
   vector's steps as backlog items each with its proof - the queue of the session that changes the device -
   the failing test the feature needs, so it shows broken or unknown until it works, and the
   DOMAIN-how-it-stands row (record-the-work, "Hand-over").

Done when the research and decision are landed, the steps are on the backlog with their proofs, the test is
in the suite, and the face shows the feature's path. Report in a few plain lines: the vector chosen, why,
and the first step.
