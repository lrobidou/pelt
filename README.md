# PELT

A small lib to compute [Pruned Exact Linear Time](https://arxiv.org/pdf/1101.1438).

## Usage
```rust
let data: Vec<u64> = /*...*/;  // your data points
let penalty: f64 = /*...*/;  // a penalty score: 0 reports regions of exact matches, higher = higher noise tolerance
let breakpoints = pelt::pelt(&data, pelt::score, penalty);
```