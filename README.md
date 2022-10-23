# MakeUC submission

### Run the DB
```sh
docker compose up
```
### Run the HTTP API
```sh
cd backend/ && cargo run --release
```

# Routes

### POST
- Create a plant
  *  /api/plant
- Create a user
  * /api/users
### GET
- Read plant by id
  * /api/plant/\<id\>
- Read all plants belonging to a UID
  * /api/plants/\<uid\>
### PUT
- Update a plant by id
  * /api/plant/\<id\>
- Update a user by id
  * /api/users/\<id\>
### Delete
- Delete a plant by id
  * /api/plant/\<id\>
- Delete a user by id
  * /api/users/\<id\>
