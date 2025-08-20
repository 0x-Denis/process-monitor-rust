-- SQLite
SELECT
    DISTINCT start AS "Время нахождения процесса",
    procname AS "Имя процесса"
FROM 
    ProcessInfo
ORDER BY "Время нахождения"