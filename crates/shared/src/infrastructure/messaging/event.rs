use std::{any::Any, fmt::Debug};

use crate::domain::value_objects::{date::Date, pid::Pid};

/// Represents a domain event - something that happened in the domain.
pub trait DomainEvent: Debug + Send + Sync + Any {
    /// Returns the type name of this event (e.g., "StashCreated")
    fn event_type(&self) -> &str;

    /// Returns the ID of the aggregate that emitted this event
    fn aggregate_id(&self) -> Pid;

    /// Returns when this event occurred
    fn occurred_at(&self) -> Date;

    /// Optional: Returns correlation ID for tracing
    fn correlation_id(&self) -> Option<String> {
        None
    }

    /// Optional: Returns causation ID (the event that caused this event)
    fn causation_id(&self) -> Option<String> {
        None
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: DomainEvent> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AsAny for Box<dyn DomainEvent> {
    fn as_any(&self) -> &dyn Any {
        &**self
    }
}

pub fn downcast_event<T: DomainEvent>(event: &Box<dyn DomainEvent>) -> &T {
    event.as_any().downcast_ref::<T>().unwrap()
}
