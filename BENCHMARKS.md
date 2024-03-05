# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [typescript.js](#typescript.js)

## Benchmark Results

### typescript.js

|                     | `oxc`                    | `swc`                            | `biome`                           |
|:--------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`single-thread`** | `58.31 ms` (✅ **1.00x**) | `210.81 ms` (❌ *3.62x slower*)   | `324.68 ms` (❌ *5.57x slower*)    |
| **`no-drop`**       | `58.43 ms` (✅ **1.00x**) | `193.08 ms` (❌ *3.30x slower*)   | `283.26 ms` (❌ *4.85x slower*)    |
| **`parallel`**      | `72.39 ms` (✅ **1.00x**) | `257.81 ms` (❌ *3.56x slower*)   | `434.60 ms` (❌ *6.00x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

