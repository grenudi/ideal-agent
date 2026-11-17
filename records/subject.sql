-- A subject records are about: what it is (a subject of that id is replaced). A harness the agents run - Claude
-- Code, Codex - is a subject too: its how-to is research about it, dated and sourced, superseded when it moves.
-- --id    string   a-short-name
-- --name  string   what it is called
-- --what  string   what it covers, in a line
DELETE FROM subjects WHERE id = $id;
INSERT INTO subjects (id, name, what) VALUES ($id, $name, $what);
