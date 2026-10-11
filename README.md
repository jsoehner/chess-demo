# ♜ Advanced Grandmaster AI

A modern, high-performance browser-based chess game featuring a robust AI opponent built with Rust and WebAssembly.

---

## 🎯 Key Features

- 🧠 **WASM-Native Engine** — Optimized Alpha-Beta search compiled from Rust for near-native performance in the browser.
- 📊 **Real-time Evaluation Bar** — Visual feedback of the engine's assessment of the board.
- ⚡ **Self-Contained** — No CDN dependencies; fully functional offline once loaded.
- 🎮 **Full Chess Rules** — Castling, en passant, pawn promotion, and check/checkmate detection via the [shakmaty](https://github.com/niklasf/shakmaty) crate.
- 🧠 **Search Optimizations** — Transposition Tables (Zobrist), MVV-LVA move ordering, and quiescence search for tactical stability.
- 🌐 **Frontier LLM Backend** — Optional move selection via Gemini, Claude, or local-OpenAI models.
- 🎨 **Premium Glassmorphism UI** — Stunning modern design with smooth animations and high-quality SVG pieces.

---

## 📦 Project Structure

```
chess-demo/
├── index.html          # Main viewer page (WASM-powered)
├── pkg/                # Generated WASM artifacts (JS glue + binary)
├── chess-engine/       # Rust source code for the engine
│   ├── src/
│   │   ├── lib.rs      # WASM-exported API
│   │   ├── eval.rs     # Material + positional evaluation
│   │   └── search.rs   # Alpha-beta search logic
│   └── Cargo.toml      # Rust dependencies
├── build.sh            # Compile Rust → WASM
├── package.sh          # Create a distributable zip
└── README.md           # You are here
```

---

## 🚀 Getting Started

### Option A — Run the pre-built version
1. Ensure you have Python installed (or any local HTTP server).
2. Run `./build.sh` (if `pkg/` is missing).
3. Start a server: `python3 -m http.server 8000`.
4. Open **http://localhost:8000** in your browser.

### Option B — Build from source
**Prerequisites:**
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

**Build:**
```bash
./build.sh
```

---

## 🛠️ Technical Details

### Search & Evaluation
The engine uses a **Minimax** algorithm with **Alpha-Beta pruning**. To improve performance and stability, it includes:
- **Transposition Tables**: Caches positions to avoid redundant calculations.
- **Quiescence Search**: Prevents the "horizon effect" by searching capture sequences to stability.
- **Move Ordering**: Uses MVV-LVA to search the most promising moves first, increasing pruning efficiency.
- **Mate Distance Scoring**: Prioritizes the shortest path to checkmate.

### Backend Options
While the **WASM Core** is the default, you can also connect to frontier LLMs (Gemini, Claude, GPT) to see how modern language models play chess.

---

## 🛡️ Automated SBOM & CBOM Supply Chain Pipeline

This repository integrates an automated Software Bill of Materials (SBOM) and Cryptographic Bill of Materials (CBOM) supply chain pipeline to satisfy modern security and compliance mandates (Executive Order 14028, NIST SP 800-218 SSDF, and NIST IR 8547 PQC Migration Guidance).

For current cryptographic analysis and quantum readiness status, see the [Cryptographic Audit & PQC Migration Report](docs/security/CRYPTOGRAPHIC_AUDIT.md).

### CI Pipeline Stages

The GitHub Actions workflow [`.github/workflows/sbom-cbom.yml`](.github/workflows/sbom-cbom.yml) executes on every push to `main` and all pull requests:

1. **Dual-Engine SBOM Generation**: Uses `anchore/sbom-action` to generate standard CycloneDX (`oss/sbom.cyclonedx.json`) and SPDX (`oss/sbom.spdx.json`) inventory files covering Rust crate dependencies and application assets.
2. **Cryptographic Bill of Materials (CBOM)**: Uses `cbomkit/cbomkit-action` to discover cryptographic algorithms, certificates, keys, and cryptographic assets (`oss/cbom.json`).
3. **Dual-Engine AST Semantic Crypto Scan**: Executes `scripts/scan_crypto_ast.py` to parse source code abstract syntax trees (ASTs), mapping cryptographic call-sites, key generations, and cipher initializations to the CBOM inventory.
4. **Automated Verification Harness**: Runs `scripts/test_boms.sh` to validate JSON schemas, verify required CycloneDX attributes, and enforce non-empty inventory thresholds as a build gate.
5. **Post-Quantum Cryptography (PQC) Assessment**: Runs `scripts/analyze_cbom.py` to classify cryptographic assets by quantum-readiness and output markdown summary dashboards to `$GITHUB_STEP_SUMMARY`.
6. **Continuous Governance Publishing**: Automatically syncs the latest cryptographic posture to [`docs/security/CRYPTOGRAPHIC_AUDIT.md`](docs/security/CRYPTOGRAPHIC_AUDIT.md) and publishes all BOM assets to workflow artifacts.

---

## 💻 Local Scripts & Verification

All pipeline stages can be executed locally in development environments using scripts in `scripts/`:

### 1. Generating BOMs (`scripts/generate_boms.sh`)

Generate SBOMs and CBOMs for either source directories or Docker container images:

```bash
# Scan current repository directory (outputs to oss/)
bash scripts/generate_boms.sh -t dir .

# Scan a local Docker image
bash scripts/generate_boms.sh -t docker chess-demo:local

# Specify custom output directory and format
bash scripts/generate_boms.sh -t dir -o build/boms -f cyclonedx-json .
```

### 2. Testing & Validating BOMs (`scripts/test_boms.sh`)

Run automated JSON validation, schema verification, and inventory count checks:

```bash
bash scripts/test_boms.sh oss
```

### 3. Scanning Source AST for Crypto Callsites (`scripts/scan_crypto_ast.py`)

Scan repository source code ASTs for cryptographic primitive invocations and reconcile with an existing CBOM:

```bash
python3 scripts/scan_crypto_ast.py . \
  --cbom oss/cbom.json \
  --output-json oss/crypto-callsites.json
```

### 4. PQC Readiness Analysis (`scripts/analyze_cbom.py`)

Analyze a generated CBOM against software dependencies and generate markdown or JSON risk summaries:

```bash
python3 scripts/analyze_cbom.py oss/cbom.json \
  --sbom oss/sbom.spdx.json \
  --json oss/crypto-analysis.json \
  --markdown docs/security/CRYPTOGRAPHIC_AUDIT.md
```

### 5. OCI Artifact Attachment (`scripts/attach_oci_bom.sh` & `scripts/fetch_oci_cbom.py`)

Publish and retrieve CBOMs attached to OCI container registries:

```bash
# Attach CBOM to container image in registry
bash scripts/attach_oci_bom.sh oss/cbom.json my-registry.io/app:v1.0.0

# Fetch CBOM from container image
python3 scripts/fetch_oci_cbom.py my-registry.io/app:v1.0.0 --output-dir oss
```

---

## 🏛️ Architectural Decision Records (ADRs)

Key architectural decisions governing this repository's security, supply chain, and cryptography strategy are documented under [`docs/adr/`](docs/adr/):

- [**ADR-0001: Automated SBOM and CBOM Supply Chain Governance Pipeline**](docs/adr/0001-automated-sbom-cbom-supply-chain-governance.md) — Context, alternatives considered, decision rationale, and downstream consequences for adopting automated dual-engine SBOM/CBOM scanning, AST call-site analysis, and CI gate verification.

---

Enjoy playing! ♟️
