UPDATE 
  USERS
SET
  E_MAIL = $1
  , UPDATED = CURRENT_TIMESTAMP
WHERE
  USER_ID = $2
RETURNING *