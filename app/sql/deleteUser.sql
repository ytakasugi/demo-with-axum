DELETE FROM
  USERS
WHERE
  USER_ID = $1
  AND DELETE_FLAG = true
RETURNING *