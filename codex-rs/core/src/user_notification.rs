use std::path::PathBuf;

use serde::Serialize;

/// User can configure a program that will receive notifications. Each
/// notification is serialized as JSON and passed as an argument to the
/// program.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum UserNotification {
    #[serde(rename_all = "kebab-case")]
    AgentTurnComplete {
        turn_id: String,

        /// Messages that the user sent to the agent to initiate the turn.
        input_messages: Vec<String>,

        /// The last message sent by the assistant in the turn.
        last_assistant_message: Option<String>,
    },
    #[serde(rename_all = "kebab-case")]
    ExecApprovalRequested {
        command: Vec<String>,
        cwd: PathBuf,
        reason: Option<String>,
    },
    #[serde(rename_all = "kebab-case")]
    ApplyPatchApprovalRequested {
        changes_count: usize,
        reason: Option<String>,
        grant_root: Option<PathBuf>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_agent_turn_complete_notification() {
        let notification = UserNotification::AgentTurnComplete {
            turn_id: "12345".to_string(),
            input_messages: vec!["Rename `foo` to `bar` and update the callsites.".to_string()],
            last_assistant_message: Some(
                "Rename complete and verified `cargo build` succeeds.".to_string(),
            ),
        };
        let serialized = serde_json::to_string(&notification).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"agent-turn-complete","turn-id":"12345","input-messages":["Rename `foo` to `bar` and update the callsites."],"last-assistant-message":"Rename complete and verified `cargo build` succeeds."}"#
        );
    }

    #[test]
    fn test_exec_approval_requested_notification() {
        let notification = UserNotification::ExecApprovalRequested {
            command: vec!["cargo".to_string(), "test".to_string()],
            cwd: PathBuf::from("/tmp/codex"),
            reason: Some("Need approval to run tests".to_string()),
        };
        let serialized = serde_json::to_string(&notification).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"exec-approval-requested","command":["cargo","test"],"cwd":"/tmp/codex","reason":"Need approval to run tests"}"#
        );
    }

    #[test]
    fn test_apply_patch_approval_requested_notification() {
        let notification = UserNotification::ApplyPatchApprovalRequested {
            changes_count: 3,
            reason: None,
            grant_root: Some(PathBuf::from("/home/user/project")),
        };
        let serialized = serde_json::to_string(&notification).unwrap();
        assert_eq!(
            serialized,
            r#"{"type":"apply-patch-approval-requested","changes-count":3,"reason":null,"grant-root":"/home/user/project"}"#
        );
    }
}
