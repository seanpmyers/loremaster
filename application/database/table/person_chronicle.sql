CREATE TABLE
    public.person_chronicle (
        person_id uuid NOT NULL REFERENCES "person" ("id"),
        chronicle_id uuid NOT NULL REFERENCES "chronicle" ("id"),
        date_recorded date NOT NULL,
        PRIMARY KEY (
            person_id,
            date_recorded,
            chronicle_id
        )
    );

CREATE INDEX
    "chronicle_date_index" ON "email_address" ("date_recorded");

CREATE INDEX
    "chronicle_person_index" ON "email_address" ("person_id");
