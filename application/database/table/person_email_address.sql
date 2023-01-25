CREATE TABLE "person_email_address" (
    "person_id" uuid NOT NULL,
    "email_address_id" uuid NOT NULL
);

ALTER TABLE "person_email_address"
    ADD PRIMARY KEY ("person_id", "email_address_id");
