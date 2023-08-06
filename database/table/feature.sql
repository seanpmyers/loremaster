CREATE TABLE "feature" (
	"id" uuid NOT NULL UNIQUE PRIMARY KEY,
	"name" text NOT NULL UNIQUE,
	"creation_date" timestamp(0) with time zone NOT NULL
);
