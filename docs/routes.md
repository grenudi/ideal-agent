<!-- Generated from data/ by tad. Do not edit by hand. -->

# Routes

<a id="lab-agent"></a>
### lab-agent

Instead
: Claude Code on the lab runs the work; a long job is run so it outlives its caller, as the harness's subject says (tad q about --subject claude-code); messages are SendMessage; practices are scripts/ideal-agent practices

Since
: 2026-11-18

Why
: Retiring since 2026-11-17 (decision 2026-11-18-job-route-retires-next); its token answered 401 for 15 hours on 2026-11-18 and walleye lost z3 and z5b to a lab-agent job route written into an item.


<a id="oco-agent"></a>
### oco-agent

Instead
: Claude Code on the lab; a lab job only where the domain's own tool provides one

Since
: 2026-11-18

Why
: Part of the oco tools the author detached from on 2026-11-17.


<a id="oco-work"></a>
### oco-work

Instead
: scripts/agent-env (sourced before every command): the name, the token, branch, land

Since
: 2026-11-18

Why
: Retired with the oco tools, 2026-11-17 (decision 2026-11-17-a-neutral-start).


<a id="ocoeco"></a>
### ocoeco

Instead
: tad and ikb for data; land for landing; named queries for what stands

Since
: 2026-11-18

Why
: Retired with the oco tools, 2026-11-17 (decision 2026-11-17-practices-without-oco-tools).


<a id="landit"></a>
### landit

Instead
: a commit and land

Since
: 2026-11-18

Why
: Retired with oco-work, 2026-11-17.


<a id="OCOECO_BY"></a>
### OCOECO_BY

Instead
: AGENT_NAME, from the brief's Name: line

Since
: 2026-11-18

Why
: Retired with oco-work, 2026-11-17; agent-env still sets it for places that read it until they move.
