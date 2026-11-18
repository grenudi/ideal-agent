-- A route retired: the name an instruction once used (a tool, a script, a job route), what does its work now,
-- since when and why (a route of that id is replaced). scripts/route-check flags it wherever it is still named.
-- --id       string        the name as it is written (lab-agent, oco-work)
-- --instead  string        what does its work now - an intent and where it is resolved
-- --since    date = today  when it retired
-- --why      string        why
DELETE FROM routes WHERE id = $id;
INSERT INTO routes (id, instead, since, why) VALUES ($id, $instead, $since, $why);
