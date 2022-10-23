
# Arcata

Scaling a garden can be tricky. Many crops tend to thrive under conditions unique to them; some may require certain soil nutrients, or to be watered more or less frequently than others. In an effort to make home-gardens more scalable, Arcata provides users with a minimal dashboard to track and monitor each of their crops individually. Users can set unique watering frequencies for each of their plants, and Arcata will remind them when it's time to water them!




# Stack

**Client:** React, NextJS

**Server:** Rust, Rocket, Diesel, PostgreSQL


# Run Locally

Clone the project

```bash
  git clone https://github.com/konstantinvvictoria/makeUC2022
```

Go to the project directory

```bash
  cd makeUC2022
```

Start the PostgreSQL instance

```bash
  docker-compose up
```

Start the backend API

```bash
cd backend/ && cargo watch -x run
```
# API Reference

See the [SQL Schemas](https://github.com/KonstantinVVictoria/makeUC2022/blob/dev/backend/migrations/2022-10-23-015705_init/up.sql) to see the data format these endpoints accept

## User
#### Create user

```http
  POST /api/user
```


#### Read user

```http
  GET /api/user/<id>
```

#### Update user
```http
PUT /api/users/<id>
```

#### Delete user
```http
DELETE /api/user/<id>
```

## Plant
#### Create plant

```http
  POST /api/plant
```


#### Read user

```http
  GET /api/plant/<id>
```

#### Update user
```http
PUT /api/plant/<id>
```

#### Delete user
```http
DELETE /api/plant/<id>
```

