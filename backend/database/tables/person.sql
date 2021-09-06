CREATE TABLE public.person (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	email_address text NOT NULL,
	hashed_password text NOT NULL,
	alias text NULL,
	creation_date date NOT NULL,
	CONSTRAINT person_pk PRIMARY KEY (id),
	CONSTRAINT person_un UNIQUE (email_address)
);