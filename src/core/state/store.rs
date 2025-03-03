use std::any::Any;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct WidgetId(pub Uuid);

impl WidgetId {
    pub fn new() -> Self {
        WidgetId(Uuid::new_v4())
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StateStore {
    states: HashMap<WidgetId, Box<dyn Any>>,
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new()
    }
}

impl StateStore {
    pub fn new() -> Self {
        StateStore {
            states: HashMap::new(),
        }
    }

    pub fn get_state(&self, id: &WidgetId) -> Option<&dyn Any> {
        self.states.get(id).map(|boxed| boxed.as_ref())
    }

    pub fn set_state(&mut self, id: WidgetId, state: Box<dyn Any>) {
        self.states.insert(id, state);
    }

    pub fn get_state_as<T: 'static>(&self, id: &WidgetId) -> Option<&T> {
        self.get_state(id).and_then(|state| state.downcast_ref::<T>())
    }
}
