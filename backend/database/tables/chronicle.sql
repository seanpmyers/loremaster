CREATE TABLE public.chronicle (
      date_recorded date NOT NULL,
      id uuid NOT NULL DEFAULT uuid_generate_v4(),
      CONSTRAINT pk_chronicle PRIMARY KEY (date_recorded, id)
   )
;