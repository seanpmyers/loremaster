CREATE TABLE "public".person
(
 "id"            uuid NOT NULL,
 email_address text NOT NULL,
 password      text NOT NULL,
 CONSTRAINT PK_person PRIMARY KEY ( "id" )
);