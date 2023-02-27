CREATE TABLE "email_address" (
    "id" uuid NOT NULL,
    "display" text NOT NULL,
    "local_part" text NOT NULL,
    "domain" text NOT NULL,
    "validated" boolean NOT NULL DEFAULT '0',
    "validation_date" timestamp(0) with time zone NULL,
    "creation_date" timestamp(0) with time zone NOT NULL
);

CREATE INDEX "email_address_domain_index" ON "email_address" ("domain");

CREATE INDEX "email_address_display_index" ON "email_address" ("display");

CREATE INDEX "email_address_validated_index" ON "email_address" ("validated");

ALTER TABLE "email_address"
    ADD PRIMARY KEY ("id");
