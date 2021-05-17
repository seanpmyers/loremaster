CREATE TABLE 
IF NOT EXISTS
   chronicle
   (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      date_recorded date UNIQUE
   )
;

CREATE TABLE 
IF NOT EXISTS
   person
   (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid()
   )
;