https://stackoverflow.com/questions/12048633/sql-query-to-find-record-with-id-not-in-another-table
​
```sql
# Write your MySQL query statement below
SELECT name AS Customers FROM Customers
WHERE id NOT IN (
SELECT customerId FROM Orders
)
```
​
```sql
# Write your MySQL query statement below
SELECT a.name AS Customers
FROM Customers a
LEFT JOIN Orders b
ON a.id = b.customerId
WHERE b.customerId IS NULL
```