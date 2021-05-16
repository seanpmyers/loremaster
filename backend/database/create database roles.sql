BEGIN
  CREATE ROLE developer WITH NOLOGIN;
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role developer -- it already exists';
END
;

BEGIN
  CREATE ROLE broker WITH NOLOGIN;
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role broker -- it already exists';
END
;

BEGIN
  CREATE ROLE naes WITH NOLOGIN;
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role naes -- it already exists';
END
;

BEGIN
  CREATE ROLE loremaster_broker WITH NOLOGIN;
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role loremaster_broker -- it already exists';
END
;