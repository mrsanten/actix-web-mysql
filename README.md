# Actix MySQL API

This is a Rust-based API built using the Actix Web framework. It manages user and group data, with endpoints for creating, reading, updating, and deleting users and groups. The application uses a MySQL database for data storage and follows RESTful design principles.

### Features

 - RESTful API for managing users and groups
 - User and group creation, retrieval, updating, and deletion
 - Status monitoring endpoint (/status) to check application health
 - Thread-safe shared state management using Arc and AtomicU32
 - MySQL database integration using sqlx for asynchronous database
   interactions

### Requirements

 - Rust (1.56 or newer)
 - MySQL database
 - Actix Web
 - sqlx for database interaction
 - serde for serialization/deserialization
 - uuid crate for UUID generation

### Installation

1. **Clone the repository**:
```
git clone https://github.com/mrsanten/actix-web-mysql.git
cd actix-web-mysql
```

2. **Install dependencies**:

Ensure you have Rust installed. If not, you can install it using rustup.
```
cargo install
```

3. **Set up the MySQL database**:

Create a MySQL database and run the necessary SQL scripts to set up the tables for users and groups. Example tables:
```
CREATE  DATABASE  IF  NOT  EXISTS  `actix_web_mysql`;
USE  `actix_web_mysql`;

DROP  TABLE  IF  EXISTS  `users_to_groups`;
DROP  TABLE  IF  EXISTS  `posts`;
DROP  TABLE  IF  EXISTS  `groups`;
DROP  TABLE  IF  EXISTS  `users`;

CREATE  TABLE  IF  NOT  EXISTS users
(
id VARCHAR(48) NOT NULL  UNIQUE,
name  VARCHAR(64) NOT NULL  UNIQUE,
email VARCHAR(256) NOT NULL  UNIQUE,
PRIMARY KEY (id)
);

CREATE  TABLE  IF  NOT  EXISTS  `groups`
(
`id`  BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`name`  VARCHAR(64) NOT NULL  UNIQUE,
PRIMARY KEY(id)
);

CREATE  TABLE  IF  NOT  EXISTS  `users_to_groups`
(
`user_id`  VARCHAR(48) NOT NULL,
`group_id`  BIGINT UNSIGNED NOT NULL,
FOREIGN KEY (`user_id`) REFERENCES  `users`(`id`),
FOREIGN KEY (`group_id`) REFERENCES  `groups`(`id`)
);

CREATE  TABLE  IF  NOT  EXISTS  `posts`
(
`id`  BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
`user_id`  VARCHAR(48) NOT NULL,
`title`  VARCHAR(128) NOT NULL,
`content`  TEXT  NOT NULL,
`created_at`  TIMESTAMP  DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY(id),
FOREIGN KEY (`user_id`) REFERENCES  `users`(`id`)
);

CREATE  USER  IF  NOT  EXISTS  'actix_web_mysql_user'@'localhost' IDENTIFIED BY  'actix_web_mysql_password';
GRANT  SELECT, INSERT, UPDATE, DELETE  ON  `actix_web_mysql`.*  TO  'actix_web_mysql_user'@'localhost';
```

### Configuration

The application requires a configuration file named config.json in the root directory. This file should contain the application and database settings:
```
{
    "app": {
        "url": "127.0.0.1",
        "port": 8080
    },
    "dao": {
        "url": "127.0.0.1",
        "port": 3306,
        "user": "your_mysql_username",
        "password": "your_mysql_password",
        "database": "your_database_name"
    }
}
```

### Running the Application

```
cargo run
```
The application should start, and you’ll see output indicating the server is running, e.g.:
```
Listening on: 127.0.0.1:8080
```

### API Endpoints

**User Endpoints**:

- GET ``/user/{id}``: Retrieve a user by ID.
- POST ``/user``: Create a new user. Accepts a JSON payload with the user’s information.
- PATCH ``/user``: Update an existing user. Accepts a JSON payload with the updated user’s information.
- DELETE ``/user/{id}``: Delete a user by ID.

**Group Endpoints**:

- GET ``/group/{id}``: Retrieve a group by ID.
- POST ``/group``: Create a new group. Accepts a JSON payload with the group’s name.
- PATCH ``/group``: Update an existing group’s name. Accepts a JSON payload with the old and new names.
- DELETE ``/group/{name}``: Delete a group by name.

**Status Endpoint**:

- GET ``/ping``: Returns a simple message indicating the application is running.

### Example Requests

1. GET User by ID:
```
curl -X GET http://127.0.0.1:8080/user/{user_id}
```
2. POST New User:
```
curl -X POST http://127.0.0.1:8080/user -H "Content-Type: application/json" -d '{"name": "John Doe", "email": "john.doe@example.com", "groups": []}'
```
