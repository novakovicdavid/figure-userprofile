use std::sync::Arc;

use figure_lib::queue::internal_event_router::{RouterError, State};

use crate::application::domain_event_dispatcher::UserCreatedDomainEvent;
use crate::state::ServerState;

pub fn user_created(State(state): State<Arc<ServerState>>, event: UserCreatedDomainEvent) -> Result<(), RouterError> {
    Ok(())
}