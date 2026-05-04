# Day 5: Tools & Runtime

## Goals
- Understand the tool system and execution
- Learn the agent runtime architecture
- Study the agent loop implementation
- Explore security boundaries and policies

## Morning Session (2-3 hours): Tool System

### 1. Tool Architecture

```
┌─────────────────────────────────────────┐
│           Tool Trait (API)              │
└────────────────┬────────────────────────┘
                 │
    ┌────────────┴────────────────────────────┐
    │                                         │
┌───▼──────────┐                    ┌────────▼─────────┐
│     Tool     │                    │       Tool       │
│  Execution   │◄───────────────────│  Implementations │
│   Surface    │                    │  (shell, file,   │
│  (security,  │                    │   browser, etc.) │
│   sandbox)   │                    │                  │
└──────────────┘                    └──────────────────┘
```

### 2. Tool Trait Deep Dive

**File**: `crates/zeroclaw-api/src/tool.rs`

Study:
- `Tool` trait definition
- Tool metadata (name, description, parameters)
- Execution interface
- Result handling
- Error types

### 3. Tool Implementations

**Directory**: `crates/zeroclaw-tools/src/` or `src/tools/`

Explore core tools:

#### Shell Tool
Study command execution:
- Command parsing
- Environment handling
- Output capture
- Security restrictions
- Timeout handling

#### File Tools
Study file operations:
- Read file
- Write file
- List directory
- Search files
- File permissions

#### Browser Tool
Study web automation:
- Page navigation
- Element interaction
- Screenshot capture
- Content extraction
- JavaScript execution

#### Memory Tools
Study memory operations:
- Store memory
- Retrieve memory
- Search memory
- Update memory
- Delete memory

### 4. Tool Call Parsing

**Directory**: `crates/zeroclaw-tool-call-parser/`

Study:
- How LLM tool calls are parsed
- XML format parsing
- JSON format parsing
- Error recovery
- Validation

### 5. Reading Exercise: Tool Execution Flow

Trace a complete tool execution:

1. LLM generates tool call
2. Parser extracts tool name and arguments
3. Security checks applied
4. Tool located and validated
5. Tool executed with arguments
6. Result captured
7. Result formatted for LLM
8. Response sent back

Document each step with file references.

## Afternoon Session (2-3 hours): Agent Runtime

### 1. Runtime Architecture

