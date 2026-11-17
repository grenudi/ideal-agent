-- The practices in force, in their order (a retired one: tad q row --id ID).
SELECT id, practice, why, "for" FROM practices WHERE retired IS NULL ORDER BY id;
