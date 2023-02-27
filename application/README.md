# loremaster - Backend

The appilcation is primarily written in Rust.

## Tech Stack

- Programming Languages
  - [Rust](https://www.rust-lang.org/)
  - [SQL](https://en.wikipedia.org/wiki/SQL)
- Frameworks
  - Rust
    - [Axum](https://github.com/tokio-rs/axum)
    - [SQLx](https://github.com/launchbadge/sqlx)
    - [Sycamore](https://github.com/sycamore-rs/sycamore)
    - [Perseus](https://github.com/framesurge/perseus)
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

You can generate a site secret for testing with the following openssl command:

```bash
openssl rand -base64 64
```

Generate SSL/TLS certs 

(rename to cert.pem/key.pem)

```bash
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -sha256 -nodes --subj '/CN=localhost/'
```
