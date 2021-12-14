CREATE TABLE "public".intention (
   id uuid NOT NULL DEFAULT uuid_generate_v4(),
   name text NOT NULL,
   CONSTRAINT PK_goal PRIMARY KEY (id),
   CONSTRAINT name_unique UNIQUE (name)
);
