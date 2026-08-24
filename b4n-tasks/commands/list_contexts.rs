use b4n_kube::client::list_contexts;
use tracing::error;

use crate::commands::CommandResult;

/// Command that reads kubeconfig file and lists all contexts from it.
pub struct ListKubeContextsCommand {
    pub kubeconfig_path: Option<String>,
}

impl ListKubeContextsCommand {
    /// Gets all contexts from the kubeconfig file.
    pub async fn execute(&self) -> Option<CommandResult> {
        match list_contexts(self.kubeconfig_path.as_deref()).await {
            Ok(contexts) => Some(CommandResult::ContextsList(contexts)),
            Err(error) => {
                error!("Cannot read contexts list: {}", error);
                None
            },
        }
    }
}
