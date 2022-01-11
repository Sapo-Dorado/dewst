INSERT INTO studying.decks(uuid, author)
VALUES ($1,$2)
RETURNING $table_fields;