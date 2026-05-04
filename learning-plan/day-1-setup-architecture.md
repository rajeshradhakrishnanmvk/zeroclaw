# Day 1: Project Setup & Architecture Overview

## Goals
- Set up your development environment
- Understand the project structure
- Learn the core architectural principles
- Run your first ZeroClaw command

## Morning Session (2-3 hours): Environment Setup

### 1. Prerequisites Check

Ensure you have installed:
- Rust (latest stable): `rustc --version`
- Cargo: `cargo --version`
- Git: `git --version`

### 2. Build the Project

```bash
# From the repository root (c:/Users/rajesh/zeroclaw)
cargo build
```

This will take some time on first build. While it compiles, continue reading!

### 3. Run Basic Commands

```bash
# Check if build succeeded
cargo run -- --help

# View version
cargo run -- --version

# Run tests (optional, takes longer)
cargo test --lib
```

### 4. Explore the Repository Structure

Open these files in your editor and skim through them:

1. **`README.md`** - Project overview
2. **`AGENTS.md`** (if exists) or **`.claude/AGENTS.md`** - Agent rules and architecture
3. **`docs/book/src/`** - Documentation structure

## Afternoon Session (2-3 hours): Architecture Deep Dive

### Core Architecture Principles

ZeroClaw follows a **microkernel architecture** with these key principles:

1. **Trait-Driven Design**: Everything is an interface (Provider, Channel, Tool, Memory)
2. **Modular & Extensible**: Add new capabilities by implementing traits
3. **Stability Tiers**: Components have different stability guarantees
4. **Security-First**: Built-in security boundaries and access control

### Key Architectural Components

```
┌─────────────────────────────────────────────────────────┐
│                    CLI Entry Point                       │
│                   (src/main.rs)                          │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
┌───────▼────────┐      ┌────────▼─────────┐
│  Configuration │      │   Agent Runtime   │
│   (zeroclaw-   │      │  (zeroclaw-       │
│    config)     │      │   runtime)        │
└───────┬────────┘      └────────┬──────────┘
        │                        │
        │         ┌──────────────┴──────────────┐
        │         │                             │
┌───────▼─────────▼──────┐           ┌─────────▼──────────┐
│   Core Traits (API)    │           │   Infrastructure   │
│  - Provider            │           │   - Memory         │
│  - Channel             │           │   - Observability  │
│  - Tool                │           │   - Security       │
│  - Memory              │           │   - Gateway        │
│  - Observer            │           │                    │
│  - Peripheral          │           │                    │
└────────┬───────────────┘           └────────────────────┘
         │
    ┌────┴─────┬──────────┬──────────┐
    │          │          │          │
┌───▼────┐ ┌──▼─────┐ ┌──▼─────┐ ┌──▼────────┐
│Provider│ │Channel │ │ Tools  │ │ Peripherals│
│ Impls  │ │ Impls  │ │ Impls  │ │   Impls    │
└────────┘ └────────┘ └────────┘ └───────────┘
```

### Stability Tiers Explained

| Tier | Meaning | Examples |
|------|---------|----------|
| **Stable** | Breaking changes require major version bump | (None yet, targeting v1.0.0) |
| **Beta** | Breaking changes allowed in MINOR versions | `zeroclaw-config`, `zeroclaw-macros` |
| **Experimental** | No stability guarantee | `zeroclaw-runtime`, `zeroclaw-channels` |

### Reading Exercise: Explore Key Files

Read these files to understand the architecture:

1. **`crates/zeroclaw-api/src/lib.rs`** (5 min)
   - Entry point for all trait definitions
   - See what traits are exported

2. **`crates/zeroclaw-api/src/provider.rs`** (10 min)
   - Understand the Provider trait
   - See how LLM providers are abstracted

3. **`crates/zeroclaw-api/src/channel.rs`** (10 min)
   - Understand the Channel trait
   - See how messaging platforms are abstracted

4. **`crates/zeroclaw-api/src/tool.rs`** (10 min)
   - Understand the Tool trait
   - See how tools are defined and executed

5. **`src/main.rs`** (15 min)
   - See the CLI entry point
   - Understand command routing

## Evening Session (1-2 hours): Documentation & Notes

### 1. Read Core Documentation

Navigate to `docs/book/src/` and read:

1. **`foundations/`** - Core concepts and RFCs
2. **`developing/extension-examples.md`** - How to extend the system
3. **`contributing/privacy.md`** - Privacy rules (important!)

### 2. Create Your Learning Notes

Create a file: `learning-plan/notes/day-1-notes.md`

Answer these questions:
- What are the 6 core traits in ZeroClaw?
- What is the difference between Beta and Experimental stability tiers?
- What is the purpose of the microkernel architecture?
- Which crate contains the agent runtime loop?

### 3. Explore the Crate Structure

Run this command to see all workspace crates:

```bash
cargo tree --depth 1
```

List the crates and their purposes in your notes.

## Exercises

### Exercise 1: Build and Run
```bash
# Build in release mode
cargo build --release

# Run help command
cargo run --release -- --help

# Explore available commands
cargo run --release -- config --help
```

### Exercise 2: Code Navigation
Using your editor's "Go to Definition" feature:
1. Open `src/main.rs`
2. Find the `main()` function
3. Navigate to the CLI command definitions
4. Explore how commands are routed

### Exercise 3: Trait Exploration
1. Open `crates/zeroclaw-api/src/provider.rs`
2. Find the `Provider` trait
3. List all methods in the trait
4. Find one implementation of this trait in `crates/zeroclaw-providers/`

## Checkpoint Questions

Before moving to Day 2, ensure you can answer:

- [ ] What is ZeroClaw's primary purpose?
- [ ] What are the 6 core extension traits?
- [ ] What is the stability tier of `zeroclaw-runtime`?
- [ ] Where is the CLI entry point located?
- [ ] What is the difference between `zeroclaw-api` and `zeroclaw-runtime`?
- [ ] Can you successfully build the project?
- [ ] Have you read the privacy guidelines?

## Resources

- **Repository Root**: `c:/Users/rajesh/zeroclaw`
- **API Traits**: `crates/zeroclaw-api/src/`
- **Documentation**: `docs/book/src/`
- **Main Entry**: `src/main.rs`

## Next Steps

Once you've completed Day 1, proceed to:
**[Day 2: Configuration System & Traits](./day-2-config-traits.md)**

---

**Time Estimate**: 5-7 hours total
**Difficulty**: ⭐⭐☆☆☆ (Beginner-friendly)