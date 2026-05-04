# Day 3: Providers & Memory Systems

## Goals
- Understand how LLM providers work in ZeroClaw
- Learn the memory system architecture
- Explore different memory backends
- Understand the resilient provider wrapper

## Morning Session (2-3 hours): Provider System

### 1. Provider Architecture

```
┌─────────────────────────────────────────┐
│         Provider Trait (API)            │
└────────────────┬────────────────────────┘
                 │
    ┌────────────┴────────────┐
    │                         │
┌───▼──────────┐    ┌────────▼─────────┐
│   Specific   │    │    Resilient     │
│  Providers   │───▶│     Wrapper      │
│ (OpenAI,     │    │  (retry, cache,  │
│  Anthropic,  │    │   fallback)      │
│  Gemini...)  │    │                  │
└──────────────┘    └──────────────────┘
```

### 2. Key Files to Study

#### Provider Trait Definition
**File**: `crates/zeroclaw-api/src/provider.rs`

Read and understand:
- `Provider` trait methods
- `CompletionRequest` structure
- `CompletionResponse` structure
- Streaming vs non-streaming
- Error handling

#### Provider Implementations
**Directory**: `crates/zeroclaw-providers/src/`

Explore these implementations:
1. **`openai.rs`** - OpenAI/Azure OpenAI provider
2. **`anthropic.rs`** - Anthropic Claude provider
3. **`gemini.rs`** - Google Gemini provider
4. **`ollama.rs`** - Local Ollama provider

#### Resilient Provider Wrapper
**File**: `crates/zeroclaw-providers/src/resilient.rs` (or similar)

Understand:
- Retry logic
- Fallback mechanisms
- Rate limiting
- Caching strategies

### 3. Reading Exercise: Provider Flow

Trace a complete request flow:

1. User sends message
2. Agent runtime receives it
3. Provider is selected
4. Request is formatted
5. Resilient wrapper applies policies
6. Actual provider makes API call
7. Response is processed
8. Result returned to agent

Document each step in your notes with file references.

### 4. Provider Configuration

Study provider configuration in `crates/zeroclaw-config/src/schema.rs`:

```toml
# Example provider config structure
[provider]
type = "openai"
api_key = "${OPENAI_API_KEY}"
model = "gpt-4"
temperature = 0.7
max_tokens = 2000

# Resilient wrapper settings
[provider.resilient]
max_retries = 3
retry_delay_ms = 1000
enable_cache = true
cache_ttl_seconds = 3600
```

## Afternoon Session (2-3 hours): Memory Systems

### 1. Memory Architecture

```
┌─────────────────────────────────────────┐
│         Memory Trait (API)              │
└────────────────┬────────────────────────┘
                 │
    ┌────────────┴────────────┬────────────┐
    │                         │            │
┌───▼──────────┐    ┌────────▼─────┐  ┌──▼────────┐
│   Markdown   │    │    SQLite    │  │  Vector   │
│    Memory    │    │    Memory    │  │  Memory   │
│  (files)     │    │  (database)  │  │(embeddings)│
└──────────────┘    └──────────────┘  └───────────┘
```

### 2. Memory Trait Deep Dive

**File**: `crates/zeroclaw-api/src/memory_traits.rs`

Study:
- `Memory` trait definition
- `MemoryEntry` structure
- Query mechanisms
- Storage and retrieval patterns

### 3. Memory Implementations

**Directory**: `crates/zeroclaw-memory/src/`

Explore each backend:

#### Markdown Memory
**File**: `crates/zeroclaw-memory/src/markdown.rs` (or similar)

- Stores memories as markdown files
- Human-readable format
- Easy to version control
- Good for simple use cases

#### SQLite Memory
**File**: `crates/zeroclaw-memory/src/sqlite.rs` (or similar)

- Structured database storage
- Fast queries
- Transactions support
- Good for complex queries

#### Vector Memory
**File**: `crates/zeroclaw-memory/src/vector.rs` (or similar)

- Embedding-based storage
- Semantic search
- Similarity matching
- Good for RAG (Retrieval Augmented Generation)

### 4. Memory Operations

Study these key operations:

