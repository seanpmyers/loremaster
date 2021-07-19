CREATE TABLE public.person (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	email_address text NOT NULL,
	hashed_password text NOT NULL,
	alias text NULL,
	CONSTRAINT pk_person PRIMARY KEY (id)
);