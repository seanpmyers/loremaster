CREATE TABLE
    "dictionary_term"(
        "dictionary_id" uuid NOT NULL REFERENCES "dictionary",
        "term_id" uuid NOT NULL REFERENCES "term",
        PRIMARY KEY ("dictionary_id", "term_id")
    );
