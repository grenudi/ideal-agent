-- Where a practice is shown: a decision that follows it (the rest of the practice is kept).
-- --id        string   the practice (NN-a-short-name)
-- --shown_in  string   the decision, as owner/repo/decisions:id
UPDATE practices SET shown_in = $shown_in WHERE id = $id;
