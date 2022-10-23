## Dev setup
First, we'll want to start the Docker Postgres image
```sh
docker compose up
```

Export the url as an environment variable
```sh
export DB_URL=postgresql://postgres:example@localhost:5432/postgres
```

Run initial migration (if necessary)
```sh
diesel migrations run --database-url $DB_URL
```

View schemas
```sh
diesel print-schema --database-url $DB_URL
```

Start the HTTP server
```sh
cargo watch -x run
```