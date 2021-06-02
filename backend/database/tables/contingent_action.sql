CREATE TABLE "public".contingent_action
(
 contingent_action_id uuid NOT NULL,
 intended_action_id   uuid NOT NULL,
 CONSTRAINT PK_contingent_action PRIMARY KEY ( contingent_action_id, intended_action_id ),
 CONSTRAINT FK_137 FOREIGN KEY ( intended_action_id ) REFERENCES "public".action ( "id" ),
 CONSTRAINT FK_140 FOREIGN KEY ( contingent_action_id ) REFERENCES "public".action ( "id" )
);

CREATE INDEX fkIdx_138 ON "public".contingent_action
(
 intended_action_id
);

CREATE INDEX fkIdx_141 ON "public".contingent_action
(
 contingent_action_id
);
