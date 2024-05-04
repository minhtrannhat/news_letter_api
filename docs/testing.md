# Testing Notes

## The Test Suite

- The OS will find an available port for the test suite to use.
- We use the same PostgreSQL database instance for both testing and production environment (might bite us in the ass later ?).

## Testing with `tokio`

- Each test has to be marked `#[tokio::test]`.
- This macro will build a new async runtime for that one test.
- By calling `spawn_app()`, we will get a API HttpServer running for the entire duration of the single test that the macro above belongs to.

## Databases and Testing

Every module level integration test we run, we spin up a new logical database with the name generate by `Uuid` crate.

This is easy and cheap, the database only lives in docker and therefore, we don't have to worry much about data size ballooning.

Example when running `\l`

```
psql postgres://postgres:password@localhost:5432
psql (16.2)
Type "help" for help.

postgres=# \l
                                                                    List of databases
                 Name                 |  Owner   | Encoding | Locale Provider |  Collate   |   Ctype    | ICU Locale | ICU Rules |   Access privileges
--------------------------------------+----------+----------+-----------------+------------+------------+------------+-----------+-----------------------
 1e644722-ccfe-4573-9caa-2477c6a4ba43 | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 2ecfa7a7-5365-42a4-8b06-1bba0a7f21a8 | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 5a783f04-00d6-491d-a47d-e8821ab75426 | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 941446de-a9a2-40b9-b9b4-e9905084eab8 | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 a60cf099-5f6c-4a6c-8179-7ed8e1c910cd | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 c9924d51-5585-4a1c-81b0-43e96fb67c16 | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 d4135928-eac5-47dd-8497-9268ea5a68c6 | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 e8406e27-80b5-47cb-8170-e3bced578c68 | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 f13ca340-888e-4e3c-a560-9e9c7f99e50e | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 newsletter                           | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 postgres                             | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           |
 template0                            | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           | =c/postgres          +
                                      |          |          |                 |            |            |            |           | postgres=CTc/postgres
 template1                            | postgres | UTF8     | libc            | en_US.utf8 | en_US.utf8 |            |           | =c/postgres          +
                                      |          |          |                 |            |            |            |           | postgres=CTc/postgres
(13 rows)

postgres=#
```
