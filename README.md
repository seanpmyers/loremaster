# loremaster

Streamlined planning and tracking of your life goals, actions, and outcomes.

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
 - [chrono](https://crates.io/crates/chrono)
   - Date and time library for Rust
 - [env_logger](https://crates.io/crates/env_logger)
   - Implements a logger that can be configured via environment variables.
 - [log](https://crates.io/crates/log)
   - A Rust library providing a lightweight logging facade.
 - [thiserror](https://crates.io/crates/thiserror)
   - This library provides a convenient derive macro for the standard library's std::error::Error trait.
 - [tokio](https://crates.io/crates/tokio)
   - A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language. It is:
 - [toml](https://crates.io/crates/toml)
   - A TOML decoder and encoder for Rust. 
 - [serde](https://crates.io/crates/serde)
   - Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
 - [SQLx](https://crates.io/crates/sqlx)
   - SQLx is an async, pure Rust† SQL crate featuring compile-time checked queries without a DSL. Used to interact with database.
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
