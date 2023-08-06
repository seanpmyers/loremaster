SELECT
   email_address
   , encrypted_password
FROM
   public.loremaster.person
WHERE
   person.email_address = '$1'
LIMIT 
   1
;