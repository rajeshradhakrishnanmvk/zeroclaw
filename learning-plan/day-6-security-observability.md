# Day 6: Security & Observability

## Goals
- Master security architecture and policies
- Understand observability infrastructure
- Learn monitoring and debugging techniques
- Explore gateway and infrastructure components

## Morning Session (2-3 hours): Security Deep Dive

### 1. Security Architecture

```
┌─────────────────────────────────────────────────┐
│              Security Layer                     │
│                                                 │
│  ┌──────────────┐  ┌──────────────┐           │
│  │   Access     │  │  Sandboxing  │           │
│  │   Control    │  │   & Limits   │           │
│  └──────┬───────┘  └──────┬───────┘           │
│         │                  │                    │
│  ┌──────▼──────────────────▼───────┐           │
│  │      Security Policies          │           │
│  │  - File access rules            │           │
│  │  - Network restrictions         │           │
│  │  - Resource limits              │           │
│  │  - Command whitelist            │           │
│  └─────────────────────────────────┘           │
│                                                 │
│  ┌─────────────────────────────────┐           │
│  │      Audit & Compliance         │           │
│  │  - Action logging               │           │
│  │  - Security events              │           │
│  │  - Compliance tracking          │           │
│  └─────────────────────────────────┘           │
└─────────────────────────────────────────────────┘
```

### 2. Security Module Exploration

**Directory**: `crates/zeroclaw-runtime/src/security/` or `src/security/`

Study these security components:

#### Access Control System
Study permission management:
- Permission definitions
- Role-based access control (RBAC)
- Resource-level permissions
- Dynamic permission checks
- Permission inheritance

#### Sandboxing Mechanisms
Study execution isolation:
- Process isolation
- File system restrictions
- Network isolation
- Resource quotas
- Capability-based security

#### Security Policies
Study policy enforcement:
- Policy definition format
- Policy evaluation
- Policy composition
- Default policies
- Custom policies

### 3. Reading Exercise: Security Boundaries

Trace security enforcement:

1. **Tool Execution Security**
   - Open tool execution code
   - Find security checks
   - Understand permission validation
   - Document enforcement points

2. **File Access Security**
   - Find file operation code
   - Identify access checks
   - Understand path validation
   - Document restrictions

3. **Network Security**
   - Find network operation code
   - Identify connection restrictions
   - Understand URL filtering
   - Document policies

4. **Command Execution Security**
   - Find shell execution code
   - Identify command validation
   - Understand whitelist/blacklist
   - Document restrictions

### 4. Security Configuration

Study security configuration in config schema:

```toml
[security]
# Access control
enable_rbac = true
default_role = "user"

# Sandboxing
enable_sandbox = true
max_memory_mb = 512
max_cpu_percent = 50
max_execution_time_seconds = 30

# File access
allowed_paths = ["/home/user/workspace"]
denied_paths = ["/etc", "/sys"]
max_file_size_mb = 10

# Network access
allow_network = true
allowed_domains = ["api.example.com"]
denied_domains = ["malicious.com"]

# Command execution
allow_shell = true
allowed_commands = ["ls", "cat", "grep"]
denied_commands = ["rm", "dd", "mkfs"]

# Audit
enable_audit_log = true
audit_log_path = "./audit.log"
log_all_actions = true
```

### 5. Threat Model Analysis

Study the threat model:

1. **Threats Considered**
   - Malicious tool execution
   - Unauthorized file access
   - Data exfiltration
   - Resource exhaustion
   - Privilege escalation

2. **Mitigations**
   - Sandboxing
   - Access control
   - Resource limits
   - Audit logging
   - Input validation

Document the threat model in your notes.

## Afternoon Session (2-3 hours): Observability Infrastructure

### 1. Observability Architecture

