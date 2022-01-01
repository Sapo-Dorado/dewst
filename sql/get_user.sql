UPDATE accounts.users
SET token_hash=$1
WHERE username=$2
RETURNING $table_fields