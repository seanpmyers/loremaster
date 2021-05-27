CREATE TABLE 
IF NOT EXISTS
   chronicle
   (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      date_recorded date UNIQUE
   )
;
