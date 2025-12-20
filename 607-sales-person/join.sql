-- TLE
-- select p.name from SalesPerson p
-- where "RED" not in
-- (select name from Company where com_id in
--   (select com_id from Orders o where p.sales_id = o.sales_id));

select p.name from SalesPerson p
where not exists
(
    select 1 from Orders o JOIN Company c on o.com_id = c.com_id
    where p.sales_id = o.sales_id
        and c.name = 'RED'
);
