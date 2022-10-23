<<<<<<< HEAD

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

Start the PostgreSQL instance and run initial migrations:
In one terminal, start the PostgreSQL container:
```bash
  docker-compose up 
```
In another, run migrations:
```bash
diesel migration run --database-url postgresql://postgres:example@localhost:5432/postgres
```

Start the backend API

```bash
cd backend/ && cargo watch -x run
```
# API Reference

See the [SQL Schemas](https://github.com/KonstantinVVictoria/makeUC2022/blob/dev/backend/migrations/2022-10-23-015705_init/up.sql) to see the data format these endpoints accept

## User
#### Create user

```sh
  POST /api/user
```


#### Read user

```sh
  GET /api/user/<id>
```

#### Update user
```sh
PUT /api/users/<id>
```

#### Delete user
```sh
DELETE /api/user/<id>
```

## Plant
#### Create plant

```sh
  POST /api/plant
```


#### Read user

```sh
  GET /api/plant/<id>
```

#### Update user
```sh
PUT /api/plant/<id>
```

#### Delete user
```sh
DELETE /api/plant/<id>
```

=======
This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `pages/index.js`. The page auto-updates as you edit the file.

[API routes](https://nextjs.org/docs/api-routes/introduction) can be accessed on [http://localhost:3000/api/hello](http://localhost:3000/api/hello). This endpoint can be edited in `pages/api/hello.js`.

The `pages/api` directory is mapped to `/api/*`. Files in this directory are treated as [API routes](https://nextjs.org/docs/api-routes/introduction) instead of React pages.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details.
>>>>>>> 1fc432409959a85a7f3113732a2d8ca71564d79d
