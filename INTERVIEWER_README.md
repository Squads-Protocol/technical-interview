Subscription Service — Interviewer Architecture Guide

Purpose
- Provide an architecture-focused question set for a 30-minute interview.
- Encourage discussion of trade-offs, scalability, reliability, and maintainability.
- Use the simple subscription service as a backdrop to probe practical judgment.

How To Use
- Skim candidate’s approach for 2–3 minutes (repo overview, goals).
- Pick 2–3 primary themes based on their background (e.g., pagination tokens, data modeling, observability).
- Use open-ended prompts; drill down with scenario-based “what if” follow-ups.

Core Themes and Question Prompts

1) API and Service Boundaries
- What is the public contract of this service? Who are the consumers?
- How would you evolve the proto without breaking existing clients? Versioning strategy?
- For pagination: why offset/limit vs cursor-based tokens? When would you switch approaches?
- Should Subscribe and Unsubscribe be idempotent? How do you communicate idempotency to clients?
- How would you expose bulk operations (bulk subscribe/unsubscribe)? Trade-offs for latency and atomicity?

What to listen for
- Clear articulation of backward compatibility, versioning, and API evolution.
- Understanding of pagination stability, determinism, and token design.

API Design Deep Dive
- RPC surface: what methods are essential now vs soon (Subscribe, Unsubscribe, List, Get, Bulk ops)? How to design bulk semantics (partial success)?
- Validation and idempotency: explicit contract in docs/proto; mapping duplicates to AlreadyExists vs no-op success.
- Pagination contracts: deterministic sorting, token format (cursor encoding), expiry policy, forward/backward paging.
- Filtering semantics: fields (domain, status, created_at range), case handling, normalization responsibilities.
- Versioning: protobuf compatibility (field tags, reserved, oneof evolution), deprecation signaling, server-side defaulting.
- Security and multi-tenancy: authZ via metadata, tenant scoping, rate limits, abuse protections.

Mini exercise
- Propose proto updates for: (a) multi-tenant requests, (b) domain filter, (c) cursor-based pagination with next_page_token. Discuss backward compatibility impacts.

2) Data Modeling and Normalization
- How would you model subscriptions beyond a single email column? (e.g., id, created_at, status)
- Would you normalize emails (lowercase, trim)? Where (app vs DB)?
- What unique constraints and indexes would you add? How would it change for Postgres?
- If we add tenant_id or user_id, how does schema and index strategy change?

What to listen for
- Awareness of canonicalization to prevent duplicates and enforce uniqueness.
- Thoughtful indexing (unique index on normalized email; composite indexes for tenant).

Schema Design Deep Dive
- Core table shape: fields beyond email (id, created_at, updated_at, status, source, tenant_id).
- Keys and constraints: natural vs surrogate primary key; unique constraints (global vs per-tenant); FKs if multi-tenant.
- Indexing: unique on normalized email; composite (tenant_id, normalized_email); functional index on LOWER(email); collation and case.
- Normalization: app vs DB triggers; preventing duplicates under race conditions.
- Data lifecycle: soft-delete vs hard-delete; retention and GDPR considerations.
- Migrations: zero-downtime rollout, backfills, roll-forward strategy, feature flags.

Mini exercise
- Sketch a minimal SQL schema to support multi-tenant unique emails with efficient list queries and substring filter. Which indexes and why?

3) Pagination Strategy and Consistency
- If using offset/limit, what are the pitfalls with insertions/deletions between pages?
- How would you design a cursor token? What fields would it encode (e.g., stable sort key)?
- How to ensure deterministic ordering for pagination (tie-breakers)?
- How do you cap page_size and set sensible defaults?

What to listen for
- Recognition of instability with offsets and a path to cursor-based tokens with stable sort keys.

4) Error Semantics and Status Codes
- Map common scenarios to gRPC codes: invalid email, duplicate subscribe, unsubscribe not found, DB failure.
- Would you mask internal DB errors? What detail would you expose (and where)?
- How would you prevent information leakage (e.g., email existence)?

What to listen for
- Consistent error taxonomy (InvalidArgument, AlreadyExists, NotFound, Internal).
- Security/privacy consideration to avoid enumeration attacks.

