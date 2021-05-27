# loremaster

Streamlined planning and tracking of your life goals, actions, and outcomes.

## Concept
a mix between a quest log, documentation, and a journal  
get all important information out of your head in one organized location  
reduce effort to organize your personal information  


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



## User Story

A *person* (user) opens the application.

The *person* starts with some *intention* they have (an objective or goal).  

Intention: Get a new Job

Plan the *actions* they must take to enact their intention.  
*Actions* can have *contingent actions* (basically sub tasks) that must be completed first.  
They can record an optional *outcome* for every action they take (what was the result of the action).   
For example if they complete and interview, they might be rejected which is an outcome or result. 

Actions:
  - Apply
    - Send Resume
      - Update Resume
        - Create Resume
    - Send Cover Letter
      - Write Cover Letter
  - Interview
    - Schedule date/time with company contact
    - Prepare
      - Practice
        - Leetcode
        - Whiteboarding
      - Study Role Topics
      - Read about company
      - Read about role
  - Accept Offer
  - Reject Offer

A *person* can create documentation for any subject matter they need with a *document*.
A *document* is a record with text written by the *person*.

A *Chronicle* is a log or recording of your daily actions and *documents*.
*Chronicles* are the basic way to track progress of a *person's* *actions* and progress towards their *intentions*.  
*Chronicles* are auto generated every day. There cannot be more than one chronicle for a given day.

When *actions* have been documented, they can then be planned with a *schedule*.  
This allows a *person* to set *actions* they want to complete on set intervals, for example they could plan daily actions to brush their teeth or practice leetcode.   
These *scheduled* *actions* will appear on relevant *chronicles* to help the user track their activity.