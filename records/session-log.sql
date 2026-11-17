-- How a stretch of work went: what was done, considered and rejected (an entry of that id is replaced).
-- --id          string        YYYY-MM-DD-what
-- --title       string        in a few words
-- --done        string        what was done
-- --considered  string?       what was considered
-- --rejected    string?       what was rejected, and why
-- --date        date = today  when
DELETE FROM session_log WHERE id = $id;
INSERT INTO session_log (id, entry_date, title, done, considered, rejected) VALUES ($id, $date, $title, $done, $considered, $rejected);
