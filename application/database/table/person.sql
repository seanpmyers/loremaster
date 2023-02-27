CREATE TABLE "person" (
    "id" uuid NOT NULL,
    "email_address_id" uuid NOT NULL,
    "encrypted_password" text NOT NULL,
    "registration_date" timestamp(0) with time zone NOT NULL,
    "alias" text NULL,
    "chronicle_id" uuid NULL
);

CREATE INDEX "person_alias_index" ON "person" ("alias");

CREATE INDEX "person_registration_date_index" ON "person" ("registration_date");

ALTER TABLE "person"
    ADD PRIMARY KEY ("id");

ALTER TABLE "person"
    ADD CONSTRAINT "person_email_address_id_unique" UNIQUE ("email_address_id");
