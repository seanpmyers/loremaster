# loremaster

Streamlined planning and tracking of life goals, actions, and outcomes.

## Concept
A mix between a quest log, documentation, and a journal.

## Intentions
Attempt to remove all important information from ones head in put it into one organized location.
Reduce effort to organize and plan past and future actions taken in life.  


## Design

---
### Backend
---
### Languages

- Rust
- SQL  

### Dependencies 

### Rust Crates
- [anyhow](https://crates.io/crates/anyhow)
  - This library provides anyhow::Error, a trait object based error type for easy idiomatic error handling in Rust applications.
- [cap](https://crates.io/crates/cap)
  - An allocator that can track and limit memory usage. This crate provides a generic allocator that wraps another allocator, tracking memory usage and enabling limits to be set. 
- [chrono](https://crates.io/crates/chrono)
  - Date and time library for Rust
- [env_logger](https://crates.io/crates/env_logger)
  - Implements a logger that can be configured via environment variables.
- [log](https://crates.io/crates/log)
  - A Rust library providing a lightweight logging facade.
- [mobc](https://crates.io/crates/mobc)
  - A generic connection pool with async/await support.
- [mobc-postgres](https://crates.io/crates/mobc-postgres)
  - Postgres support for the mobc connection pool
- [Rocket](https://crates.io/crates/rocket)
  - Web framework with a focus on usability, security, extensibility, and speed. 
- [serde](https://crates.io/crates/serde)
  - Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
- [thiserror](https://crates.io/crates/thiserror)
  - This library provides a convenient derive macro for the standard library's std::error::Error trait.
- [tokio](https://crates.io/crates/tokio)
  - A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language. It is:
- [tokio-postgres](https://crates.io/crates/tokio-postgres)
  - A native, asynchronous PostgreSQL client
- [tokio-test](https://crates.io/crates/tokio-test)
  - Testing utilities for Tokio- and futures-based code 
- [toml](https://crates.io/crates/toml)
  - A TOML decoder and encoder for Rust. 
- [uuid](https://crates.io/crates/uuid)
  - Generate and parse UUIDs.


### Data Model

- Person/User/Client
- Actions
   - An action the user takes that is related to one or more of their goals or outcomes.
- Chronicle
   - All encompassing log/lore for a given date.

### Functionality
 
#### Database

- Store
- Query
- Create, Read, Update, and Delete


#### HTTP Server

- Serve frontend 
- Respond to requests to create, read, update, and delete

#### Authentication

- Login/Logout
- User Sessions


### Frontend

---

#### Languages

#### Dependencies

#### Functionality
