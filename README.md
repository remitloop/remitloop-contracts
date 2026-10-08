# RemitLoop — remitloop-contracts

> Cross-border payment state tracking and reconciliation.

![RemitLoop Smart contracts](banner.png)

## About the project

RemitLoop tracks cross-border payments (remittances) from start to finish on Stellar/Soroban. Each transfer has a state that moves through its lifecycle, and the sending and receiving sides can reconcile their records against one shared, tamper-evident source instead of trading spreadsheets and emails. The loop closes when both sides agree a payment has settled.

**Who it is for:** Remittance operators, payment partners on both sides of a corridor, and the senders and recipients who want to know where their money is.

**How the pieces fit together:**

| Repository | Responsibility |
|---|---|
| `remitloop-contracts` | On-chain Soroban state and authorization — the source of truth |
| `remitloop-backend` | Off-chain indexing, read models and operational APIs |
| `remitloop-app` | User-facing web application |

Typical flow:

1. A payment's state is recorded and advanced on-chain by an authorized party (authorized by that party's Stellar account).
2. The backend indexes state changes into a read model for tracking and reconciliation reports.
3. Operators and customers open the web app to follow a payment and see whether both sides reconcile.

## This repository: Smart contracts

The **contracts** repository is the on-chain core of RemitLoop. It holds the minimum durable state the product needs and enforces who is allowed to change it. Anything that must be trustworthy and publicly verifiable lives here; everything else lives in the app and backend.

### What is included today

- A Soroban smart contract (`RemitLoopContract`, Rust, `#![no_std]`) in a Cargo workspace (`soroban-sdk` 28).
- `initialize(admin)` — sets the administrator; requires that address to authorize the call and stores it under the `ADMIN` key.
- `record(actor, value)` — writes an `i128` value under the `VALUE` key; requires `actor` to authorize the call.
- `read()` — returns the stored value (defaults to `0` if nothing has been recorded).
- A unit test (`records_value`) that initializes the contract, records `42` and reads it back.
- Size- and safety-focused release profile: `opt-level = "z"`, LTO, `overflow-checks = true`, `panic = "abort"`, stripped symbols.

> The generic `value` model is a placeholder. The architecture notes call for replacing it with a RemitLoop-specific state machine (see *Roadmap*) before any mainnet deployment.

### Tech stack

Rust · Soroban SDK 28 · Stellar CLI

### Getting started

```bash
# run the contract unit tests
cargo test

# build the WASM contract
stellar contract build

# formatting check (same as CI)
cargo fmt --all -- --check
```

The same three commands are available as `make test`, `make build` and `make fmt`.

## Roadmap

- Replace the generic `i128` value with the RemitLoop domain model.
- Add granular authorization (admin vs. authorized actors) and emitted events for the backend to index.
- Expand tests to cover unauthorized access and edge cases, then get an independent audit before mainnet.

## Maintainer

`@ollypee22`

## Status

**v0.1.0 development baseline — not audited and not production-ready.**

## Stellar alignment

The project uses Stellar/Soroban where on-chain state is the source of truth or where deterministic settlement is valuable. Off-chain services are kept out of consensus-critical logic.

## License

Apache-2.0
