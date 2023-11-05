```sql
CREATE TABLE IF NOT EXISTS BirthsDB (year INTEGER, month INTEGER, date_of_month INTEGER, day_of_week INTEGER, births INTEGER);
```

```sql
INSERT INTO BirthsDB (year,month,date_of_month,day_of_week,births) VALUES (2000,10,2,3,10000);
```

```sql
INSERT INTO BirthsDB (year,month,date_of_month,day_of_week,births) VALUES (2000,10,2,3,10000);
```

```sql
SELECT * FROM BirthsDB WHERE year = 2001;
```

```sql
UPDATE BirthsDB SET births=7869 WHERE year = 2003;
```

```sql
DELETE FROM BirthsDB WHERE year = 2003;
```

