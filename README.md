# Marco Polo Rs
<img style="margin: 0 auto" src="https://cdn.britannica.com/53/194553-050-88A5AC72/Marco-Polo-Italian-portrait-woodcut.jpg"  height="500">

# Table of contents
- [About](#about)
- [Installation](#installation)
  - [Linux and Wsl](#linux-and-wsl)
- [Env config](#env-config)
  - [Env files](#env-files)
  - [Docker](#docker)
  - [Database](#database)
  - [Migrations](#migrations)
  - [Ngrok](#ngrok)
- [Running](#running)
  - [Server](#server)
  - [Watch mode:](#watch-mode)
- [More on migrations](#more-on-migrations)
- [CLI](#cli)
  - [The api_keys.json file](#the-api_keysjson-file)
  - [Installation](#installation-1)

# About
The Marco Polo project is a video translator, and subtitle generator,
capable of translating videos from more then 100 languages to more then 100 languages.

# Installation

### Linux and Wsl
```bash
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
$ sudo apt install libssl-dev
$ apt install pkg-config
```

# Env config

### Env files
You should use the .example.env file as a template to create your own .env file.

### Docker
Docker it's not required to run the project, but it's recommended to use it to run the database.

You can download docker [here](https://docs.docker.com/get-docker/).

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

### Migrations
Install sqlx-cli:
```bash
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
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
$ cd ~
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

### Watch mode:
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
$ cargo sqlx prepare --merged
```
This will create a file called `sqlx-data.json` with the database schema, and sqlx will use this file to run the queries instead of the DATABASE_URL env var.

# CLI

The CLI is a tool to translate videos, and generate subtitles locally.

The videos will not be uploaded to the server or youtube.

But the transcription and translation will be done by the AI api's, so you need internet connection to use the CLI and api's
keys.

### The api_keys.json file
The CLI will use the api_keys.json file to get the api keys needed.

```json
{
  "deepl": "your_api_key",
  "assemblyAi": "your_api_key"
}
```


### Installation

First create a folder to store the CLI files:

```bash
$ mkdir -p ~/.marco-polo-rs-cli
```

Then compile the project and copy the binary to the folder:


```bash
$ cargo build --release --package marco-polo-rs-cli --bin marco-polo-rs-cli && cp target/release/marco-polo-rs-cli ~/.marco-polo-rs-cli
```

Inside the folder you should have the binary and the api_keys.json file.

Create a .sh file to run the CLI and the api_keys.json file:

```bash
$ touch ~/.marco-polo-rs-cli/marco-polo.sh
$ touch ~/.marco-polo-rs-cli/api_keys.json
```

Add this to the .sh file:

```bash
#!/bin/bash
~/.marco-polo-rs-cli/marco-polo-rs-cli -k ~/api_keys.json $@
```

Make the file executable:

```bash
$ chmod +x ~/.marco-polo-rs-cli/marco-polo.sh
```

Create a symbolic link to the file:

```bash
$ sudo ln -s ~/.marco-polo-rs-cli/marco-polo.sh usr/local/bin/marco-polo
```

Restart the terminal.

Now you can run the CLI with the command:

```bash
$ marco-polo --help
```
