use crate::connection::manager::ConnectionManager;
use tokio_util::sync::CancellationToken;

pub struct AppState {
    pub connection_manager: ConnectionManager,
    pub cancel_token: CancellationToken,
}

impl AppState {
    pub fn new() -> Self {
        let cancel_token = CancellationToken::new();
        Self {
            connection_manager: ConnectionManager::new(cancel_token.clone()),
            cancel_token,
        }
    }

    pub fn shutdown(&self) {
        self.cancel_token.cancel();
    }
}
