# Standalone review counterexamples

Run from the repository root. The files here do not modify project code.
The Rust commands use the rlib produced by `cargo test --release`.

```sh
cc -DALLOW_BORG -DNDEBUG -ffunction-sections -fdata-sections -Iangband/src \
  docs/review-evidence-4c06cec08/borg-review-note.c \
  angband/src/borg/borg-item.c angband/src/borg/borg-item-id.c \
  -Wl,--gc-sections -o /tmp/borg-review-c-note
/tmp/borg-review-c-note

review_rlibs=(target/release/deps/libborg_rs-*.rlib)
rustc --edition 2021 docs/review-evidence-4c06cec08/borg-review-note.rs \
  --extern borg_rs="${review_rlibs[0]}" -L dependency=target/release/deps \
  -o /tmp/borg-review-rust-note
/tmp/borg-review-rust-note
```

Expected: C reports `hole note=[ longer {??}] needs_id=1`; Rust reports `hole note=[] needs_id=false`.

```sh
cc -DNDEBUG -fsanitize=address -ffunction-sections -fdata-sections \
  -Iangband/src -c angband/src/z-bitflag.c -o /tmp/borg-review-bitflag.o
rustc --edition 2021 docs/review-evidence-4c06cec08/borg-review-flag.rs \
  --extern borg_rs="${review_rlibs[0]}" -L dependency=target/release/deps \
  -C link-arg=/tmp/borg-review-bitflag.o -C link-arg=-lasan \
  -o /tmp/borg-review-safe-flag
ASAN_OPTIONS=verify_asan_link_order=0:detect_leaks=0 /tmp/borg-review-safe-flag
```

Expected: ASan invalid read at address `0x2`, exit nonzero. The Rust caller contains no unsafe. The linked C flag routine is the actual project source with the same `NDEBUG` policy as the harness.

For the allocated one-byte C variant:

```sh
cc -DNDEBUG -fsanitize=address -ffunction-sections -fdata-sections \
  -Iangband/src docs/review-evidence-4c06cec08/borg-review-flag.c \
  angband/src/z-bitflag.c -Wl,--gc-sections -o /tmp/borg-review-flag
ASAN_OPTIONS=detect_leaks=0 /tmp/borg-review-flag
```

Expected: heap-buffer-overflow in `flag_has`.

Wrong-binary acceptance:

```sh
BORG_IMPL=rust ANGBAND_BIN="$PWD/build/c/game/angband" JOBS=2 \
  harness/suite.sh smoke /tmp/borg-review-wrong-binary
harness/diff --suite smoke /tmp/borg-review-wrong-binary harness/reference/smoke
```

Expected: success, despite the manifest's C/Rust mismatch. The normal Rust suite result logs are separate from this negative experiment.
