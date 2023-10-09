# loremaster - Backend

The application is primarily written in Rust.

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
docker-compose -f ../database/docker-compose.yml up
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
