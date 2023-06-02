# StudyHub BXL
Brussels Student Guide is een website waarbij studenten plaatsen kunnen opzoeken die dichtbij hun noden liggen. Deze noden zijn vooral studiegerelateerd maar ook budgetsgewijs. De look van de website zal vergelijkbaar zijn met die van Google Maps.

# Technologieën

## Database

SurrealDB

## Backend

Rust, Serde, Actix Web

## Frontend

Node.js, Vue.js, Typescript

## Documentatie Setup

### Prerequisites

#### Database

* [SurrealDB](https://surrealdb.com/install).

#### Backend

* The [Rust toolchain](https://www.rust-lang.org/tools/install).
* Database

#### Frontend

* [Node.js and npm](https://nodejs.org/en/download).
* Backend


### Run

All the following sections assume a starting position at the git repo root.

#### Database

* `surreal start --log trace --user root --pass root --bind 0.0.0.0:8000 memory` to start SurrealDB.
* `surreal import --conn "http://localhost:8000" --user root --pass root --ns db --db db database/Single2.surreal` to add all the data.

#### Backend

* `cd backend`
* `cargo run` to run in debug mode.

#### Frontend

* `cd frontend`
* `npm install` to install dependancies.
* add a `.env` file with `VITE_MAPBOX=YOUR_MAPBOX_API_KEY_HERE`.
* `npm run dev` to run in debug mode.
