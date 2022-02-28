CREATE TABLE "public".action
(
 "id"          uuid NOT NULL,
 Name        text NOT NULL,
 category_id uuid NULL,
 CONSTRAINT PK_action PRIMARY KEY ( "id" ),
 CONSTRAINT FK_254 FOREIGN KEY ( category_id ) REFERENCES "public".action_category ( "id" )
);

CREATE INDEX fkIdx_255 ON "public".action
(
 category_id
);