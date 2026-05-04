# Day 2: Configuration System & Traits

## Goals
- Understand ZeroClaw's configuration system
- Learn how the trait system works
- Explore the Configurable derive macro
- Create your first configuration file

## Morning Session (2-3 hours): Configuration Deep Dive

### 1. Configuration Architecture

ZeroClaw uses a layered configuration system:

```
User Config (TOML)
       ↓
Config Schema (Rust structs)
       ↓
Validation & Merging
       ↓
Runtime Configuration
```

### 2. Key Configuration Files

Explore these files:

1. **`crates/zeroclaw-config/src/lib.rs`** (15 min)
   - Main configuration module
   - See how configs are loaded

2. **`crates/zeroclaw-config/src/schema.rs`** (20 min)
   - Configuration schema definitions
   - All available config options

3. **`crates/zeroclaw-macros/src/lib.rs`** (10 min)
   - The `Configurable` derive macro
   - How it generates config code

### 3. Configuration Loading Process

```rust
// Typical flow:
1. Load default config
2. Load user config from file
3. Merge configs (user overrides defaults)
4. Validate configuration
5. Initialize components with config
```

### Reading Exercise: Configuration Schema

Open `crates/zeroclaw-config/src/schema.rs` and identify:

- Main configuration struct
- Provider configuration options
- Channel configuration options
- Memory configuration options
- Security configuration options

Create a mind map in your notes showing the config hierarchy.

## Afternoon Session (2-3 hours): Trait System Deep Dive

### 1. Core Traits Overview

ZeroClaw has 6 core extension traits:

#### Provider Trait
**Location**: `crates/zeroclaw-api/src/provider.rs`

```rust
// Simplified view
pub trait Provider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    async fn stream(&self, request: CompletionRequest) -> Result<Stream>;
    // ... other methods
}
```

**Purpose**: Abstract LLM providers (OpenAI, Anthropic, Gemini, etc.)

#### Channel Trait
**Location**: `crates/zeroclaw-api/src/channel.rs`

```rust
// Simplified view
pub trait Channel {
    async fn send(&self, message: Message) -> Result<()>;
    async fn receive(&self) -> Result<Message>;
    // ... other methods
}
```

**Purpose**: Abstract messaging platforms (Telegram, Discord, Slack, etc.)

#### Tool Trait
**Location**: `crates/zeroclaw-api/src/tool.rs`

```rust
// Simplified view
pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: ToolArgs) -> Result<ToolResult>;
}
```

**Purpose**: Define executable tools (file operations, shell commands, etc.)

#### Memory Trait
**Location**: `crates/zeroclaw-api/src/memory_traits.rs`

```rust
// Simplified view
pub trait Memory {
    async fn store(&self, entry: MemoryEntry) -> Result<()>;
    async fn retrieve(&self, query: Query) -> Result<Vec<MemoryEntry>>;
    // ... other methods
}
```

**Purpose**: Abstract memory backends (SQLite, Markdown, Vector DB, etc.)

#### Observer Trait
**Location**: `crates/zeroclaw-api/src/observability_traits.rs`

```rust
// Simplified view
pub trait Observer {
    fn on_event(&self, event: Event);
    fn on_metric(&self, metric: Metric);
    // ... other methods
}
```

**Purpose**: Observability and monitoring

#### Peripheral Trait
**Location**: `crates/zeroclaw-api/src/peripherals_traits.rs`

```rust
// Simplified view
pub trait Peripheral {
    async fn read(&self) -> Result<Data>;
    async fn write(&self, data: Data) -> Result<()>;
    // ... other methods
}
```

**Purpose**: Hardware integration (GPIO, USB, Serial, etc.)

### 2. Trait Implementation Pattern

Study this pattern by examining existing implementations:

```
1. Define trait in zeroclaw-api
2. Implement trait in specific crate
3. Register implementation in factory
4. Configure via TOML
5. Runtime instantiates and uses
```

