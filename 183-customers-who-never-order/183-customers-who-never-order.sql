# Write your MySQL query statement below
SELECT a.name AS Customers
FROM Customers a
LEFT JOIN Orders b
ON a.id = b.customerId
WHERE b.customerId IS NULL
