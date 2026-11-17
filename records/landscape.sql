-- How others do it: what they do, and what we take from it (an entry of that id is replaced).
-- --id       string        a-short-name
-- --name     string        what it is called
-- --project  string        whose it is
-- --what     string        what it does
-- --for_us   string        what we take, and what not
-- --link     string        https://...
-- --checked  date = today  when it was read
DELETE FROM landscape WHERE id = $id;
INSERT INTO landscape (id, name, project, what, for_us, link, checked) VALUES ($id, $name, $project, $what, $for_us, $link, $checked);
