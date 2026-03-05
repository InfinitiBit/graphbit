# GuardRail prebuilt library

GraphBit links GuardRail via this directory. **No GuardRail source is required.**

## Setup

1. Build GuardRail (in the guardrail repo):
   ```bash
   cd guardrail && cargo build -p ffi --release
   ```

2. Copy the static library here:
   ```bash
   cp guardrail/target/release/libguardrail_ffi.a graphbit/vendor/guardrail/
   ```
   (On Windows the file may be `guardrail_ffi.lib`; adjust build.rs if needed.)

3. Build GraphBit as usual; `graphbit-core`'s build.rs will link this `.a`.

## Layout

- `libguardrail_ffi.a` — single static library (C ABI). Must be present to build with GuardRail support.

## API note

The FFI exposes `guardrail_encode(..., encode_context)` and returns a JSON object `{ payload, rules_applied_count, rule_names, policy_name }`; `guardrail_decode` returns the same shape. Rebuild and copy the `.a` whenever the GuardRail FFI or core changes.