```
┌─────────────────────────────────────────────────┐
│          Observability Layer                    │
│                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │ Logging  │  │ Metrics  │  │ Tracing  │     │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘     │
│       │             │              │            │
│  ┌────▼─────────────▼──────────────▼─────┐     │
│  │      Observer Trait (API)             │     │
│  └───────────────────┬───────────────────┘     │
│                      │                          │
│  ┌───────────────────▼───────────────────┐     │
│  │    Observer Implementations           │     │
│  │  - Console Observer                   │     │
│  │  - File Observer                      │     │
│  │  - Metrics Collector                  │     │
│  │  - Distributed Tracing                │     │
│  └───────────────────────────────────────┘     │
└─────────────────────────────────────────────────┘
```

### 2. Observer Trait Deep Dive

**File**: `crates/zeroclaw-api/src/observability_traits.rs`

Study:
- `Observer` trait definition
- Event types
- Metric types
- Trace context
- Integration points

### 3. Observability Implementations

**Directory**: `crates/zeroclaw-runtime/src/observability/` or `src/observability/`

Explore observability components:

#### Logging System
Study structured logging:
- Log levels (trace, debug, info, warn, error)
- Structured fields
- Context propagation
- Log formatting
- Log rotation

#### Metrics Collection
Study metrics:
- Counter metrics
- Gauge metrics
- Histogram metrics
- Custom metrics
- Metrics export

#### Distributed Tracing
Study tracing:
- Span creation
- Span context
- Trace propagation
- Performance profiling
- Trace visualization

### 4. Reading Exercise: Observability Integration

Trace observability through the system:

1. **Agent Loop Observability**
   - Open agent loop code
   - Find logging statements
   - Find metric collection
   - Find trace spans
   - Document instrumentation

2. **Tool Execution Observability**
   - Find tool execution code
   - Identify logged events
   - Identify collected metrics
   - Document observability

3. **Channel Observability**
   - Find channel code
   - Identify message logging
   - Identify performance metrics
   - Document observability

### 5. Observability Configuration

```toml
[observability]
# Logging
log_level = "info"
log_format = "json"  # or "text"
log_file = "./zeroclaw.log"
log_rotation = "daily"
max_log_size_mb = 100

# Metrics
enable_metrics = true
metrics_port = 9090
metrics_path = "/metrics"

# Tracing
enable_tracing = true
tracing_endpoint = "http://localhost:4317"
sample_rate = 0.1  # 10% sampling

# Observers
observers = ["console", "file", "metrics"]
```

## Evening Session (1-2 hours): Gateway & Infrastructure

### 1. Gateway Architecture

**Directory**: `crates/zeroclaw-gateway/` or `src/gateway/`

Study the gateway server:

#### Webhook Server
- HTTP server setup
- Webhook endpoints
- Request validation
- Signature verification
- Rate limiting

#### API Server
- REST API endpoints
- Authentication
- Authorization
- Request handling
- Response formatting

#### WebSocket Server
- Real-time connections
- Message broadcasting
- Connection management
- Heartbeat/keepalive

### 2. Infrastructure Components

**Directory**: `crates/zeroclaw-infra/` or `src/infra/`

Study shared infrastructure:

#### Debounce System
**File**: Look for debounce implementation

- Duplicate detection
- Time-based debouncing
- Content-based debouncing
- State management

#### Session Management
**Files**: Session-related files

- Session storage
- Session lifecycle
- Session expiration
- Session cleanup

#### Stall Watchdog
**File**: Watchdog implementation

- Timeout detection
- Stall recovery
- Health monitoring
- Alert generation

### 3. Reading Exercise: Gateway Flow

Trace a webhook request:

1. Webhook received
2. Signature verified
3. Request parsed
4. Routed to channel
5. Processed by agent
6. Response generated
7. Response sent

Document each step with security checks.

## Exercises

### Exercise 1: Security Policy Design

Create: `learning-plan/exercises/day-6/security-policy.toml`

Design a comprehensive security policy for:
- Development environment
- Testing environment
- Production environment

Include all security settings.

### Exercise 2: Threat Model Document

Create: `learning-plan/exercises/day-6/threat-model.md`

Document:
- Identified threats
- Attack vectors
- Mitigations
- Residual risks
- Security recommendations

