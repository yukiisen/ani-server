# Ani-Server

Ani-Server is a REST API for the AniLib ecosystem built using Rust and Actix-web. This server provides various endpoints to fetch anime information, episodes, search results, latest updates, and synopsis.

## Features

- Fetch anime details by MyAnimeList ID
- Fetch anime episodes
- Search for anime
- Get the latest updates
- Fetch anime synopsis

## Getting Started

### Prerequisites

- Rust and Cargo installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).

### Installing

1. Clone the repository
   ```sh
   git clone https://github.com/yukiisen/ani-server.git
   cd ani-server
   ```

2. Build the project
   ```sh
   cargo build
   ```

3. Run the server
   ```sh
   cargo run
   ```

### Configuration

Configuration is loaded using the `ani_core::utils::config` module. Ensure that you have the necessary configuration file in place before running the server.

Check [ani-core](https://github.com/yukiisen/ani-core) for more information about how to configure the project.

### Endpoints

- `GET /v1/anime/{mal_id}`: Fetch anime details by MyAnimeList ID
- `GET /v1/anime/{mal_id}/episodes`: Fetch anime episodes
- `GET /v1/search`: Search for anime
- `GET /v1/updates/{offset}`: Get the latest updates
- `GET /v1/synopsis/{mal_id}`: Fetch anime synopsis
- Static files are served from the `/static` endpoint.

### Dependencies

The project relies on the following dependencies:
- [ani-core](https://github.com/yukiisen/ani-core)
- [actix-cors](https://crates.io/crates/actix-cors)
- [actix-web](https://crates.io/crates/actix-web)
- [log](https://crates.io/crates/log)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)
- [sqlx](https://crates.io/crates/sqlx)
- [thiserror](https://crates.io/crates/thiserror)
- [anyhow](https://crates.io/crates/anyhow)
- [env_logger](https://crates.io/crates/env_logger)
- [actix-files](https://crates.io/crates/actix-files)
