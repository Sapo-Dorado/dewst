INSERT INTO studying.decks(uuid, author, title)
VALUES ($1,$2, $3)
RETURNING $table_fields;