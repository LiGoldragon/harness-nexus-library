use crate::{Event, Flow, Observed, State, Transitionable};

impl Transitionable for Observed {
    fn after(&mut self, input: Event) {
        match input {
            Event::Started => self.state = State::Dry,
            Event::Resumed => {
                self.state = State::Flowing(Flow::Rested);
                self.background_work_vector.clear();
            }
            Event::PromptSubmitted | Event::ToolUsed => {
                self.state = State::Flowing(Flow::Working);
            }
            Event::PermissionAsked => self.state = State::Flowing(Flow::Blocked),
            Event::InFlight(work) => self.background_work_vector = work,
            Event::WorkStarted(work) => self.background_work_vector.push(work),
            Event::WorkEnded(work_id) => {
                self.background_work_vector
                    .retain(|work| work.work_id != work_id);
                if self.state == State::Flowing(Flow::Waiting)
                    && self.background_work_vector.is_empty()
                {
                    self.state = State::Flowing(Flow::Rested);
                }
            }
            Event::TurnEnded | Event::Interrupted | Event::Failed(_) => {
                self.state = State::Flowing(if self.background_work_vector.is_empty() {
                    Flow::Rested
                } else {
                    Flow::Waiting
                });
            }
            Event::ContextSize(tokens) => self.tokens = tokens,
            Event::Ended(_) => self.state = State::Ended,
            Event::Responded(_) | Event::Compacted(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BackgroundWork, Description, Tokens, WorkId, WorkKind};

    fn observed(state: State, background_work_vector: Vec<BackgroundWork>) -> Observed {
        Observed {
            flow_id: flow::FlowId("15c0ce".to_string()),
            state,
            tokens: Tokens(0),
            background_work_vector,
        }
    }

    fn shell_work() -> BackgroundWork {
        BackgroundWork {
            work_id: WorkId("b1".to_string()),
            work_kind: WorkKind::Shell,
            description: Description("sleep".to_string()),
        }
    }

    #[test]
    fn resumed_rests_a_dry_session() {
        let mut session = observed(State::Dry, Vec::new());
        session.after(Event::Resumed);
        assert_eq!(session.state, State::Flowing(Flow::Rested));
    }

    #[test]
    fn resumed_rests_a_working_session_and_clears_its_background_work() {
        let mut session = observed(State::Flowing(Flow::Waiting), vec![shell_work()]);
        session.after(Event::Resumed);
        assert_eq!(session.state, State::Flowing(Flow::Rested));
        assert!(session.background_work_vector.is_empty());
    }

    #[test]
    fn resumed_keeps_the_context_size() {
        let mut session = observed(State::Flowing(Flow::Working), Vec::new());
        session.after(Event::ContextSize(Tokens(181204)));
        session.after(Event::Resumed);
        assert_eq!(session.tokens, Tokens(181204));
    }
}
