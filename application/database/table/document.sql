CREATE TABLE "public".document
(
 "id"          uuid NOT NULL,
 type_id     uuid NOT NULL,
 content     text NOT NULL,
 category_id uuid NULL,
 CONSTRAINT PK_document PRIMARY KEY ( "id" ),
 CONSTRAINT FK_151 FOREIGN KEY ( type_id ) REFERENCES "public".document_type ( "id" ),
 CONSTRAINT FK_165 FOREIGN KEY ( category_id ) REFERENCES "public".document_category ( "id" )
);

CREATE INDEX fkIdx_152 ON "public".document
(
 type_id
);

CREATE INDEX fkIdx_166 ON "public".document
(
 category_id
);
