# Axum Demo API

This project is an Axum-based API, built with Rust and SQLx for PostgreSQL database interactions.

## Features

1. **POST /**: Accepts a JSON payload, validates the schema, and stores the data in PostgreSQL.
2. **GET /**: Fetches and returns the stored user data from PostgreSQL as JSON.
3. **Asynchronous**: Both endpoints are implemented asynchronously for better performance.
4. **Schema Validation**: Request payloads are validated before inserting into the database.

## Tech Stack

- Rust (latest stable version)
- axum
- tokio
- PostgreSQL
- sqlx 
- serde

## Setup

### 1. Clone the repository

```sh
git clone https://github.com/Zairi-Maissene/axum-demo-api.git
cd axum-demo-api
```
### 2. Configure the database
   Create a PostgreSQL database and update the database URL in .env file:

````sh
DATABASE_URL=postgres://user:password@localhost/db_name
````

### 3. Run the application

```sh
cargo build
cargo run
```

### API Endpoints
Make request to `http://localhost:3000/` or your SERVER_ADDRESS set in .env with the following payload:

#### 1. POST 
```sh
Request Body
{
  "username": "john_doe",
  "email": "john_doe@example.com"
}
````
#### 2. GET
```sh
Response
[
  {
    "id": 1,
    "username": "john_doe",
    "email": "john_doe@example.com"
   }
]
````