### Exercise 3: Observability Dashboard Design

Create: `learning-plan/exercises/day-6/observability-dashboard.md`

Design a monitoring dashboard showing:
- Key metrics to track
- Important logs to monitor
- Traces to analyze
- Alerts to configure
- Visualization ideas

### Exercise 4: Security Audit Checklist

Create: `learning-plan/exercises/day-6/security-audit.md`

Create a checklist for:
- Configuration review
- Permission audit
- Access log review
- Security event analysis
- Compliance verification

### Exercise 5: Gateway Configuration

Create: `learning-plan/exercises/day-6/gateway-config.toml`

Configure the gateway for:
- Multiple webhook endpoints
- API authentication
- Rate limiting
- SSL/TLS settings
- CORS policies

## Deep Dive: Audit Logging

### Audit Log Format

Study audit logging:
- What events are logged
- Log format and structure
- Sensitive data handling
- Log retention
- Log analysis

### Reading Exercise

Find and document:
1. Audit log generation
2. Audit log storage
3. Audit log rotation
4. Audit log analysis tools
5. Compliance requirements

## Deep Dive: Performance Monitoring

### Performance Metrics

Study performance monitoring:

1. **Latency Metrics**
   - Request processing time
   - Tool execution time
   - Provider response time
   - End-to-end latency

2. **Throughput Metrics**
   - Messages per second
   - Tool calls per minute
   - API requests per hour

3. **Resource Metrics**
   - CPU usage
   - Memory usage
   - Disk I/O
   - Network I/O

### Reading Exercise

Find and document:
1. Where metrics are collected
2. How metrics are aggregated
3. How metrics are exported
4. How to query metrics

## Checkpoint Questions

Before moving to Day 7, ensure you can answer:

- [ ] What security mechanisms does ZeroClaw implement?
- [ ] How does sandboxing work?
- [ ] What is the Observer trait's purpose?
- [ ] What observability features are available?
- [ ] How are audit logs generated?
- [ ] What does the gateway server do?
- [ ] How are webhooks secured?
- [ ] What metrics are collected?
- [ ] How does distributed tracing work?
- [ ] What infrastructure components exist?

## Exercises Summary

Create in `learning-plan/exercises/day-6/`:

1. **`security-policy.toml`** - Comprehensive security policy
2. **`threat-model.md`** - Threat analysis document
3. **`observability-dashboard.md`** - Dashboard design
4. **`security-audit.md`** - Audit checklist
5. **`gateway-config.toml`** - Gateway configuration
6. **`monitoring-guide.md`** - Monitoring best practices

## Resources

- **Security Module**: `crates/zeroclaw-runtime/src/security/`
- **Observability API**: `crates/zeroclaw-api/src/observability_traits.rs`
- **Observability Impl**: `crates/zeroclaw-runtime/src/observability/`
- **Gateway**: `crates/zeroclaw-gateway/`
- **Infrastructure**: `crates/zeroclaw-infra/`
- **Security Docs**: `docs/book/src/security/`

## Key Concepts

1. **Defense in Depth**: Multiple security layers
2. **Least Privilege**: Minimal permissions by default
3. **Audit Trail**: Complete action logging
4. **Observability**: Comprehensive monitoring
5. **Structured Logging**: Machine-readable logs
6. **Distributed Tracing**: Request flow tracking
7. **Metrics Collection**: Performance monitoring
8. **Gateway Pattern**: Centralized entry point

## Testing Security & Observability

```bash
# Run security tests
cargo test security

# Run observability tests
cargo test observability

# Run gateway tests
cargo test --package zeroclaw-gateway

# Run with debug logging
RUST_LOG=debug cargo run

# Run with tracing
RUST_LOG=trace cargo run
```

## Next Steps

Once you've completed Day 6, proceed to:
**[Day 7: Integration & Extension](./day-7-integration-extension.md)**

---

**Time Estimate**: 5-7 hours total
**Difficulty**: ⭐⭐⭐⭐⭐ (Expert)