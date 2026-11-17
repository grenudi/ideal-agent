-- A practice retired: no longer read at a session's start, its words and its reason kept in the row so
-- that why it was there is still known (the rest of the practice is kept).
-- --id       string   the practice (NN-a-short-name)
-- --retired  string   when and why: YYYY-MM-DD: what replaced it, or why it no longer holds
UPDATE practices SET retired = $retired WHERE id = $id;
