use async_trait::async_trait;
use figure_lib::queue::internal_event_router::{Context, FromContext, StateTrait};
use strum_macros::EnumDiscriminants;
use time::OffsetDateTime;

#[derive(Clone, Eq, Hash, PartialEq, EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum DomainEvent {
    PasswordResetRequested(PasswordResetRequested),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PasswordResetRequested {
    pub token: String,
    pub email: String,
    pub requester: String,
    pub datetime: OffsetDateTime
}

#[async_trait]
impl<S: StateTrait> FromContext<DomainEventDiscriminants, DomainEvent, S> for PasswordResetRequested
{
    async fn from_context(ctx: &Context<DomainEvent, S>) -> Self {
        match &ctx.event {
            DomainEvent::PasswordResetRequested(event) => event.clone(),
            _ => unreachable!()
        }
    }

    fn topic() -> Option<DomainEventDiscriminants> {
        Some(DomainEventDiscriminants::PasswordResetRequested)
    }
}

impl From<PasswordResetRequested> for DomainEvent {
    fn from(value: PasswordResetRequested) -> Self {
        DomainEvent::PasswordResetRequested(value)
    }
}