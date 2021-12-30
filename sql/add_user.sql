INSERT INTO accounts.users(username, password_hash, token)
VALUES ($1,$2,$3)
RETURNING $table_fields