use crate::{Event, Flow, Observed, State, Transitionable};

impl Transitionable for Observed {
    fn after(&mut self, input: Event) {
        match input {
            Event::Started => self.state = State::Dry,
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
