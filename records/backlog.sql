-- An item of work ahead, or done (an item of that id is replaced).
-- --id      string         a-short-name
-- --title   string         what it is
-- --status  string = open  open, in-progress or done
-- --note    string?        where it stands, and the next step
-- --waits   string?        what it waits on, if anything
-- --opened  date = today   when it was opened
DELETE FROM backlog WHERE id = $id;
INSERT INTO backlog (id, title, status, note, opened, waits) VALUES ($id, $title, $status, $note, $opened, $waits);
