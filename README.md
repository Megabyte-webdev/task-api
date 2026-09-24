# Task API

A simple REST API built with **Rust** and **Axum**. The project demonstrates the basic structure of an asynchronous Rust web service: routes receive HTTP requests, handlers process application logic, and Serde serializes and deserializes JSON data.

## Core Concept

Rust provides a fast, memory-safe foundation for backend development. Axum is a lightweight web framework built on Tokio and Tower, making it straightforward to create asynchronous APIs with typed request handlers and composable middleware.

This example implements a small task service with an in-memory store. It is intended as a practical starting point for learning how Rust and Axum work together.

## Features

- Asynchronous HTTP server powered by Tokio
- Routing with Axum
- JSON request and response handling with Serde
- In-memory task management
- CRUD operations for tasks
- Health check endpoint

## Technology Stack

- Rust 2024 edition
- Axum 0.8
- Tokio
- Serde and Serde JSON

## Project Structure

```text
src/
├── main.rs              # Application entry point
├── models.rs            # Request and response models
├── routers.rs           # Application routes
├── state.rs             # Shared application state
└── handlers/
		└── tasks.rs         # Task endpoint handlers
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo

### Run the API

```bash
cargo run
```

The server listens on `http://localhost:8000`.

## API Endpoints

| Method | Endpoint      | Description                           |
| ------ | ------------- | ------------------------------------- |
| GET    | `/`           | Returns a welcome message             |
| GET    | `/health`     | Checks whether the service is running |
| GET    | `/tasks`      | Lists all tasks                       |
| POST   | `/tasks`      | Creates a task                        |
| GET    | `/tasks/{id}` | Gets one task                         |
| PATCH  | `/tasks/{id}` | Updates a task                        |
| DELETE | `/tasks/{id}` | Deletes a task                        |

### Create a Task

```bash
curl -X POST http://localhost:8000/tasks ^
	-H "Content-Type: application/json" ^
	-d "{\"title\":\"Learn Axum\"}"
```

### Update a Task

```bash
curl -X PATCH http://localhost:8000/tasks/1 ^
	-H "Content-Type: application/json" ^
	-d "{\"done\":true}"
```

## Development Notes

Tasks are stored in memory, so all data is lost when the server stops. A production-ready version would typically replace the shared in-memory state with a persistent database and add validation, error responses, authentication, and configuration management.