```
┌─────────────────────────────────────────┐
│          Agent Runtime                  │
│  ┌────────────────────────────────┐    │
│  │       Agent Loop               │    │
│  │  ┌──────────────────────────┐  │    │
│  │  │  1. Receive Message      │  │    │
│  │  │  2. Load Context/Memory  │  │    │
│  │  │  3. Call Provider        │  │    │
│  │  │  4. Parse Tool Calls     │  │    │
│  │  │  5. Execute Tools        │  │    │
│  │  │  6. Store Memory         │  │    │
│  │  │  7. Send Response        │  │    │
│  │  └──────────────────────────┘  │    │
│  └────────────────────────────────┘    │
│                                         │
│  ┌────────────────────────────────┐    │
│  │    Security Layer              │    │
│  │  - Access Control              │    │
│  │  - Sandboxing                  │    │
│  │  - Rate Limiting               │    │
│  └────────────────────────────────┘    │
│                                         │
│  ┌────────────────────────────────┐    │
│  │    Observability               │    │
│  │  - Logging                     │    │
│  │  - Metrics                     │    │
│  │  - Tracing                     │    │
│  └────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

### 2. Agent Loop Deep Dive

**File**: `crates/zeroclaw-runtime/src/agent/loop_.rs`

This is the heart of ZeroClaw! Study carefully:

#### Loop Initialization
- Configuration loading
- Component initialization
- Channel setup
- Provider setup
- Memory setup

#### Message Processing
- Message reception
- Context building
- Memory retrieval
- Prompt construction

#### LLM Interaction
- Provider call
- Streaming handling
- Response parsing
- Tool call extraction

#### Tool Execution
- Tool validation
- Security checks
- Execution
- Result handling

#### Response Generation
- Result formatting
- Memory storage
- Response sending
- Error handling

### 3. Reading Exercise: Trace the Agent Loop

Open `crates/zeroclaw-runtime/src/agent/loop_.rs` and:

1. Find the main loop function
2. Identify each phase of processing
3. Note error handling at each step
4. Document state management
5. Understand how the loop continues

Create a detailed flowchart in your notes.

### 4. Runtime Components

**Directory**: `crates/zeroclaw-runtime/src/`

Explore other runtime components:

#### Security Module
**Directory**: `crates/zeroclaw-runtime/src/security/`

Study:
- Access control policies
- Sandboxing mechanisms
- Permission system
- Security boundaries
- Audit logging

#### Cron/Scheduling
Study scheduled tasks:
- Cron job management
- Backup scheduling
- Maintenance tasks
- Periodic operations

#### SOP (Standard Operating Procedures)
Study automation:
- Procedure definitions
- Trigger conditions
- Execution flow
- Error handling

#### Skills System
Study skill management:
- Skill definitions
- Skill loading
- Skill execution
- Skill composition

## Evening Session (1-2 hours): Security & Observability

### 1. Security Deep Dive

**Directory**: `crates/zeroclaw-runtime/src/security/`

Study security mechanisms:

#### Access Control
- Permission model
- Role-based access
- Resource restrictions
- Policy enforcement

#### Sandboxing
- Execution isolation
- Resource limits
- Network restrictions
- File system restrictions

#### Audit Trail
- Action logging
- Security events
- Compliance tracking

### 2. Observability Deep Dive

**Directory**: `crates/zeroclaw-runtime/src/observability/` or `src/observability/`

Study observability:

#### Logging
- Log levels
- Structured logging
- Log aggregation
- Log rotation

#### Metrics
- Performance metrics
- Usage metrics
- Error rates
- Custom metrics

#### Tracing
- Distributed tracing
- Span management
- Trace context
- Performance profiling

### 3. Reading Exercise: Security Boundaries

Study how security is enforced:

1. Find tool execution security checks
2. Find file access restrictions
3. Find network access controls
4. Find resource limits
5. Document the security model

## Exercises

### Exercise 1: Tool Implementation Analysis

Create: `learning-plan/exercises/day-5/tool-analysis.md`

For 3 different tools, document:
- Purpose and use cases
- Input parameters
- Output format
- Security considerations
- Error handling
- Example usage

### Exercise 2: Agent Loop Flowchart

Create: `learning-plan/exercises/day-5/agent-loop-flowchart.md`

Draw a detailed flowchart showing:
- All phases of the agent loop
- Decision points
- Error handling paths
- State transitions
- Component interactions

### Exercise 3: Security Policy Analysis

Create: `learning-plan/exercises/day-5/security-policies.md`

Document:
- Access control rules
- Sandboxing restrictions
- Resource limits
- Audit requirements
- Security best practices

### Exercise 4: Tool Call Examples

Create: `learning-plan/exercises/day-5/tool-call-examples.md`

Write example tool calls in:
- XML format
- JSON format
- Show parsing process
- Show validation
- Show execution

### Exercise 5: Runtime Configuration

Create: `learning-plan/exercises/day-5/runtime-config.toml`

Write a complete runtime configuration including:
- Agent settings
- Security policies
- Tool permissions
- Observability settings
- Resource limits

## Deep Dive: Tool Shared State

### Understanding Tool Context

Study how tools share state:
- Session context
- User preferences
- Conversation history
- Temporary data
- Persistent data

### Reading Exercise

Find and document:
1. How context is passed to tools
2. How tools access shared state
3. How state is persisted
4. How state is cleaned up

## Deep Dive: Error Handling & Recovery

### Error Handling Strategy

Study error handling throughout the runtime:

1. **Tool Execution Errors**
   - How are they caught?
   - How are they reported?
   - How does the agent recover?

2. **Provider Errors**
   - API failures
   - Rate limits
   - Timeouts
   - Fallback strategies

3. **Channel Errors**
   - Connection failures
   - Message send failures
   - Recovery mechanisms

### Reading Exercise

Trace error handling:
1. Find error types defined
2. Find error handling code
3. Find recovery mechanisms
4. Document the strategy

## Checkpoint Questions

Before moving to Day 6, ensure you can answer:

- [ ] What is the Tool trait's main purpose?
- [ ] Name 5 core tools in ZeroClaw
- [ ] What are the phases of the agent loop?
- [ ] How are tool calls parsed from LLM output?
- [ ] What security mechanisms are in place?
- [ ] How does sandboxing work?
- [ ] What observability features exist?
- [ ] How does error recovery work?
- [ ] What is the tool execution flow?
- [ ] How is the agent loop structured?

## Exercises Summary

Create in `learning-plan/exercises/day-5/`:

1. **`tool-analysis.md`** - Detailed tool analysis
2. **`agent-loop-flowchart.md`** - Complete loop flowchart
3. **`security-policies.md`** - Security documentation
4. **`tool-call-examples.md`** - Tool call examples
5. **`runtime-config.toml`** - Runtime configuration
6. **`error-handling-notes.md`** - Error handling strategy

## Resources

- **Tool API**: `crates/zeroclaw-api/src/tool.rs`
- **Tool Implementations**: `crates/zeroclaw-tools/src/`
- **Tool Parser**: `crates/zeroclaw-tool-call-parser/`
- **Agent Loop**: `crates/zeroclaw-runtime/src/agent/loop_.rs`
- **Security**: `crates/zeroclaw-runtime/src/security/`
- **Observability**: `crates/zeroclaw-runtime/src/observability/`
- **Tests**: `tests/integration/agent*.rs`

## Key Concepts

1. **Tool Abstraction**: Unified interface for agent capabilities
2. **Agent Loop**: Core processing cycle
3. **Security Boundaries**: Protecting system resources
4. **Sandboxing**: Isolated execution environment
5. **Observability**: Monitoring and debugging
6. **Error Recovery**: Graceful failure handling
7. **Tool Call Parsing**: Extracting structured calls from LLM output

## Testing Runtime

```bash
# Run runtime tests
cargo test --package zeroclaw-runtime

# Run agent tests
cargo test agent

# Run tool tests
cargo test --package zeroclaw-tools

# Run integration tests
cargo test --test test_integration agent
```

## Next Steps

Once you've completed Day 5, proceed to:
**[Day 6: Security & Observability](./day-6-security-observability.md)**

---

**Time Estimate**: 5-7 hours total
**Difficulty**: ⭐⭐⭐⭐⭐ (Expert)