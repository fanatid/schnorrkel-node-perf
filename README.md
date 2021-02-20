# schnorrkel bindings test for Node.js

This project about testing performance of [github.com/w3f/schnorrkel](https://github.com/w3f/schnorrkel) bindings to Node.js.

Before run tests you need to install `wasm-pack` tool: [rustwasm.github.io/wasm-pack/installer](https://rustwasm.github.io/wasm-pack/installer/).

To run tests:

```bash
npm i
make bench
```

Results:
```
Input seed: DWHFeX6QLfsy3pe6xoCH0ONdsnAgNjv2
steateless/sign         time:   [2.9381 ms 2.9732 ms 3.0145 ms]                             
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe
steateless/verify       time:   [6.3220 ms 6.4078 ms 6.5072 ms]                              
Found 12 outliers among 100 measurements (12.00%)
  12 (12.00%) high severe

stateful/sign           time:   [2.3950 ms 2.4287 ms 2.4677 ms]                           
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
stateful/verify         time:   [5.8211 ms 5.8940 ms 5.9766 ms]                            
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

napi/stateful/sign x 338 ops/sec ±2.30% (83 runs sampled)
napi/stateful/verify x 169 ops/sec ±1.04% (85 runs sampled)
napi/stateless/sign x 289 ops/sec ±2.18% (85 runs sampled)
napi/stateless/verify x 155 ops/sec ±1.74% (87 runs sampled)
neon/stateless/sign x 279 ops/sec ±3.46% (82 runs sampled)
neon/stateless/verify x 147 ops/sec ±1.88% (82 runs sampled)
wasm/stateless/sign x 70.33 ops/sec ±1.33% (72 runs sampled)
wasm/stateless/verify x 27.42 ops/sec ±1.76% (49 runs sampled)
```

With features: `asm, u64_backend, avx2_backend`
```
steateless/sign         time:   [2.8704 ms 2.9095 ms 2.9550 ms]                             
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
steateless/verify       time:   [4.2636 ms 4.3263 ms 4.3970 ms]                               
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

stateful/sign           time:   [2.3244 ms 2.3635 ms 2.4094 ms]                           
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high severe
stateful/verify         time:   [3.6087 ms 3.6588 ms 3.7173 ms]                             
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

napi/stateful/sign x 356 ops/sec ±2.32% (84 runs sampled)
napi/stateful/verify x 275 ops/sec ±1.39% (86 runs sampled)
napi/stateless/sign x 282 ops/sec ±3.06% (83 runs sampled)
napi/stateless/verify x 228 ops/sec ±2.11% (82 runs sampled)
neon/stateless/sign x 297 ops/sec ±1.62% (85 runs sampled)
neon/stateless/verify x 232 ops/sec ±1.56% (83 runs sampled)
wasm/stateless/sign x 68.72 ops/sec ±1.95% (71 runs sampled)
wasm/stateless/verify x 27.47 ops/sec ±1.92% (49 runs sampled)
```
