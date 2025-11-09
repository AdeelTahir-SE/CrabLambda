<div align="center">

```
                                 ___          _     _                    _         
                                / __\_ __ __ _| |__ | |    __ _ _ __ ___ | |__   __| | __ _
                               / / | '__/ _` | '_ \| |   / _` | '_ ` _ \| '_ \ / _` |/ _` |
                              / /__| | | (_| | |_) | |__| (_| | | | | | | |_) | (_| | (_| |
                              \____/_|  \__,_|_.__/|_____\__,_|_| |_| |_|_.__/ \__,_|\__,_|
```

### 🦀 A blazingly fast Function as a Service backend in Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/yourusername/crablambda/releases)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/crablambda/actions)
[![SQLx](https://img.shields.io/badge/SQLx-0.7-purple.svg)](https://github.com/launchbadge/sqlx)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://hub.docker.com/r/yourusername/crablambda)

[Features](#features) • [Installation](#installation) • [Quick Start](#quick-start) • [Documentation](#api-documentation) • [Contributing](#contributing)

---

</div>

## 📖 Overview

CrabLambda is a high-performance, self-hosted Function as a Service (FaaS) platform written in Rust. It provides serverless function execution with persistent storage, versioning, and comprehensive execution tracking. Built with SQLx for robust database operations and leveraging Rust's memory safety guarantees for secure function execution.

## ✨ Features

- 🚀 **High Performance**: Built on Tokio for async execution and high concurrency
- 📦 **Function Deployment**: Deploy and manage functions via REST API
- 🔄 **Version Control**: Automatic versioning of function deployments
- 💾 **Persistent Storage**: SQLx-based storage with PostgreSQL or SQLite support
- 📊 **Execution Tracking**: Detailed logs and metrics for every invocation
- 🔒 **Resource Limits**: CPU, memory, and timeout constraints per function
- 🌐 **HTTP Triggers**: Simple HTTP endpoint invocation
- 🎯 **Multiple Runtimes**: Support for Rust, Python, Node.js, and more
- 📝 **Execution History**: Query past executions with filtering
- 🔐 **Isolated Execution**: Sandboxed function runtime environments

## 🏗️ Architecture

```
┌─────────────────┐
│   API Layer     │  (Axum/Actix-web)
│  - REST API     │
│  - Auth/Auth    │
└────────┬────────┘
         │
┌────────▼─────────────┐
│   Function Manager   │
│  - Versioning        │
│  - Metadata Storage  │
└────────┬─────────────┘
         │
┌────────▼─────────────┐
│   SQLx Data Layer    │
│  - Functions         │
│  - Versions          │
│  - Executions        │
└────────┬─────────────┘
         │
┌────────▼─────────────┐
│  Execution Engine    │
│  - Runtime Isolation │
│  - Resource Limiting │
└──────────────────────┘
```

## 📋 Prerequisites

- **Rust**: 1.70 or later
- **Database**: PostgreSQL 14+ or SQLite 3.35+
- **Docker**: (Optional) For containerized function execution
- **System**: Linux, macOS, or Windows with WSL2

## 🚀 Installation

### Using Cargo

```bash
# Install from crates.io
cargo install crablambda

# Or build from source
git clone https://github.com/yourusername/crablambda.git
cd crablambda
cargo build --release
```

### Using Docker

```bash
docker pull yourusername/crablambda:latest
docker run -p 8080:8080 -e DATABASE_URL=sqlite:data.db crablambda:latest
```

### Database Setup

#### PostgreSQL

```bash
# Create database
createdb crablambda

# Set environment variable
export DATABASE_URL="postgresql://user:password@localhost/crablambda"

# Run migrations
sqlx migrate run
```

#### SQLite

```bash
# Set environment variable
export DATABASE_URL="sqlite:crablambda.db"

# Run migrations (will create the database)
sqlx migrate run
```

## ⚙️ Configuration

Create a `.env` file or set environment variables:

```env
# Database
DATABASE_URL=postgresql://user:password@localhost/crablambda
# or for SQLite:
# DATABASE_URL=sqlite:crablambda.db

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Function Limits
MAX_FUNCTION_SIZE_MB=10
DEFAULT_MEMORY_MB=128
DEFAULT_TIMEOUT_SECS=30
MAX_TIMEOUT_SECS=300

# Execution
MAX_CONCURRENT_EXECUTIONS=100
EXECUTION_LOG_RETENTION_DAYS=30

# Optional: Authentication
API_KEY=your-secret-key
ENABLE_AUTH=true
```

### Run the Server

```bash
# Development mode
cargo run

# Production mode
cargo run --release

# Or use the binary
./target/release/crablambda
```

## 🎯 Quick Start

### 1. Deploy Your First Function

```bash
curl -X POST http://localhost:8080/api/functions \
  -H "Content-Type: application/json" \
  -d '{
    "name": "hello-world",
    "runtime": "rust",
    "code": "fn handler(event: serde_json::Value) -> String { format!(\"Hello, {}!\", event[\"name\"].as_str().unwrap_or(\"World\")) }",
    "handler": "handler",
    "memory_mb": 128,
    "timeout_secs": 5
  }'
```

### 2. Invoke the Function

```bash
curl -X POST http://localhost:8080/api/functions/hello-world/invoke \
  -H "Content-Type: application/json" \
  -d '{"name": "CrabLambda"}'
```

Response:
```json
{
  "execution_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "success",
  "output": "Hello, CrabLambda!",
  "duration_ms": 45
}
```

### 3. View Function Details

```bash
curl http://localhost:8080/api/functions/hello-world
```

## 💾 Database Schema

### Functions Table

```sql
CREATE TABLE functions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    runtime VARCHAR(50) NOT NULL,
    handler VARCHAR(255) NOT NULL,
    memory_mb INTEGER NOT NULL DEFAULT 128,
    timeout_secs INTEGER NOT NULL DEFAULT 30,
    current_version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_memory CHECK (memory_mb > 0 AND memory_mb <= 10240),
    CONSTRAINT valid_timeout CHECK (timeout_secs > 0 AND timeout_secs <= 900)
);

CREATE INDEX idx_functions_name ON functions(name);
CREATE INDEX idx_functions_created_at ON functions(created_at);
```

### Function Versions Table

```sql
CREATE TABLE function_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    function_id UUID NOT NULL REFERENCES functions(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    code TEXT NOT NULL,
    code_hash VARCHAR(64) NOT NULL,
    memory_mb INTEGER NOT NULL,
    timeout_secs INTEGER NOT NULL,
    deployed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    
    UNIQUE(function_id, version)
);

CREATE INDEX idx_versions_function_id ON function_versions(function_id);
CREATE INDEX idx_versions_deployed_at ON function_versions(deployed_at);
```

### Executions Table

```sql
CREATE TABLE executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    function_id UUID NOT NULL REFERENCES functions(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL,
    input JSONB,
    output TEXT,
    error TEXT,
    duration_ms INTEGER,
    memory_used_mb REAL,
    executed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_status CHECK (status IN ('success', 'error', 'timeout', 'oom'))
);

CREATE INDEX idx_executions_function_id ON executions(function_id);
CREATE INDEX idx_executions_executed_at ON executions(executed_at);
CREATE INDEX idx_executions_status ON executions(status);
```

### Execution Logs Table

```sql
CREATE TABLE execution_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    execution_id UUID NOT NULL REFERENCES executions(id) ON DELETE CASCADE,
    level VARCHAR(20) NOT NULL,
    message TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_logs_execution_id ON execution_logs(execution_id);
```

## 📚 API Documentation

### Function Management

#### Create Function
```http
POST /api/functions
Content-Type: application/json

{
  "name": "my-function",
  "runtime": "rust",
  "code": "fn handler(event: Value) -> String { ... }",
  "handler": "handler",
  "memory_mb": 256,
  "timeout_secs": 10
}
```

#### List Functions
```http
GET /api/functions?page=1&limit=10
```

#### Get Function
```http
GET /api/functions/:name
```

#### Update Function
```http
PUT /api/functions/:name
Content-Type: application/json

{
  "code": "fn handler(event: Value) -> String { ... }",
  "memory_mb": 512
}
```

#### Delete Function
```http
DELETE /api/functions/:name
```

### Function Execution

#### Invoke Function
```http
POST /api/functions/:name/invoke
Content-Type: application/json

{
  "key": "value"
}
```

#### Invoke Specific Version
```http
POST /api/functions/:name/versions/:version/invoke
Content-Type: application/json

{
  "key": "value"
}
```

### Version Management

#### List Versions
```http
GET /api/functions/:name/versions
```

#### Get Version Details
```http
GET /api/functions/:name/versions/:version
```

#### Rollback to Version
```http
POST /api/functions/:name/rollback/:version
```

### Execution History

#### Get Executions
```http
GET /api/functions/:name/executions?status=success&limit=50
```

#### Get Execution Details
```http
GET /api/executions/:execution_id
```

#### Get Execution Logs
```http
GET /api/executions/:execution_id/logs
```

## 📂 Project Structure

```
crablambda/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── config.rs               # Configuration management
│   ├── api/
│   │   ├── mod.rs              # API module
│   │   ├── functions.rs        # Function endpoints
│   │   ├── executions.rs       # Execution endpoints
│   │   ├── versions.rs         # Version endpoints
│   │   └── middleware.rs       # Auth & logging middleware
│   ├── models/
│   │   ├── mod.rs
│   │   ├── function.rs         # Function model
│   │   ├── version.rs          # Version model
│   │   └── execution.rs        # Execution model
│   ├── db/
│   │   ├── mod.rs
│   │   ├── functions.rs        # Function queries
│   │   ├── versions.rs         # Version queries
│   │   └── executions.rs       # Execution queries
│   ├── executor/
│   │   ├── mod.rs
│   │   ├── runtime.rs          # Runtime abstraction
│   │   ├── rust.rs             # Rust runtime
│   │   ├── python.rs           # Python runtime
│   │   └── sandbox.rs          # Isolation layer
│   ├── error.rs                # Error types
│   └── utils.rs                # Utility functions
├── migrations/
│   ├── 001_create_functions.sql
│   ├── 002_create_versions.sql
│   ├── 003_create_executions.sql
│   └── 004_create_logs.sql
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/
├── examples/
│   ├── deploy_function.rs
│   └── invoke_function.rs
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
└── README.md
```

## 🛠️ Development

### Run Tests

```bash
# All tests
cargo test

# Integration tests only
cargo test --test integration_tests

# With output
cargo test -- --nocapture
```

### Database Migrations

```bash
# Create new migration
sqlx migrate add create_new_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Generate SQLx Metadata

```bash
# For offline compilation
cargo sqlx prepare
```

### Code Formatting & Linting

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check without building
cargo check
```

## 🐳 Docker Deployment

### Using Docker Compose

```yaml
version: '3.8'
services:
  crablambda:
    image: yourusername/crablambda:latest
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://crablambda:password@postgres:5432/crablambda
    depends_on:
      - postgres
  
  postgres:
    image: postgres:15
    environment:
      - POSTGRES_USER=crablambda
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=crablambda
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

Run with:
```bash
docker-compose up -d
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [SQLx](https://github.com/launchbadge/sqlx) - The Rust SQL Toolkit
- Powered by [Tokio](https://tokio.rs) - Asynchronous runtime
- Web framework: [Axum](https://github.com/tokio-rs/axum)
- Inspired by AWS Lambda and other FaaS platforms

## 📞 Support

- 📧 Email: support@crablambda.dev
- 💬 Discord: [Join our community](https://discord.gg/crablambda)
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/crablambda/issues)
- 📖 Docs: [docs.crablambda.dev](https://docs.crablambda.dev)

## 🗺️ Roadmap

- [ ] WebAssembly runtime support
- [ ] Event triggers (Cron, Queue, Webhook)
- [ ] Function composition & workflows
- [ ] Built-in monitoring dashboard
- [ ] Multi-region deployment
- [ ] Auto-scaling based on load
- [ ] GraphQL API support
- [ ] CLI tool for function management

---

<div align="center">

**[⬆ back to top](#crablambda)**

Made with 🦀 and ❤️ by the CrabLambda Team

</div>
