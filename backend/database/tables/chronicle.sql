CREATE TABLE "public".chronicle
(
 date_recorded date NOT NULL,
 "id"            uuid NOT NULL,
 CONSTRAINT PK_chronicle PRIMARY KEY ( date_recorded, "id" )
);
