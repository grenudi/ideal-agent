-- A decision, with what was rejected (a decision of that id is replaced).
-- --id        string        YYYY-MM-DD-what-was-decided
-- --decision  string        what was decided
-- --why       string        why: the evidence, the author's words
-- --rejected  string?       what was weighed and not chosen, and why not
-- --about     string?       the milestone it is about (a row of milestones, where this place keeps them)
-- --research  string?       the research it rests on (its id)
-- --date      date = today  when
DELETE FROM decisions WHERE id = $id;
INSERT INTO decisions (id, date, decision, why, rejected, about, research)
VALUES ($id, $date, $decision, $why, $rejected, $about, $research);
