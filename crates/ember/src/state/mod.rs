use smithay::reexports::wayland_server::DisplayHandle;

use self::ember::Ember;

mod client;
pub mod ember;

pub struct CalloopData {
    state: Ember,
    display_handle: DisplayHandle,
}

impl CalloopData {
    pub fn new(state: Ember, display_handle: DisplayHandle) -> Self {
        Self {
            state,
            display_handle,
        }
    }
}
