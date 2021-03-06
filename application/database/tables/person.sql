CREATE TABLE public.person (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	email_address text NOT NULL,
	encrypted_password text NOT NULL,
	registration_date timestamptz NOT NULL,
	alias text NULL,
	CONSTRAINT person_pk PRIMARY KEY (id),
	CONSTRAINT person_un UNIQUE (email_address)
);