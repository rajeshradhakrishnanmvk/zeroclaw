# ZeroClaw Quick Reference Guide

## Project Structure

```
zeroclaw/
├── crates/
│   ├── zeroclaw-api/          # Core trait definitions
│   ├── zeroclaw-config/       # Configuration system
│   ├── zeroclaw-macros/       # Derive macros
│   ├── zeroclaw-providers/    # LLM provider implementations
│   ├── zeroclaw-channels/     # Channel implementations
│   ├── zeroclaw-tools/        # Tool implementations
│   ├── zeroclaw-memory/       # Memory backends
│   ├── zeroclaw-runtime/      # Agent runtime & loop
│   ├── zeroclaw-gateway/      # Webhook/API server
│   ├── zeroclaw-infra/        # Shared infrastructure
│   ├── zeroclaw-hardware/     # Hardware integration
│   ├── zeroclaw-plugins/      # WASM plugin system
│   └── zeroclaw-tool-call-parser/  # Tool call parsing
├── src/                       # CLI and main entry
├── docs/                      # Documentation
├── tests/                     # Integration tests
└── learning-plan/             # This learning plan!
```

## Core Traits

### 1. Provider Trait
**Location:** `crates/zeroclaw-api/src/provider.rs`

```rust
pub trait Provider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    async fn stream(&self, request: CompletionRequest) -> Result<Stream>;
    // ... other methods
}
```

**Purpose:** Abstract LLM providers (OpenAI, Anthropic, Gemini, etc.)

### 2. Channel Trait
**Location:** `crates/zeroclaw-api/src/channel.rs`

```rust
pub trait Channel {
    async fn send(&self, message: Message) -> Result<()>;
    async fn receive(&self) -> Result<Message>;
    // ... other methods
}
```

**Purpose:** Abstract messaging platforms (Telegram, Discord, Slack, etc.)

### 3. Tool Trait
**Location:** `crates/zeroclaw-api/src/tool.rs`

```rust
pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: ToolArgs) -> Result<ToolResult>;
}
```

**Purpose:** Define executable tools (file ops, shell, browser, etc.)

### 4. Memory Trait
**Location:** `crates/zeroclaw-api/src/memory_traits.rs`

```rust
pub trait Memory {
    async fn store(&self, entry: MemoryEntry) -> Result<()>;
    async fn retrieve(&self, query: Query) -> Result<Vec<MemoryEntry>>;
    // ... other methods
}
```

**Purpose:** Abstract memory backends (SQLite, Markdown, Vector DB, etc.)

### 5. Observer Trait
**Location:** `crates/zeroclaw-api/src/observability_traits.rs`

```rust
pub trait Observer {
    fn on_event(&self, event: Event);
    fn on_metric(&self, metric: Metric);
    // ... other methods
}
```

**Purpose:** Observability and monitoring

### 6. Peripheral Trait
**Location:** `crates/zeroclaw-api/src/peripherals_traits.rs`

```rust
pub trait Peripheral {
    async fn read(&self) -> Result<Data>;
    async fn write(&self, data: Data) -> Result<()>;
    // ... other methods
}
```

**Purpose:** Hardware integration (GPIO, USB, Serial, etc.)

## Common Commands

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific crate
cargo build --package zeroclaw-runtime

# Check without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests for specific crate
cargo test --package zeroclaw-memory

# Run integration tests
cargo test --test test_integration

# Run with output
cargo test -- --nocapture
```

### Running

```bash
# Run with default config
cargo run

# Run with specific command
cargo run -- config --help

# Run with debug logging
RUST_LOG=debug cargo run

# Run with trace logging
RUST_LOG=trace cargo run
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy

# Run clippy with warnings as errors
cargo clippy --all-targets -- -D warnings
```

### Documentation

```bash
# Build docs
cargo doc

# Build and open docs
cargo doc --open

# Build docs for all crates
cargo doc --workspace
```

## Configuration Example

```toml
# zeroclaw.toml

[agent]
name = "my-agent"
personality = "helpful and friendly"

[provider]
type = "openai"
api_key = "${OPENAI_API_KEY}"
model = "gpt-4"
temperature = 0.7
max_tokens = 2000

[memory]
type = "sqlite"
path = "./memory.db"

[channels.telegram]
enabled = true
bot_token = "${TELEGRAM_BOT_TOKEN}"

