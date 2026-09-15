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
        // Generate a new timer ID.
        let mut id = 0;

        // Start at 0, loop incrementing the ID until an ID is generated that's not in the hashmap.
        let id = loop {
            if !self.timers.contains_key(&id) {
                break id;
            }
            id += 1;
        };

        // Insert ID, callback, and repetition info into the timers hashmap.
        self.timers.insert(id, (callback, repeats));

        // Return the ID.
        id
    }

    pub(crate) fn clear_timer(&mut self, id: i64) {
        self.timers.remove(&id);
    }

    pub(crate) fn fire_timer(&mut self, id: i64) {
        // If the timer can be found, then call its callback function.
        if let Some((callback, repeats)) = self.timers.get(&id) {
            callback();

            // If it doesn't repeat, then remove it from the hashmap.
            if !repeats {
                self.timers.remove(&id);
            }
        }
    }
}