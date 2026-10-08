# RemitLoop contract architecture

The contract owns only the minimum durable state needed by the product flow.

- `initialize(admin)` establishes the authorized administrator.
- `record(actor,value)` writes a domain value after actor authorization.
- `read()` exposes the stored value.

Replace the generic value model with the domain-specific state machine before mainnet deployment.