[security]
enable_sandbox = true
max_memory_mb = 512
allowed_paths = ["./workspace"]

[observability]
log_level = "info"
enable_metrics = true
```

## Key File Locations

### Entry Points
- **CLI Entry:** `src/main.rs`
- **Library Root:** `src/lib.rs`

### Core APIs
- **Provider API:** `crates/zeroclaw-api/src/provider.rs`
- **Channel API:** `crates/zeroclaw-api/src/channel.rs`
- **Tool API:** `crates/zeroclaw-api/src/tool.rs`
- **Memory API:** `crates/zeroclaw-api/src/memory_traits.rs`

### Implementations
- **Providers:** `crates/zeroclaw-providers/src/`
- **Channels:** `crates/zeroclaw-channels/src/` or `src/channels/`
- **Tools:** `crates/zeroclaw-tools/src/`
- **Memory:** `crates/zeroclaw-memory/src/`

### Runtime
- **Agent Loop:** `crates/zeroclaw-runtime/src/agent/loop_.rs`
- **Security:** `crates/zeroclaw-runtime/src/security/`
- **Observability:** `crates/zeroclaw-runtime/src/observability/`

### Configuration
- **Config Schema:** `crates/zeroclaw-config/src/schema.rs`
- **Config Loading:** `crates/zeroclaw-config/src/lib.rs`

### Documentation
- **Book:** `docs/book/src/`
- **Extension Guide:** `docs/book/src/developing/extension-examples.md`
- **Contributing:** `docs/book/src/contributing/`

## Stability Tiers

| Tier | Meaning | Examples |
|------|---------|----------|
| **Stable** | Breaking changes require major version | (None yet, targeting v1.0.0) |
| **Beta** | Breaking changes in MINOR versions | `zeroclaw-config`, `zeroclaw-macros` |
| **Experimental** | No stability guarantee | `zeroclaw-runtime`, `zeroclaw-channels` |

## Common Patterns

### Implementing a Custom Provider

1. Implement `Provider` trait
2. Add configuration struct with `#[derive(Configurable)]`
3. Register in provider factory
4. Add to config schema
5. Write tests

### Implementing a Custom Channel

1. Implement `Channel` trait
2. Add configuration struct
3. Register in channel factory
4. Handle webhooks or polling
5. Write tests

### Implementing a Custom Tool

1. Implement `Tool` trait
2. Define name, description, parameters
3. Implement execute logic
4. Register in tool registry
5. Write tests

## Troubleshooting

### Build Issues

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for issues
cargo check
```

### Runtime Issues

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Enable trace logging for specific module
RUST_LOG=zeroclaw_runtime=trace cargo run

# Check configuration
cargo run -- config validate
```

### Test Issues

```bash
# Run single test with output
cargo test test_name -- --nocapture

# Run tests serially
cargo test -- --test-threads=1

# Show test output
cargo test -- --show-output
```

## Environment Variables

```bash
# Rust logging
export RUST_LOG=debug

# Provider API keys
export OPENAI_API_KEY=sk-...
export ANTHROPIC_API_KEY=sk-ant-...
export GEMINI_API_KEY=...

# Channel tokens
export TELEGRAM_BOT_TOKEN=...
export DISCORD_BOT_TOKEN=...
export SLACK_BOT_TOKEN=...

# Configuration
export ZEROCLAW_CONFIG_PATH=./config.toml
```

## Useful Links

- **Repository:** `c:/Users/rajesh/zeroclaw`
- **Documentation:** `docs/book/src/`
- **API Docs:** Run `cargo doc --open`
- **Tests:** `tests/`
- **Examples:** Look in implementation crates

## Quick Tips

1. **Read before write** - Always read existing code before modifying
2. **Use the right tool** - Choose appropriate tools for the task
3. **Follow patterns** - Study existing implementations
4. **Test thoroughly** - Write tests for new code
5. **Document well** - Clear documentation helps everyone
6. **Ask questions** - Don't hesitate to seek clarification
7. **Iterate** - Start simple, then enhance

## Getting Help

1. Check documentation in `docs/book/src/`
2. Look at test files for examples
3. Study existing implementations
4. Review error messages carefully
5. Use debug logging to trace issues
6. Check the AGENTS.md file for rules

---

**Last Updated:** Day 7 of Learning Plan
**Version:** Based on current repository state