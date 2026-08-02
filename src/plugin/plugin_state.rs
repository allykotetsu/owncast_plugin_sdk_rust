use std::collections::HashMap;

pub struct PluginState {
    pub(crate) timers: HashMap<i64, (fn() -> (), bool)>
}

impl PluginState {
    pub fn new() -> Self {
        Self {
            timers: HashMap::new()
        }
    }

    pub(crate) fn set_timer(&mut self, callback: fn() -> (), repeats: bool) -> i64 {
        let mut id = 0;
        let id = loop {
            if !self.timers.contains_key(&id) {
                break id;
            }
            id += 1;
        };

        self.timers.insert(id, (callback, repeats));
        id
    }

    pub(crate) fn clear_timer(&mut self, id: i64) {
        self.timers.remove(&id);
    }

    pub(crate) fn fire_timer(&mut self, id: i64) {
        if let Some((callback, repeats)) = self.timers.get(&id) {
            callback();
            if !repeats {
                self.timers.remove(&id);
            }
        }
    }
}