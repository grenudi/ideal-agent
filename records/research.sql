-- What was learned, and from where (a research of that id is replaced).
-- --id        string        a-short-name
-- --question  string        what was asked
-- --found     string        what was found
-- --sources   string?       the sources' ids, comma-separated
-- --date      date = today  when
DELETE FROM research WHERE id = $id;
INSERT INTO research (id, date, question, found, sources) VALUES ($id, $date, $question, $found, $sources);
