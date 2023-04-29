# Description
Video translator and subtitle generator

# Installation

### Linux and Wsl
```bash
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```
# Env config

### env files
You should use the .env.example file as a template to create your own .env file.

### Database

It's very important to have a local database running, you can use docker to run a postgres database.

```bash
$ docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
```
**Note:**
You can change the password/username/port if you want. 

This should create a postgres database running on port 5432.
With the URL: 
`postgresql://localhost/postgres?user=postgres&password=postgres&port=5432&sslmode=disable`
Use this URL on the .env file.

After that it's recommended to create a database called `video_translator` on the postgres database.

### migrations
Install sqlx-cli:
```bash
$ cargo install sqlx-cli --no-default-features --features postgres
```

After the database is running and the .env file is configured, you can run the migrations to create the tables on the database.

```bash
$ sqlx migrate run
```

**Important:**
You should be on the root folder of the project to run this command.

### Ngrok
A useful tool to expose your local server to the internet is ngrok, you can download it [here](https://ngrok.com/download).

***Linux and Wsl installation:***
```bash
$ wget https://bin.equinox.io/c/4VmDzA7iaHb/ngrok-stable-linux-amd64.zip
$ unzip ngrok-stable-linux-amd64.zip
$ sudo mv ngrok /usr/local/bin
```

**Running:**
```bash
$ ngrok http 8080
```

This should give you a URL that you can use to access the server from the internet.
Something like: `https://<random_string>.ngrok.io`
Set this URL on key `API_URL` on the .env file.

Now you can receive requests from the internet on your local server.
# Running

### Server
To run the server you can use the command:
```bash
$ cargo run
```
**Note:**
The migrations will run automatically when you run the server.

Watch mode:
```bash
$ cargo watch -x run
```

# More on migrations
You can find more information about migrations [here](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
But in short.

You can create a new migration with the command:
```bash
$ sqlx migrate add -r <migration_name>
```

This will create 2 SQL files on `migrations/` folder.
1. {timestamp}_{migration_name}.down.sql
2. {timestamp}_{migration_name}.up.sql

The first one is the rollback migration, and the second one is the migration itself.

Sqlx will use the DATABASE_URL env variable to run the migrations.

The same var will be used on the macros `sqlx::query!` and `sqlx::query_as!` to run the queries against the database,
this can cause problems if the database is not running or the URL is not correct.

To mitigate this you can run the `prepare` command:
```bash	
$ sqlx prepare
```
This will create a file called `sqlx-data.json` with the database schema, and sqlx will use this file to run the queries instead of the DATABASE_URL env var.
