
# Solana ShadowLens (Rust Edition)

![Rust](https://img.shields.io/badge/language-rust-orange.svg)
![Rocket](https://img.shields.io/badge/framework-rocket-red)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Solana](https://img.shields.io/badge/platform-solana-3a3a3a)

## 🔍 Overview

**Solana ShadowLens** is a Rust-based web dashboard designed to bring transparency to the Solana blockchain by enabling users to inspect and analyze closed-source smart contracts. It decodes base64-encoded BPF bytecode from on-chain programs and displays it in a human-readable hexadecimal format.

This tool is aimed at developers, auditors, and researchers who want to reverse-engineer Solana programs that do not publish their source code, IDLs, or SDKs. It is built using the high-performance [Rocket](https://rocket.rs/) web framework.

---

## 🚀 Features

- Input any Solana Program ID
- Fetch on-chain program binary via RPC
- Decode base64-encoded data
- Display bytecode in readable hexadecimal format
- Web interface powered by Tera templates
- Easily extendable for advanced disassembly and reverse engineering

---

## 🛠️ Setup Instructions

### 🔧 Prerequisites

- Rust (install via https://rustup.rs)
- Cargo (comes with Rust)
- Git

### 📦 Clone the Repository

```bash
git clone https://github.com/YOUR_USERNAME/solana-shadowlens-rust.git
cd solana-shadowlens-rust
```

### 🏗️ Build and Run

```bash
cargo run
```

This will start a Rocket server at `http://localhost:8000`.

### 🌐 Using the Dashboard

1. Visit `http://localhost:8000`
2. Enter any valid Solana Program ID (e.g. `Vote111111111111111111111111111111111111111`)
3. View the decoded binary data of the program in hex format

---

## 📁 Project Structure

```
solana_shadowlens/
├── src/
│   └── main.rs            # Main Rocket application
├── templates/
│   ├── form.tera          # Input form template
│   ├── result.tera        # Analysis results template
│   └── error.tera         # Error message display
├── Cargo.toml             # Project dependencies and metadata
└── README.md              # This file
```

---

## ⚙️ Future Enhancements

- Add instruction discriminator extractor
- Heuristics-based IDL generator
- Integration with Ghidra or Binary Ninja plugins
- Program upgrade authority analysis
- Visualization of instruction & account relationships

---

## 🧑‍💻 Contributing

We welcome contributions to extend the capabilities of this tool. Please submit pull requests or open issues on GitHub.

---

## 📜 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## 🌐 References

- [Solana Docs](https://docs.solana.com/)
- [Rocket Framework](https://rocket.rs/)
- [Solana BPF](https://docs.solana.com/developing/on-chain-programs/overview)
- [Base64 Decoding](https://docs.rs/base64/)
- [Solana JSON-RPC](https://docs.solana.com/developing/clients/jsonrpc-api)

---
