use std::fmt::Debug;

use crate::domain::value_objects::{date::Date, pid::Pid};

/// Represents a domain event - something that happened in the domain.
pub trait DomainEvent: Debug + Send + Sync {
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
