ALTER ROLE developer CREATEDB CREATEROLE NOSUPERUSER;
GRANT developer TO naes;
GRANT application_broker TO loremaster_broker;
--ALTER ROLE '' WITH PASSWORD NULL;
GRANT
SELECT,
UPDATE,
INSERT,
  DELETE,
  TRUNCATE,
  REFERENCES,
  CREATE,
  CONNECT,
  EXECUTE,
  USAGE,
  TRIGGER,
  TEMPORARY ON ALL TABLES IN SCHEMA "public" TO developer;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA "public" TO developer;
GRANT
SELECT,
INSERT,
UPDATE,
  DELETE ON ALL TABLES IN SCHEMA "public" TO application_broker;