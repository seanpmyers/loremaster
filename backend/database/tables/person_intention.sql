CREATE TABLE "public".person_intention
(
 intention_status_id uuid NOT NULL,
 intention_id        uuid NOT NULL,
 "id"                  uuid NOT NULL,
 set_on              timestamp NOT NULL,
 CONSTRAINT PK_person_goal PRIMARY KEY ( intention_status_id, intention_id, "id" ),
 CONSTRAINT FK_102 FOREIGN KEY ( "id" ) REFERENCES "public".person ( "id" ),
 CONSTRAINT FK_79 FOREIGN KEY ( intention_status_id ) REFERENCES "public".intention_status ( "id" ),
 CONSTRAINT FK_88 FOREIGN KEY ( intention_id ) REFERENCES "public".intention ( "id" )
);

CREATE INDEX fkIdx_103 ON "public".person_intention
(
 "id"
);

CREATE INDEX fkIdx_80 ON "public".person_intention
(
 intention_status_id
);

CREATE INDEX fkIdx_89 ON "public".person_intention
(
 intention_id
);
