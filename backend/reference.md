# API Reference

See the [SQL Schemas](https://github.com/KonstantinVVictoria/makeUC2022/blob/dev/backend/migrations/2022-10-23-015705_init/up.sql) to see the data format these endpoints accept

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