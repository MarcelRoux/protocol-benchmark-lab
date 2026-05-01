Use a durable relational store as the source of truth, with artifacts on disk/object storage.

Recommended baseline

- PostgreSQL for metadata/state.
- Filesystem (or S3/MinIO later) for raw benchmark artifacts.
- Orchestrator writes both; API reads from Postgres.

Data design (scales with languages/scenarios)

1. languages

- id (pk), code (rust, go), name, version, container_image, active

1. protocols

- id, code (http, ws, grpc), name, active

1. scenarios

- id, key (http_baseline), workload_type (io|cpu), version, definition_hash, definition_path, active

1. benchmark_runs (one row per run request)

- id (uuid pk)
- language_id, protocol_id, scenario_id (fks)
- status (queued|running|completed|failed|canceled)
- requested_at, started_at, finished_at
- requested_by, priority
- error_code, error_message
- summary_json (p50/p95/p99/throughput/etc)
- artifact_uri (path to full raw output)
- orchestrator_version, service_version, scenario_hash (immutability/reproducibility)
5.benchmark_run_events (append-only lifecycle/audit)
- id, run_id (fk), event_type, event_time, payload_json

1. benchmark_run_metrics (optional normalized metrics for fast querying)

- run_id, metric_name, metric_value, unit, tags_json

How run records should be stored

- On POST /benchmarks/run: insert into benchmark_runs (queued) + insert event row.
- Worker picks run: transactionally move to running, set started_at, add event.
- Completion/failure: update status/timestamps/summary/artifact URI, add event.
- Keep raw full results outside DB; store only URI + summarized metrics in DB.

Why this works

- Normalized dimensions prevent duplication as languages/scenarios grow.
- Event table gives traceability and future replay/debug.
- Snapshot row (benchmark_runs) keeps API reads simple/fast.
- Easy to extend with new protocols, scenario versions, and richer metrics without schema churn.

Note:

- confirm definition of postgres in compose.yaml
