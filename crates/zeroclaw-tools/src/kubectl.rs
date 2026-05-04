use async_trait::async_trait;
use serde_json::json;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use anyhow::Result;

use zeroclaw_api::tool::{Tool, ToolResult};

/// Simple kubectl execution tool. Performs basic validation before executing
/// `kubectl` as a subprocess and returns captured stdout/stderr.
pub struct KubectlTool {
    pub security: Arc<zeroclaw_config::policy::SecurityPolicy>,
    pub default_kubeconfig: Option<PathBuf>,
    pub default_namespace: Option<String>,
    pub timeout_secs: u64,
}

impl KubectlTool {
    pub fn new(
        security: Arc<zeroclaw_config::policy::SecurityPolicy>,
        default_kubeconfig: Option<PathBuf>,
        default_namespace: Option<String>,
        timeout_secs: u64,
    ) -> Self {
        Self {
            security,
            default_kubeconfig,
            default_namespace,
            timeout_secs,
        }
    }

    fn build_args(&self, subcommand: &str, args: &[String], namespace: &Option<String>, kubeconfig: &Option<String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        out.push(subcommand.to_string());
        out.extend_from_slice(args);
        if let Some(ns) = namespace {
            if !ns.is_empty() {
                out.push("--namespace".to_string());
                out.push(ns.clone());
            }
        } else if let Some(ns) = &self.default_namespace {
            out.push("--namespace".to_string());
            out.push(ns.clone());
        }
        if let Some(kc) = kubeconfig {
            if !kc.is_empty() {
                out.push("--kubeconfig".to_string());
                out.push(kc.clone());
            }
        } else if let Some(kc) = &self.default_kubeconfig {
            out.push("--kubeconfig".to_string());
            out.push(kc.to_string_lossy().into_owned());
        }
        out
    }
}

#[async_trait]
impl Tool for KubectlTool {
    fn name(&self) -> &str {
        "kubectl_command"
    }

    fn description(&self) -> &str {
        "Executes kubectl commands to manage Kubernetes resources"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "kubectl subcommand (e.g., get, describe, apply)" },
                "args": { "type": "array", "items": { "type": "string" }, "description": "Arguments for the subcommand" },
                "namespace": { "type": "string", "description": "Kubernetes namespace (optional)" },
                "kubeconfig": { "type": "string", "description": "Path to kubeconfig (optional)" },
                "timeout_secs": { "type": "integer", "description": "Override timeout in seconds (optional)" }
            },
            "required": ["command"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        // Parse parameters
        let command = match args.get("command").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => {
                return Ok(ToolResult {
                    success: false,
                    output: "".to_string(),
                    error: Some("missing required parameter: command".to_string()),
                })
            }
        };

        let args_vec: Vec<String> = if let Some(arr) = args.get("args") {
            if arr.is_array() {
                arr.as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let namespace = args.get("namespace").and_then(|v| v.as_str()).map(|s| s.to_string());
        let kubeconfig = args.get("kubeconfig").and_then(|v| v.as_str()).map(|s| s.to_string());
        let timeout_override = args.get("timeout_secs").and_then(|v| v.as_u64());

        // Build arg list
        let cmd_args = self.build_args(&command, &args_vec, &namespace, &kubeconfig);

        // Security check: validate kubectl invocation against runtime-loaded policy
        if let Err(reason) = self.security.validate_kubectl_call(&command, &args_vec, namespace.as_deref(), false) {
            return Ok(ToolResult {
                success: false,
                output: "".to_string(),
                error: Some(format!("kubectl blocked by security policy: {}", reason)),
            });
        }

        // Execute with timeout
        let effective_timeout = timeout_override.unwrap_or(self.timeout_secs);
        let dur = Duration::from_secs(effective_timeout);

        let mut cmd = Command::new("kubectl");
        cmd.args(&cmd_args);

        let output_res = timeout(dur, cmd.output()).await;

        match output_res {
            Err(_) => Ok(ToolResult {
                success: false,
                output: "".to_string(),
                error: Some("kubectl execution timed out".to_string()),
            }),
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let combined = if stderr.is_empty() {
                    stdout.clone()
                } else if stdout.is_empty() {
                    stderr.clone()
                } else {
                    format!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout, stderr)
                };

                Ok(ToolResult {
                    success: output.status.success(),
                    output: combined,
                    error: if output.status.success() { None } else { Some(format!("exit code: {:?}", output.status.code())) },
                })
            }
            Ok(Err(e)) => Ok(ToolResult {
                success: false,
                output: "".to_string(),
                error: Some(format!("failed to spawn kubectl: {}", e)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn params_schema_contains_command() {
        let sec = Arc::new(zeroclaw_config::policy::SecurityPolicy::default());
        let t = KubectlTool::new(sec, None, None, 15);
        let schema = t.parameters_schema();
        assert!(schema.get("properties").is_some());
        assert!(schema["properties"].get("command").is_some());
    }

    #[tokio::test]
    async fn blocks_disallowed_commands_by_default() {
        let sec = Arc::new(zeroclaw_config::policy::SecurityPolicy::default());
        let t = KubectlTool::new(sec, None, None, 1);
        // attempt a command with a disallowed pattern (e.g. shell expansion)
        let args = json!({"command":"get","args":["pods","|","rm","-rf","/"]});
        let res = t.execute(args).await.unwrap();
        // Policy should block weird constructs; result should be failure
        assert!(!res.success);
    }
}
