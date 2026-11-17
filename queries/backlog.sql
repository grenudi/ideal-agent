-- The work ahead: every item not done, the one in progress first.
SELECT id, status, title, note, waits FROM backlog WHERE status <> 'done' ORDER BY status = 'open', opened, id;
