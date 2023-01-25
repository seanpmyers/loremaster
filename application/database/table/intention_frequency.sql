CREATE TABLE "intention_frequency" (
    "intention_id" uuid NOT NULL,
    "frequency" frequency NOT NULL
);

ALTER TABLE "intention_frequency"
    ADD PRIMARY KEY ("intention_id", "frequency");
