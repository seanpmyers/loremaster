SELECT DISTINCT
   chronicle.id
   , chronicle.date_recorded
FROM
   public.chronicle
WHERE
   chronicle.date_recorded = CURRENT_DATE
;