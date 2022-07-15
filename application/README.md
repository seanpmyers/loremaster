# loremaster - Backend

[![dependency status](https://deps.rs/repo/github/seanpmyers/loremaster/status.svg)](https://deps.rs/repo/github/seanpmyers/loremaster/deps.rs)

The appilcation is primarily written in Rust.

## Tech Stack

- Programming Languages and Frameworks
  - [Rust](https://www.rust-lang.org/)
    - [Rocket](https://rocket.rs/)
  - [SQL](https://en.wikipedia.org/wiki/SQL)
- Databases / Datastores
  - [PostgreSQL](https://www.postgresql.org/)
- Development Operations
  - Cloud Service Provider
    - [AWS](https://aws.amazon.com/)
  - Version Control
    - [Git](https://git-scm.com/)
    - [GitHub](https://github.com/)
    - [Docker](https://www.docker.com/)
  - Observability/Logging

### Database

Docker compose command

```sh
docker-compose -f infrastructure/docker-compose.yml up
```

### Configuration File Schema - Loremaster.toml

```toml
TestField = "TestValue"

[local_debug]

[local_debug.database]
POSTGRESQL = "postgres://<user_name>:<password>@localhost/postgres"

[local_debug.encryption]
HASH_ITERATIONS = 1
SITE_SECRET = "<secret>"

[local_debug.web_server]
IPV4_ADDRESS = [127, 0, 0, 0]
PORT = 8000

[dev]

[qa]

[prod]
```
