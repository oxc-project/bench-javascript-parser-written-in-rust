# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [typescript.js](#typescript.js)

## Benchmark Results

### typescript.js

|                     | `oxc`                    | `swc`                            | `biome`                           |
|:--------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`single-thread`** | `49.79 ms` (✅ **1.00x**) | `104.17 ms` (❌ *2.09x slower*)   | `167.76 ms` (❌ *3.37x slower*)    |
| **`no-drop`**       | `49.97 ms` (✅ **1.00x**) | `97.29 ms` (❌ *1.95x slower*)    | `159.32 ms` (❌ *3.19x slower*)    |
| **`parallel`**      | `88.36 ms` (✅ **1.00x**) | `181.59 ms` (❌ *2.06x slower*)   | `330.30 ms` (❌ *3.74x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

