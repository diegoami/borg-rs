# v15R parity and link receipts

Runtime commit `5e6cd07565d145d8d5044d4c31536318ba38f785`. The
[adoption record](../../adoption/c-v15r/README.md) records the native
gates, fixture results, and independent review.

- [Smoke comparison](smoke-diff.log) and [per-log SHA-256 values](smoke-log-sha256.tsv)
  cover 3 scenarios.
- [Full comparison](full-diff.log) and [per-log SHA-256 values](full-log-sha256.tsv)
  cover 45 scenarios.
- The [link audit](no-c-borg-final.log) found no C Borg code in the Rust
  binary, matched all 155 production engine objects, and reproduced the
  Rust binary by relinking.

The manifests record the build source, binary, suite, and configuration
hashes for each run. The SHA-256 tables retain fingerprints of the
uncompressed key, message, and state logs for each implementation.
Raw gameplay logs are omitted from this receipt.
