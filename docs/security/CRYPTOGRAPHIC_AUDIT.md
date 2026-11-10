## 🛡️ Cryptographic Bill of Materials (CBOM) & PQC Migration Assessment

**Format**: CycloneDX (v1.6) | **First-Party Code Crypto Assets**: 0 | **Total Tracked Crypto Assets**: 0

### 📊 Post-Quantum Migration Scorecard

| Metric | Count | Migration Status |
|---|---|---|
| **Post-Quantum Ready (PQC)** | **0** | 🟢 Quantum-Resistant (NIST FIPS 203/204/205) |
| **Quantum-Vulnerable (Backlog)** | **0** | 🔴 At Risk of 'Harvest Now, Decrypt Later' |
| **Classical Symmetric / Digest** | **0** | 🟡 Classical Security (Requires AES-256 / SHA-256+) |
| **Asymmetric PQC Migration Progress** | **N/A (0 detected)** | (No asymmetric primitives detected in current scope) |

### 🎯 Cryptographic Supply Chain Coverage & Confidence

| Evaluation Layer | Coverage / Status | Audit Confidence Assessment |
|---|---|---|
| **First-Party Code (`src/`, `scripts/`)** | **100% Audited** (0 Custom Primitives) | 🟢 **HIGH** (Direct AST & SAST verified) |
| **Third-Party Supply Chain** | **36.2%** (17 of 47 dependencies cataloged) | 🔴 LOW (Known profiles assimilated) |
| **Overall Audit Confidence Score** | **37.5%** | 🔴 LOW (30 unassimilated supply chain dependencies) |

| **System OS Inventory** | **Pending Update** | 🟡 **Warning** (Local OS crypto components not yet inventoried) |

### ✅ Post-Quantum Ready Assets

> ⚠️ **No Post-Quantum Ready assets detected.** Immediate migration planning recommended for asymmetric key exchanges and digital signatures.

### ⚠️ Quantum-Vulnerable Assets & Remediation Plan

> ℹ️ **No quantum-vulnerable asymmetric assets found.** Asymmetric cryptographic primitives were not detected in current scope.

### 🔒 Classical Symmetric & Digest Assets

> ℹ️ **No classical symmetric assets detected.**

### ⚠️ Unassimilated Third-Party Binaries & Cryptographic Blind Spots

> ℹ️ *The following third-party dependencies do not have verified upstream CBOM attestations in the catalog. They lower the audit confidence score until explicit CBOMs or attestations are published.* 

| Dependency Name | Version | Package URL (purl) | Status |
|---|---|---|---|
| `Swatinem/rust-cache` | v2 | `pkg:github/Swatinem/rust-cache@v2` | 🟡 Unassimilated (No upstream CBOM) |
| `arrayvec` | 0.7.6 | `pkg:cargo/arrayvec@0.7.6` | 🟡 Unassimilated (No upstream CBOM) |
| `autocfg` | 1.5.0 | `pkg:cargo/autocfg@1.5.0` | 🟡 Unassimilated (No upstream CBOM) |
| `bitflags` | 2.11.1 | `pkg:cargo/bitflags@2.11.1` | 🟡 Unassimilated (No upstream CBOM) |
| `btoi` | 0.4.3 | `pkg:cargo/btoi@0.4.3` | 🟡 Unassimilated (No upstream CBOM) |
| `bumpalo` | 3.20.2 | `pkg:cargo/bumpalo@3.20.2` | 🟡 Unassimilated (No upstream CBOM) |
| `cfg-if` | 1.0.4 | `pkg:cargo/cfg-if@1.0.4` | 🟡 Unassimilated (No upstream CBOM) |
| `chess-engine` | 0.1.0 | `N/A` | 🟡 Unassimilated (No upstream CBOM) |
| `console_error_panic_hook` | 0.1.7 | `pkg:cargo/console_error_panic_hook@0.1.7` | 🟡 Unassimilated (No upstream CBOM) |
| `dtolnay/rust-toolchain` | stable | `pkg:github/dtolnay/rust-toolchain@stable` | 🟡 Unassimilated (No upstream CBOM) |
| `itoa` | 1.0.18 | `pkg:cargo/itoa@1.0.18` | 🟡 Unassimilated (No upstream CBOM) |
| `memchr` | 2.8.0 | `pkg:cargo/memchr@2.8.0` | 🟡 Unassimilated (No upstream CBOM) |
| `nohash-hasher` | 0.2.0 | `pkg:cargo/nohash-hasher@0.2.0` | 🟡 Unassimilated (No upstream CBOM) |
| `num-traits` | 0.2.19 | `pkg:cargo/num-traits@0.2.19` | 🟡 Unassimilated (No upstream CBOM) |
| `once_cell` | 1.21.4 | `pkg:cargo/once_cell@1.21.4` | 🟡 Unassimilated (No upstream CBOM) |
| *... and 15 more unassimilated dependencies* | | | |
