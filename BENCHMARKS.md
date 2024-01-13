# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [typescript.js](#typescript.js)

## Benchmark Results

### typescript.js

|                     | `oxc`                    | `swc`                            | `Biome`                            |
|:--------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`single-thread`** | `55.86 ms` (✅ **1.00x**) | `108.68 ms` (❌ *1.95x slower*)   | `168.88 ms` (❌ *3.02x slower*)    |
| **`no-drop`**       | `56.17 ms` (✅ **1.00x**) | `101.47 ms` (❌ *1.81x slower*)   | `159.46 ms` (❌ *2.84x slower*)    |
| **`parallel`**      | `90.12 ms` (✅ **1.00x**) | `180.26 ms` (❌ *2.00x slower*)   | `315.39 ms` (❌ *3.50x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

