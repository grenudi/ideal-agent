# Steps that compose, device values as data (2026-11-19)

Asked by the author through the Top Coordinator: "is test procedures are small so that they can be later on
chained together and extracted into general tools that will work for any device ? do we have that reflected
somewhere in our structures ?" Before this, ideal-agent had only pieces of the answer: practice 20 (an action
past three asks for a tool), bring-up-a-part's "the first device is not the mould", and decision
2026-11-19-attended-one-command-away. The decision resting on this research is
2026-11-19-small-steps-compose.

## How others split steps from device values

- **LAVA.** A device dictionary extends a device-type template and sets only that device's values; the
  template holds the board family's logic. A new template is made only when a board needs more than its base.
  Sources: [device dictionary](https://lava.readthedocs.io/en/latest/technical-references/configuration/device-dictionary.html),
  [device-type template](https://lava.readthedocs.io/en/latest/technical-references/configuration/device-type-template.html).
- **pytest.** Fixtures request other fixtures. A parametrised fixture runs each test once per device, without
  the test knowing. A `conftest.py` lower down overrides a fixture by name.
  Source: [pytest fixtures](https://docs.pytest.org/en/stable/how-to/fixtures.html).
- **Ansible.** The role is the unit of reuse. Its `defaults/` have the lowest precedence; host and group values
  live in `host_vars/` and `group_vars/`. Sources: [roles](https://docs.ansible.com/ansible/latest/playbook_guide/playbooks_reuse_roles.html),
  [inventory](https://docs.ansible.com/ansible/latest/inventory_guide/intro_inventory.html).
- **labgrid.** Resources only describe access to the hardware. Drivers act on resources and compose. A
  strategy is the one place for "board- or project-specific actions". The docs warn against building
  higher-level fixtures on fixtures that change the board.
  Source: [labgrid overview](https://labgrid.readthedocs.io/en/latest/overview.html).

## When to share a step

- **The rule of three** (Don Roberts, in Fowler's *Refactoring*, 1999): extract on the third copy.
  Source: [Wikipedia](https://en.wikipedia.org/wiki/Rule_of_three_(computer_programming)).
- **The wrong abstraction** (Sandi Metz, 2016): "duplication is far cheaper than the wrong abstraction". When a
  shared step keeps gaining flags, copy it back into its callers.
  Source: [blog post](https://sandimetz.com/blog/2016/1/20/the-wrong-abstraction).

## What we take

1. Each step does one job, and procedures chain steps instead of branching on the device.
2. Device values live in the device's data, layered over family defaults; steps read names, never values.
3. A board's own path is one marked unit, like a labgrid strategy or a LAVA template.
4. A step is shared on the second device that needs it unchanged, or on its third copy if the copies differ.
   A shared step that grows a device's flags goes back to its callers.
5. A step that changes the device says so in its name, and is never hidden inside another.

## Not known yet

- Fuego was not looked at.
- Neither LAVA nor labgrid gives a number of uses after which a step must move into shared code.
