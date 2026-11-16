# PR92 Home history confirmation

Follow-up to the P2 finding on PR92 head `5dd3f7325d299335afbb78b898bead0cb61aac7e`.
Home STORE/TAKE_HOME attempts now retain the initial carried quantity of the
item kind. The next refreshed inventory commits history only if that quantity
changes in the requested direction. Unchanged attempts are discarded; slot
reordering and stack merging do not establish progress. Visit reset clears
pending and confirmed history.

Validation:

- `cargo test --release --manifest-path borg-rs/Cargo.toml`: 298 passed, 4 ignored.
- `cargo clippy --release --all-targets --manifest-path borg-rs/Cargo.toml -- -D warnings`: passed.
- The same Clippy check with `--features test-town-review`: passed.
- `cmake --build build/review92 -j 4` (configured with `BORG_RS_FEATURES=test-town-review`): passed.
- `python3 harness/regress/town-review/run.py build/review92 /tmp/pr92-fix-native-final`: passed; compact native receipt adjacent.

The native fixture calls production `execute_store` and `settle_sale` in an
initialized engine. It checks no stamp before observation, successful and
canceled STORE/TAKE_HOME attempts, rejected TAKE_HOME handling, and visit reset.
It discards queued keys and supplies controlled refreshed inventory; it does
not execute the native UI transaction or reproduce the original seed-501 oil
cycle. The original livelock reproduction remains unconfirmed.
