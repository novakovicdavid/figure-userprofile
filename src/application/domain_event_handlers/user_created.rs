use std::sync::Arc;

use figure_lib::queue::internal_event_router::{RouterError, State};
use tracing::info;

use crate::application::domain_event_dispatcher::PasswordResetRequested;
use crate::application::state::DomainEventHandlerState;

// todo save to some kind of audit table/system
pub async fn password_reset_requested(State(state): State<Arc<DomainEventHandlerState>>, event: PasswordResetRequested) -> Result<(), RouterError> {
    info!("{}", event.requester);
    Ok(())
}