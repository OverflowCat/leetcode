# Write your MySQL query statement below
SELECT
employee_id,
IF (
    MOD (employee_id, 2) = 0,
    0,
    IF (
        name LIKE 'M%', 0, salary
    )
) AS bonus
FROM Employees
ORDER BY employee_id;