# Arcata 
<img alt="APM" src="https://img.shields.io/apm/l/vim-mode"> <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/konstantinvvictoria/makeUC2022/Rust"> 

Scaling a garden can be tricky. Many crops tend to thrive under conditions unique to them; some may require certain soil nutrients, or to be watered more or less frequently than others. Arcata is a garden management solution that provides a minimal interface for managing your plants. By integrating with 3rd party APIs, Arcata provides users with neat facts on the plants in their garden, and bundles them in a clean visualization so users can show off their gardens.


# Stack

**Client:** React, NextJS

**Server:** Rust, Rocket, Diesel, PostgreSQL


# Run Locally

### Dependencies (see setup below)
* Docker
* Rust
  * diesel_cli

---
Clone the project

```bash
git clone https://github.com/konstantinvvictoria/makeUC2022
```

Go to the project directory

```bash
cd makeUC2022
```

With the __Docker daemon running__, start the PostgreSQL instance

```bash
docker-compose up
```

Visit [rustup.sh](https://rustup.sh) to download Rust if you don't already have it installed, then install the diesel-cli to run our database migrations:

```bash
cargo install diesel_cli@1.4.1 --no-default-features --features postgres
```

Run initial DB migrations (from root directory)
```bash
diesel migration run --database-url postgresql://postgres:example@localhost:5432/postgres
```

Start the backend API

```bash
cd backend/ && cargo watch -x run
```
# API Reference

See the [SQL Schemas](https://github.com/KonstantinVVictoria/makeUC2022/blob/main/backend/migrations/2022-10-23-074500_init/up.sql) to see the data format these endpoints accept

## User
#### Create user

```sh
POST /api/user
```


#### Read user

```sh
GET /api/user/<uid>
```

#### Update user
```sh
PUT /api/users/<uid>
```

#### Delete user
```sh
DELETE /api/user/<uid>
```

## Plant
#### Create plant

```sh
POST /api/plant
```

#### Read all plants belonging to a user
```sh
GET /api/plants/<uid>
```

### Read a plant belonging to a user
```sh
GET /api/plant/<uid><id>
```

#### Update plant belonging to user
```sh
PUT /api/plant/<uid><id>
```

#### Delete plant belonging to user
```sh
DELETE /api/plant/<uid><id>
```
