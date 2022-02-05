CREATE TABLE public.person_chronicle (
	person_id uuid NOT NULL,
	chronicle_id uuid NOT NULL,
	date_recorded date NOT NULL,
	CONSTRAINT person_chronicle_pk PRIMARY KEY (person_id, date_recorded, chronicle_id)
);


-- public.person_chronicle foreign keys

ALTER TABLE 
	public.person_chronicle 
ADD CONSTRAINT 
	person_chronicle_fk 
FOREIGN KEY 
	(person_id) 
REFERENCES 
	public.person(id)
;

ALTER TABLE 
	public.person_chronicle 
ADD CONSTRAINT 
	person_chronicle_fk_1 
FOREIGN KEY 
	(date_recorded,chronicle_id) 
REFERENCES 
	public.chronicle(date_recorded,id)
;