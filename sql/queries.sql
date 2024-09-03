-- TRUNCATE TABLE public.bakery CASCADE;

-- ALTER SEQUENCE bakery_id_seq RESTART WITH 1;
-- ALTER SEQUENCE chef_id_seq RESTART WITH 1;

select  id
,name
,profit_margin
from public.bakery
LIMIT 1000;


select  id
,name
,contact_details
,bakery_id
from public.chef
LIMIT 1000;


select b.name
from bakery b
join chef c on b.id = c.bakery_id
where c.name = 'Charles';
