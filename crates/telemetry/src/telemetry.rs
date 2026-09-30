//! Telemetry is removed from this fork. Upstream call sites compile to nothing.
pub use serde_json;
pub use telemetry_events::FlexibleEvent as Event;

/// Ignore both the event and its property expressions without evaluating them.
#[macro_export]
macro_rules! event {
    ($($ignored:tt)*) => {{}};
}

#[macro_export]
macro_rules! serialize_property {
    ($key:ident) => {
        $key
    };
    ($key:ident = $value:expr) => {
        $value
    };
}
pub fn send_event(_: Event) {}
pub fn init(_: futures::channel::mpsc::UnboundedSender<Event>) {}

#[cfg(test)]
mod tests {
    #[test]
    fn no_telemetry_fork_does_not_evaluate_event_properties() {
        let called = std::cell::Cell::new(false);
        crate::event!(
            "Ignored",
            private_data = {
                called.set(true);
                "secret"
            }
        );
        assert!(!called.get());
    }
    #[test]
    fn no_telemetry_fork_never_enqueues_events() {
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        crate::init(tx.clone());
        crate::send_event(crate::Event {
            event_type: "Ignored".into(),
            event_properties: Default::default(),
        });
        assert!(rx.try_recv().is_err());
        drop(tx);
    }
}
