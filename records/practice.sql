-- A practice: how an agent works, why, and for whom (a practice of that id is replaced; the number in its
-- id is its place in the list).
-- --id        string   NN-a-short-name
-- --practice  string   what to do, in plain words
-- --why       string   what happened, or what the author said, that makes it so
-- --for       string   agents, or agents and people
-- --shown_in  string?  a decision that follows it (owner/repo/decisions:id)
DELETE FROM practices WHERE id = $id;
INSERT INTO practices (id, practice, why, "for", shown_in) VALUES ($id, $practice, $why, $for, $shown_in);
