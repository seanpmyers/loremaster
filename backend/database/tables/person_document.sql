CREATE TABLE "public".person_document
(
 person_id   uuid NOT NULL,
 document_id uuid NOT NULL,
 CONSTRAINT PK_person_document PRIMARY KEY ( person_id, document_id ),
 CONSTRAINT FK_243 FOREIGN KEY ( document_id ) REFERENCES "public".document ( "id" ),
 CONSTRAINT FK_246 FOREIGN KEY ( person_id ) REFERENCES "public".person ( "id" )
);

CREATE INDEX fkIdx_244 ON "public".person_document
(
 document_id
);

CREATE INDEX fkIdx_247 ON "public".person_document
(
 person_id
);
