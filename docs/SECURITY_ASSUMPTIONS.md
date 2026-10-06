# Security Assumptions

Status: PLANNED constraints; no security controls implemented.

| ID | Assumption | Consequence if violated |
|---|---|---|
| A-01 | Recipient runs the approved client with trusted OS and dependencies | Software can be patched to omit watermark or ledger; no universal prevention claim |
| A-02 | KEM and signature private keys are independent and protected | Key theft enables decryption/signing; signatures identify keys, not humans |
| A-03 | CA enrollment and publisher authorization are administered honestly | Malicious certificates or intentionally distributed source copies undermine attribution |
| A-04 | OS CSPRNG and cryptographic libraries operate correctly | Confidentiality/authenticity can fail; fail on RNG or library errors |
| A-05 | At least the required ledger trust policy and retained audit evidence remain honest | Administrator collusion/rollback can defeat infrastructure provenance |
| A-06 | Local clock, trust anchors and revocation epochs are controlled and fresh | Expired/revoked credentials may appear valid after rollback |
| A-07 | Eligible watermark carriers and trustworthy reference geometry exist | Extraction may fail or be inconclusive |
| A-08 | Evidence collection preserves originals and tool versions | Reproducibility and chain of custody are weakened |

An authorized recipient who controls usable KEM key material and has the package
can implement their own decryptor without a ledger or watermark. The specified
local software architecture cannot cryptographically force that recipient to use
the trusted workflow. Trusted execution/hardware or online key release would
change the architecture and are outside this approved phase. The invariant applies
to the trusted workflow, not every possible decryption of distributed ciphertext.

Revocation gates future approved workflows; it cannot retract a recovered CEK,
a released document, or a previously delivered key/package combination. Signature
validity alone does not attest correct client execution or the claimed build ID.
No remote attestation exists in this design.

Physical memory extraction, malicious firmware/hypervisor/kernel, privileged
debugging, sophisticated collusion, and guaranteed survival after arbitrary OCR
are outside the initial prototype, per the project specification. Same-user key
access and custom-client bypass are disclosed even without root privileges.
