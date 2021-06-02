CREATE TABLE person_chronicle
(
 chronicle_id  uuid NOT NULL,
 date_recorded date NOT NULL,
 person_id     uuid NOT NULL,
 CONSTRAINT PK_person_chronicle PRIMARY KEY ( date_recorded, chronicle_id, person_id ),
 CONSTRAINT FK_198 FOREIGN KEY ( person_id ) REFERENCES "public".person ( "id" ),
 CONSTRAINT FK_205 FOREIGN KEY ( date_recorded, chronicle_id ) REFERENCES "public".chronicle ( date_recorded, "id" )
);

CREATE INDEX fkIdx_199 ON person_chronicle
(
 person_id
);

CREATE INDEX fkIdx_206 ON person_chronicle
(
 chronicle_id,
 date_recorded
);