```rust
// Simplified examples

// 1. Store a memory
memory.store(MemoryEntry {
    content: "User prefers Python",
    timestamp: now(),
    tags: vec!["preference", "language"],
}).await?;

// 2. Retrieve memories
let results = memory.retrieve(Query {
    text: "programming language",
    limit: 5,
}).await?;

// 3. Update a memory
memory.update(entry_id, updated_entry).await?;

// 4. Delete a memory
memory.delete(entry_id).await?;
```

### 5. Reading Exercise: Memory Flow

Trace how memories are used in the agent loop:

1. Open `crates/zeroclaw-runtime/src/agent/loop_.rs`
2. Find where memories are stored
3. Find where memories are retrieved
4. Understand the context window management
5. Document the flow in your notes

## Evening Session (1-2 hours): Integration & Testing

### Exercise 1: Provider Comparison

Create: `learning-plan/exercises/day-3/provider-comparison.md`

Compare 3 providers:
- API authentication method
- Request/response format
- Streaming support
- Rate limits
- Error handling

### Exercise 2: Memory Backend Analysis

Create: `learning-plan/exercises/day-3/memory-backends.md`

For each memory backend, document:
- Storage format
- Query capabilities
- Performance characteristics
- Use cases
- Limitations

### Exercise 3: Code Reading - Resilient Wrapper

1. Open the resilient provider wrapper
2. Find the retry logic
3. Find the fallback logic
4. Find the caching logic
5. Document how they work together

### Exercise 4: Memory Testing

Explore memory tests:

```bash
# Run memory tests
cargo test --package zeroclaw-memory

# Look at test files
# tests/integration/memory_*.rs
```

Study:
- `tests/integration/memory_comparison.rs`
- `tests/integration/memory_loop_continuity.rs`
- `tests/integration/memory_restart.rs`

## Deep Dive: Provider Selection & Fallback

### How Providers are Selected

Study the provider factory pattern:

1. Configuration specifies primary provider
2. Optional fallback providers configured
3. Runtime instantiates providers
4. Resilient wrapper manages selection
5. Fallback on failure

### Reading Exercise

Find and document:
1. Where providers are registered
2. How provider factory works
3. Fallback decision logic
4. Error handling strategy

## Deep Dive: Memory Merging & Vector Search

### Vector Memory Details

If vector memory is implemented:

1. How embeddings are generated
2. Similarity search algorithm
3. Index management
4. Performance optimization

### Reading Exercise

Study `crates/zeroclaw-memory/src/` and document:
- Embedding generation process
- Vector storage format
- Search algorithm
- Merge strategies

## Checkpoint Questions

Before moving to Day 4, ensure you can answer:

- [ ] What is the Provider trait's main purpose?
- [ ] Name 3 provider implementations
- [ ] What does the resilient wrapper do?
- [ ] What are the 3 memory backends?
- [ ] How are memories stored in markdown format?
- [ ] What is vector memory used for?
- [ ] How does provider fallback work?
- [ ] Where is the agent loop located?
- [ ] How are memories retrieved during agent execution?
- [ ] What is the difference between streaming and non-streaming?

## Exercises Summary

Create in `learning-plan/exercises/day-3/`:

1. **`provider-comparison.md`** - Compare provider implementations
2. **`memory-backends.md`** - Analyze memory backends
3. **`resilient-wrapper-notes.md`** - Document resilient wrapper
4. **`memory-flow-diagram.md`** - Draw memory flow in agent loop
5. **`provider-config-examples.toml`** - Example provider configs

## Resources

- **Provider API**: `crates/zeroclaw-api/src/provider.rs`
- **Provider Implementations**: `crates/zeroclaw-providers/src/`
- **Memory API**: `crates/zeroclaw-api/src/memory_traits.rs`
- **Memory Implementations**: `crates/zeroclaw-memory/src/`
- **Agent Loop**: `crates/zeroclaw-runtime/src/agent/loop_.rs`
- **Memory Tests**: `tests/integration/memory_*.rs`

## Key Concepts

1. **Provider Abstraction**: Unified interface for different LLMs
2. **Resilient Wrapper**: Retry, fallback, caching for reliability
3. **Memory Backends**: Different storage strategies for different needs
4. **Vector Search**: Semantic similarity for memory retrieval
5. **Context Management**: How memories fit in LLM context window

## Next Steps

Once you've completed Day 3, proceed to:
**[Day 4: Channels & Communication](./day-4-channels-communication.md)**

---

**Time Estimate**: 5-7 hours total
**Difficulty**: ⭐⭐⭐⭐☆ (Advanced)