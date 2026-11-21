use crate::{Identifiable, SessionId};

impl Identifiable for SessionId {
    /// The flow id of the flow that runs in the session: the first six
    /// characters of the session id.
    fn identify(&self) -> flow::FlowId {
        flow::FlowId(self.0.chars().take(6).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_takes_the_first_six_characters() {
        let session = SessionId("15c0ce3a-7d41-4b2e-9f10-0c8a5e2d6b77".to_string());
        assert_eq!(session.identify(), flow::FlowId("15c0ce".to_string()));
    }

    #[test]
    fn identify_keeps_a_short_session_whole() {
        let session = SessionId("abc".to_string());
        assert_eq!(session.identify(), flow::FlowId("abc".to_string()));
    }
}
