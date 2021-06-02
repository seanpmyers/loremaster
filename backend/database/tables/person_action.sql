CREATE TABLE "public".person_action
(
 action_id uuid NOT NULL,
 "id"        uuid NOT NULL,
 taken_on  timestamp NOT NULL,
 CONSTRAINT PK_person_action PRIMARY KEY ( action_id, "id" ),
 CONSTRAINT FK_116 FOREIGN KEY ( "id" ) REFERENCES "public".person ( "id" ),
 CONSTRAINT FK_120 FOREIGN KEY ( action_id ) REFERENCES "public".action ( "id" )
);

CREATE INDEX fkIdx_117 ON "public".person_action
(
 "id"
);

CREATE INDEX fkIdx_121 ON "public".person_action
(
 action_id
);
