use async_trait::async_trait;
use figure_lib::queue::internal_event_router::{Context, FromContext, StateTrait};
use strum_macros::EnumDiscriminants;

#[derive(Clone, Eq, Hash, PartialEq, EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum DomainEvent {
    UserCreated(UserCreatedDomainEvent),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct UserCreatedDomainEvent {
    id: String,
    email: String,
    role: String,
}

#[async_trait]
impl<S: StateTrait> FromContext<DomainEventDiscriminants, DomainEvent, S> for UserCreatedDomainEvent
{
    async fn from_context(ctx: &Context<DomainEvent, S>) -> Self {
        match &ctx.event {
            DomainEvent::UserCreated(event) => event.clone(),
            _ => unreachable!()
        }
    }

    fn topic() -> Option<DomainEventDiscriminants> {
        Some(DomainEventDiscriminants::UserCreated)
    }
}