# Axum API Template

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/axum-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![CI](https://github.com/arrkpong/axum-api-template/actions/workflows/ci.yml/badge.svg)

A production-ready [Axum](https://github.com/tokio-rs/axum) template with:

- **Clean Architecture** (API, Domain, Infrastructure)
- **Authentication** (JWT + Argon2)
- **Database** (PostgreSQL + SQLx)
- **Docker** (Multi-stage build + Compose)
- **Auto Migrations** (Runs on startup)

## 🚀 Quick Start

1. **Clone & Rename**

   ```bash
   cp -r axum-api-template my-new-project
   cd my-new-project
   ```

2. **Update Project Name**

   - Edit `Cargo.toml`: `name = "my-new-project"`
   - Edit `docker-compose.yml`: Change container names (optional)

3. **Start with Docker 🐳**

   ```bash
   docker-compose up -d --build
   ```

   _That's it! The server will start, create the database, and run migrations automatically._

4. **Verify**
   ```bash
   curl http://localhost:3000/api/v1/health
   ```

## 🛠️ Development

### Prerequisites

- Rust 1.84+ (Edition 2024)
- PostgreSQL 14+
- `sqlx-cli`: `cargo install sqlx-cli`

### Local Setup

```bash
cp .env.example .env
# Edit .env database credentials

sqlx database create
sqlx migrate run
cargo run
```

## 📁 Structure

```
src/
├── api/            # Routes, Handlers, Middleware
├── domain/         # Business Logic, Models
├── infrastructure/ # Database Repositories
├── common/         # Utilities (JWT, Validation)
└── config/         # App Config
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
