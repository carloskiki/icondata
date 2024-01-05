# Wasm Binary Size Benchmarks

Here are some benchmarks for the binary sizes of generated `wasm` with all icons included in the binary.

## Enum

Benchmarks for when each icon is an `enum` variant of `Icon`.

| Build Config   | Total Size            |
| -------------- | ----------            |
| Debug no lto   | 1.332 MB + ~ 1kB/icon |
| Debug lto      | 1.374 MB              |
| Release no lto | 589 kB                |
| Release lto    | 60 kB + ~ 700kB/icon  |
