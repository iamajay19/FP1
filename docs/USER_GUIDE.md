# User Guide

Status: Phase 0 documentation/scaffold only. No document-processing application yet.

## Available demonstration

From repository root:

```sh
make help
make check
make environment
make encrypt
```

The first command explains scope. `make check` runs the Phase 0 checks and reports
any skipped Mermaid rendering. The environment command prints availability/failure
per probe; its success does not mean prerequisites are all ready. `make encrypt`
intentionally exits nonzero with `NOT IMPLEMENTED: Phase 0`; this is the expected
scaffold behavior, not a failed encryption test. It creates no output file.

For a complete review, open [ARCHITECTURE.md](ARCHITECTURE.md),
[CRYPTO_PROTOCOL.md](CRYPTO_PROTOCOL.md), [LEDGER_PROTOCOL.md](LEDGER_PROTOCOL.md),
[THREAT_MODEL.md](THREAT_MODEL.md) and [PHASE_0_REVIEW.md](PHASE_0_REVIEW.md).
Follow the flow from package validation through reservation, watermarking, signed
receipt and commit-gated output. Review [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md)
before approving Phase 1. No GUI, certificate creation or Alice/Bob demonstration
exists at this stage.

## Planned user journey

Administrator provisions offline CA/identities and trusted registry. Publisher
registers a document and distributes one signed CADP package with multiple recipient
entries. Recipient runs the trusted client, selects an encrypted identity and receives
only the verified watermarked artifact after a valid ledger receipt commit. A forensic
operator preserves the candidate, extracts the frame and independently verifies the
receipt/certificate/ledger evidence. Failures remain distinct and do not release raw
plaintext. Exact runtime commands will be added only with their implementation phases.

Never interpret a receipt as proof of a human leak or a watermark checksum as a
signature. The fully offline runtime demonstration is reserved for Phases 13/14.
