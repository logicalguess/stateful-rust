use std::fmt::Display;

pub struct Application<Interaction: 'static, Input: 'static, State: 'static, Event: 'static> {
    pub(crate) state: State,
    pub(crate) interaction_listener: &'static dyn Fn() -> Interaction,
    pub(crate) interaction_handler: &'static dyn Fn(Interaction) -> Input,
    pub(crate) domain_logic: &'static dyn Fn((&mut State, Input)) -> Event,
    pub(crate) publish_state: &'static dyn Fn(&State) -> (),
    pub(crate) event_handler: &'static dyn Fn(Event) -> (),
    pub(crate) respond: &'static dyn Fn(&dyn Display) -> (),
}

impl<Interaction, Input, State: Display, Event> Application<Interaction, Input, State, Event> {
    pub(crate) fn run(&mut self) {
        let msg = "\nWaiting for interaction...";
        (self.respond)(&self.state);
        loop {
            (self.respond)(&msg);
            let interaction = (self.interaction_listener)();
            let input = (self.interaction_handler)(interaction);
            let event = (self.domain_logic)((&mut self.state, input));
            (self.publish_state)(&self.state);
            (self.respond)(&self.state);
            (self.event_handler)(event);
        }
    }
}