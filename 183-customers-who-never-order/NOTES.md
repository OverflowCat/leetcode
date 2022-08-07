```sql
# Write your MySQL query statement below
SELECT name FROM Customers
WHERE id NOT IN (
SELECT customerId FROM Orders
)
```