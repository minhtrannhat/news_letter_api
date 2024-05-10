# Database Notes

## SQLx

The SQLx library will run compile time checks to make sure our SQL queries are valid. This is done by running PostgreSQL queries during compile time. Therefore, it is important that DATABASE_URL must be properly set.

### Offline mode vs Online mode

- Online mode is when the database is up and running and therefore, `SQLx` can perform compile time SQL queries check against it.
- Offline mode is when the database is NOT up and running. But we can save query metadata for offline usage and build to let the app run without SQLx complaining.
