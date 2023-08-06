CREATE TABLE "public".chronicle_document
(
 chronicle_id  uuid NOT NULL,
 document_id   uuid NOT NULL,
 date_recorded date NOT NULL,
 CONSTRAINT PK_chronicle_document PRIMARY KEY ( chronicle_id, document_id, date_recorded )
);

CREATE INDEX fkIdx_214 ON "public".chronicle_document
(
 chronicle_id,
 date_recorded
);

CREATE INDEX fkIdx_218 ON "public".chronicle_document
(
 document_id
);