### Reading Exercise: Trace an Implementation

Pick one provider implementation:

1. Open `crates/zeroclaw-providers/src/` and choose a provider (e.g., `openai.rs`)
2. Find the `Provider` trait implementation
3. Trace how it's registered in the factory
4. Find its configuration schema
5. Document the flow in your notes

## Evening Session (1-2 hours): Hands-On Configuration

### Exercise 1: Explore Default Configuration

```bash
# Generate a sample config (if command exists)
cargo run -- config init

# Or manually explore the schema
# Look at crates/zeroclaw-config/src/schema.rs
```

### Exercise 2: Create a Learning Configuration

Create: `learning-plan/exercises/my-config.toml`

```toml
# Example configuration structure
[agent]
name = "learning-agent"
personality = "helpful and educational"

[provider]
type = "openai"  # or another provider
# Add other provider settings

[memory]
type = "markdown"
path = "./learning-plan/memory"

[security]
# Security settings
```

### Exercise 3: Configuration Validation

Study how configuration is validated:

1. Open `crates/zeroclaw-config/src/lib.rs`
2. Find validation functions
3. Understand error handling
4. List validation rules in your notes

### Exercise 4: Macro Exploration

1. Open `crates/zeroclaw-macros/src/lib.rs`
2. Understand what `#[derive(Configurable)]` generates
3. Find examples of structs using this macro
4. Document the generated code pattern

## Deep Dive: Configuration Merging

### How Configs Merge

```
Default Config (hardcoded)
    ↓
System Config (/etc/zeroclaw/config.toml)
    ↓
User Config (~/.config/zeroclaw/config.toml)
    ↓
Project Config (./zeroclaw.toml)
    ↓
Environment Variables (ZEROCLAW_*)
    ↓
CLI Arguments (--flag value)
```

Later sources override earlier ones.

### Reading Exercise

Find the merging logic:
1. Open `crates/zeroclaw-config/src/lib.rs`
2. Search for merge functions
3. Understand the precedence rules
4. Document in your notes

## Checkpoint Questions

Before moving to Day 3, ensure you can answer:

- [ ] What are the 6 core traits in ZeroClaw?
- [ ] Where is the Provider trait defined?
- [ ] What does the Configurable macro do?
- [ ] How does configuration merging work?
- [ ] What is the stability tier of zeroclaw-config?
- [ ] Where are provider implementations located?
- [ ] How do you validate a configuration?
- [ ] What file format is used for configuration?

## Exercises Summary

Create in `learning-plan/exercises/day-2/`:

1. **`trait-comparison.md`** - Compare all 6 core traits
2. **`config-schema-map.md`** - Mind map of configuration structure
3. **`implementation-trace.md`** - Trace one provider implementation
4. **`my-config.toml`** - Your learning configuration

## Resources

- **Config Crate**: `crates/zeroclaw-config/`
- **API Traits**: `crates/zeroclaw-api/src/`
- **Macros**: `crates/zeroclaw-macros/`
- **Provider Implementations**: `crates/zeroclaw-providers/src/`
- **Documentation**: `docs/book/src/developing/extension-examples.md`

## Key Files to Study

1. `crates/zeroclaw-api/src/provider.rs` - Provider trait
2. `crates/zeroclaw-api/src/channel.rs` - Channel trait
3. `crates/zeroclaw-api/src/tool.rs` - Tool trait
4. `crates/zeroclaw-api/src/memory_traits.rs` - Memory trait
5. `crates/zeroclaw-config/src/schema.rs` - Config schema
6. `crates/zeroclaw-macros/src/lib.rs` - Configurable macro

## Next Steps

Once you've completed Day 2, proceed to:
**[Day 3: Providers & Memory Systems](./day-3-providers-memory.md)**

---

**Time Estimate**: 5-7 hours total
**Difficulty**: ⭐⭐⭐☆☆ (Intermediate)