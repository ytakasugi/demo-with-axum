INSERT INTO USERS(USER_NAME, E_MAIL)
 VALUES ($1, $2)
RETURNING *