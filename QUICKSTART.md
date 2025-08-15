# 🚀 Quick Start Guide

## ⚡ Get Running in 5 Minutes

### 1. Install Rust (if not already installed)

```bash
# Visit https://rustup.rs/ and follow the installation instructions
# Or run this command:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install Required Tools

```bash
cargo install wasm-pack
cargo install trunk
```

### 3. Build and Run

```bash
# Build the project
cargo build

# Start the development server
trunk serve
```

### 4. Open Your Browser

Navigate to `http://localhost:8080`

---

## 🎯 What You'll See

1. **Input Form**: Enter your startup metrics
2. **Real-time Results**: Watch valuation update as you type
3. **Risk Analysis**: Get color-coded risk assessment
4. **Recommendations**: Actionable insights to improve metrics

---

## 🔧 Troubleshooting

### Common Issues:

**"cargo not found"**

- Install Rust from https://rustup.rs/

**"wasm-pack not found"**

- Run: `cargo install wasm-pack`

**"trunk not found"**

- Run: `cargo install trunk`

**Build errors**

- Make sure you're using Rust 1.70+ (run `rustc --version`)
- Try: `cargo clean && cargo build`

**Port 8080 already in use**

- Change port in `Trunk.toml` or kill the process using that port

---

## 📱 Try These Example Scenarios

### Scenario 1: Early Startup

- Revenue: $50K
- Growth: 25%
- Burn Rate: 40%
- Customers: 25

### Scenario 2: Growth Startup

- Revenue: $2M
- Growth: 15%
- Burn Rate: 20%
- Customers: 500

### Scenario 3: Mature Startup

- Revenue: $25M
- Growth: 8%
- Burn Rate: 10%
- Customers: 5,000

---

## 🎉 You're All Set!

The Startup Value Simulator is now running locally. Experiment with different metrics to see how they affect valuation and risk assessment.

For more detailed information, check out the full [README.md](README.md) and [demo examples](demo.md).

---

**Need help?** Check the troubleshooting section above or create an issue in the repository.


