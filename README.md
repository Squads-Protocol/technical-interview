# Subscription Service

Small gRPC service for managing email subscriptions. This repo is used in a face‑to‑face interview. Keep it simple, ask clarifying questions, and implement the core tasks.

## Quick Start

- `cd subscription`
- Optional env vars: `DATABASE_URL` (default `sqlite://interview.db`), `PORT` (default `50051`), `RUST_LOG` (e.g., `info`)
- Run the server: `cargo run`
- Optional ping: `grpcurl -plaintext -d '{"message":"hello"}' localhost:50051 subscription.SubscriptionService/Ping`
- Optional tests: `cargo test` (there’s a skeleton in `subscription/tests/assignment.rs`)

Requirements: Rust + Cargo; SQLite 3 available on your system.

## Your Tasks (≈30 min)

- List subscriptions
  - Implement `list_subscriptions` to return emails.
  - Extend the proto to support an optional email filter and pagination.

- Subscribe hardening
  - Validate email; map invalid input to `InvalidArgument`.
  - Make duplicate subscribe clearly idempotent (document behavior or return `AlreadyExists`).

- Unsubscribe
  - Remove by email; return `NotFound` if missing. Be explicit about case sensitivity.

Optional add‑ons (pick one if time allows):
- Domain stats RPC (counts grouped by domain)
- Server‑streaming export RPC
- Idempotency‑Key on Subscribe
- Simple rate‑limiting or request tracing interceptor

## Clarify With Us

- Pagination: offset vs cursor token; expected stability across writes.
- Sorting: default order; deterministic tie‑breakers.
- Filtering: substring vs prefix; case sensitivity; normalization.
- Validation: how strict should email checks be; max lengths.
- Idempotency: duplicate subscribe/unsubscribe semantics and status codes.
- Page size: defaults and maximum caps.
- Error semantics: mapping DB and validation errors to gRPC codes; how much detail to expose.
- Observability: what to log (and not log), request IDs/tracing.

## Paths You’ll Touch

- Proto: `subscription/proto/api.proto`
- Handler: `subscription/src/handlers/api.rs`
- Entrypoint: `subscription/src/main.rs`
- Descriptor (optional for Postman): `subscription/src/api/gen/description.bin`

AI tools are allowed. Aim for small, clean changes and sensible defaults.
