# Rust Axum Observability Stack (local Windows)

A learning-oriented API starter with:
- Axum REST API + structured JSON tracing
- PostgreSQL 17
- Prometheus metrics endpoint (`/metrics`)
- Prometheus scrape configuration
- Grafana provisioned Prometheus datasource + dashboard
- Docker Compose stack for Windows Docker Desktop

## Prerequisites (Windows)

Install:
- Docker Desktop (Linux containers / WSL2 backend)
- Optional Rust toolchain if running `cargo run` on host

Recommended: run the complete stack via Docker Compose from PowerShell.

## Start the stack

```powershell
Copy-Item .env.example .env
docker compose up --build -d
docker compose ps
```

Services:
- API: http://localhost:3000
- Health: http://localhost:3000/health
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3001 (user `admin`, password `admin` — change for any non-local/shared use)
- API metrics: http://localhost:3000/metrics

Grafana dashboard: Dashboards -> Task API Overview.
Prometheus target: Status -> Targets; `task-api` should be UP.

## API quick tests (PowerShell)

Create task:

```powershell
Invoke-RestMethod -Method Post -Uri http://localhost:3000/api/tasks -ContentType "application/json" -Body '{"title":"Observability lab","description":"Try metrics and dashboards"}'
```

List tasks:

```powershell
Invoke-RestMethod http://localhost:3000/api/tasks
```

Filter:

```powershell
Invoke-RestMethod "http://localhost:3000/api/tasks?status=pending"
Invoke-RestMethod "http://localhost:3000/api/tasks?keyword=metrics"
```

Get:

```powershell
Invoke-RestMethod http://localhost:3000/api/tasks/1
```

Update:

```powershell
Invoke-RestMethod -Method Put -Uri http://localhost:3000/api/tasks/1 -ContentType "application/json" -Body '{"title":"Updated task","description":"Updated from PowerShell"}'
```

Change status:

```powershell
Invoke-RestMethod -Method Patch -Uri http://localhost:3000/api/tasks/1/status -ContentType "application/json" -Body '{"status":"completed"}'
```

Delete:

```powershell
Invoke-RestMethod -Method Delete http://localhost:3000/api/tasks/1
```

## Logging

Logs are emitted as JSON to stdout by tracing-subscriber. View API logs:

```powershell
docker compose logs -f api
```

Filter log levels:

```powershell
$env:RUST_LOG="debug"
cargo run
```

When running via Compose, `RUST_LOG` is set on the api service in compose.

## Metrics

Prometheus metrics exposed by API:
- `http_requests_total` (method, route template, status)
- `http_request_duration_seconds` histogram (method, route template)
- `task_api_build_info` gauge

Avoid high-cardinality metric labels such as raw URL paths containing task IDs. The code records route templates, not concrete IDs.

Prometheus config in `observability/prometheus.yml` scrapes the API at `api:3000` every 5 seconds.

## Useful commands

```powershell
docker compose down          # Stop/remove containers, persistent volumes stay
docker compose up -d         # Start existing stack
docker compose logs -f api
docker compose logs -f postgres
docker compose logs -f prometheus
docker compose logs -f grafana
docker compose ps
```

Do not use `docker compose down -v` unless you want to delete DB, metrics, and Grafana persisted data.

## Security note

This is a local learning/dev stack, not a hardened production deployment. Before exposing it to the Internet, add authentication/authorization, secret management, TLS via a reverse proxy, rate limiting, backup/restore, migrations, and security review. Change default Grafana credentials.
