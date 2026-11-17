-- The decisions, the latest first.
-- --since  date?  on or after
SELECT id, date, decision, rejected FROM decisions WHERE ($since IS NULL OR date >= $since) ORDER BY date DESC, id;