5) Validation, Security, and Abuse Resistance
- What validation is sufficient for emails? What are the risks of overly strict or lax rules?
- How would you protect the service from brute-force or spam (rate limiting, quotas)?
- Any concern about SQL injection or malformed inputs with SQLx? How to keep it safe?

What to listen for
- Parameterized queries, minimal PII in logs, rate limiting at edge or service.

6) Observability and Operability
- What would you log on Subscribe/List/Unsubscribe? What would you NOT log (PII)?
- How would you add tracing spans and correlation IDs? Where do they originate?
- What metrics would you publish (RPS, error rates, p95 latency, DB query times)?
- What SLOs would you set and how would you alert on them?

What to listen for
- Structured logs with context, tracing with request IDs, actionable metrics.

7) Performance and Scaling
- What are expected read/write ratios? How does that guide optimization?
- Where would you add indexes? How to detect and prevent N+1 or full scans?
- If QPS grows 10x, what bottlenecks appear (DB connections, CPU, I/O)? How to mitigate?
- Would you introduce caching? Where (client, service, DB, CDN)? Consistency trade-offs?

What to listen for
- Understanding of DB constraints (connection pools), indexing, and realistic scaling paths.

8) Testing Strategy
- What tests would you prioritize: unit, integration, contract, property-based?
- How would you test pagination correctness (ordering, boundaries, token round-trips)?
- How to set up DB state deterministically in tests? In-memory SQLite vs containers?

What to listen for
- Emphasis on integration tests for DB behavior and targeted unit tests for validation/token logic.

9) Deployment, Configuration, and Migrations
- How do you manage config (env, files, flags)? Secrets management?
- Startup sequence: migrations on boot vs separate job? Rollback strategy?
- Health checks (liveness/readiness) and graceful shutdown for gRPC.

What to listen for
- Sensible config layering, safe migrations, readiness tied to DB availability.

10) Future Extensions and Trade-offs
- If we add domains, tags, or subscription topics, how does schema and API evolve?
- How would you support multi-tenant isolation and per-tenant limits?
- If moving to event-driven (audit logs, notifications), where do you put queues and ensure idempotency?

What to listen for
- Ability to extend the design without major rewrites; clear trade-off discussions.

11) Resilience and Reliability
- Timeouts, retries, and circuit breakers around DB and downstreams.
- Streaming vs unary RPCs: backpressure, flow control, partial failures.
- Idempotency keys for write operations to achieve at-least-once safety.

What to listen for
- Awareness of failure modes and mitigation (retry storms, thundering herd, exponential backoff).

12) Security, Multi-tenancy, and Abuse Protection
- Tenant scoping via metadata; enforcing isolation at query and index levels.
- Rate limiting and quotas (global, per-tenant, per-user); burst handling.
- PII protection in logs; authN/Z considerations for real deployments.

What to listen for
- Specific, enforceable strategies (filters on queries, composite indexes, minimal logging of PII).

Scenario-Based Follow-ups
- Pagination bug: client reports missing some items while paging. How do you reproduce and fix?
- Duplicate emails observed despite unique constraint: what could cause it (race, collation), and how to mitigate?
- Spikes cause DB timeouts: immediate mitigations (backoff, circuit breaker, pool tuning) and longer-term fixes.
- Clients stream large exports then disconnect mid-stream: how to manage resource cleanup and partial work?
- Abuse spike from a single IP: where to apply rate limiting (edge, gateway, service), and why?

Lightweight Rubric (Signals)
- Communicates assumptions clearly and proposes pragmatic defaults.
- Prioritizes deterministic pagination and backward-compatible APIs.
- Uses parameterized queries, proper error mapping, and avoids PII leakage.
- Demonstrates understanding of observability (logs, traces, metrics) and SLOs.
- Thinks in trade-offs; identifies when to move from SQLite to Postgres.

Quick Reference (Repo Context)
- Handlers: subscription/src/handlers/api.rs (List/Subscribe/Unsubscribe).
- Proto: subscription/proto/api.proto; generated code in subscription/src/api/gen/.
- Build: subscription/build.rs regenerates protos on build.
- Runtime: .env support, logging, env vars in subscription/src/main.rs.
- Tests: skeleton in subscription/tests/assignment.rs to encourage candidate tests.

