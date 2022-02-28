CREATE TABLE public.chronicle (
      date_recorded timestamptz NOT NULL,
      id uuid NOT NULL DEFAULT uuid_generate_v4(),
      CONSTRAINT pk_chronicle PRIMARY KEY (date_recorded, id)
   )
;