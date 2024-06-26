use anyhow::{Context, Result};
use gitbutler_core::{
    deltas,
    projects::ProjectId,
    reader,
    sessions::{self, SessionId},
    virtual_branches,
};
use tauri::Manager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    name: String,
    payload: serde_json::Value,
    project_id: ProjectId,
}

impl Event {
    pub fn send(&self, app_handle: &tauri::AppHandle) -> Result<()> {
        app_handle
            .emit_all(&self.name, Some(&self.payload))
            .context("emit event")?;
        tracing::trace!(event_name = self.name);
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn project_id(&self) -> ProjectId {
        self.project_id
    }

    pub fn git_index(project_id: ProjectId) -> Self {
        Event {
            name: format!("project://{}/git/index", project_id),
            payload: serde_json::json!({}),
            project_id,
        }
    }

    pub fn git_fetch(project_id: ProjectId) -> Self {
        Event {
            name: format!("project://{}/git/fetch", project_id),
            payload: serde_json::json!({}),
            project_id,
        }
    }

    pub fn git_head(project_id: ProjectId, head: &str) -> Self {
        Event {
            name: format!("project://{}/git/head", project_id),
            payload: serde_json::json!({ "head": head }),
            project_id,
        }
    }

    pub fn git_activity(project_id: ProjectId) -> Self {
        Event {
            name: format!("project://{}/git/activity", project_id),
            payload: serde_json::json!({}),
            project_id,
        }
    }

    pub fn file(
        project_id: ProjectId,
        session_id: SessionId,
        file_path: &str,
        contents: Option<&reader::Content>,
    ) -> Self {
        Event {
            name: format!("project://{}/sessions/{}/files", project_id, session_id),
            payload: serde_json::json!({
                "filePath": file_path,
                "contents": contents,
            }),
            project_id,
        }
    }

    pub fn session(project_id: ProjectId, session: &sessions::Session) -> Self {
        Event {
            name: format!("project://{}/sessions", project_id),
            payload: serde_json::to_value(session).unwrap(),
            project_id,
        }
    }

    pub fn deltas(
        project_id: ProjectId,
        session_id: SessionId,
        deltas: &[deltas::Delta],
        relative_file_path: &std::path::Path,
    ) -> Self {
        Event {
            name: format!("project://{}/sessions/{}/deltas", project_id, session_id),
            payload: serde_json::json!({
                "deltas": deltas,
                "filePath": relative_file_path,
            }),
            project_id,
        }
    }

    pub fn virtual_branches(
        project_id: ProjectId,
        virtual_branches: &virtual_branches::VirtualBranches,
    ) -> Self {
        Event {
            name: format!("project://{}/virtual-branches", project_id),
            payload: serde_json::json!(virtual_branches),
            project_id,
        }
    }
}
