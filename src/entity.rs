use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum EntityType {
    Elevator,
    Person,
}

pub(crate) trait Entity: Sized {
    fn entity_type() -> EntityType;
    fn simulate(&mut self, time: Duration);
}