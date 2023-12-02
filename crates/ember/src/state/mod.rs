use smithay::{
    reexports::wayland_server::backend::ClientData, wayland::compositor::CompositorClientState,
};
use tracing::{instrument, trace};

#[derive(Default)]
pub struct Client {
    compositor_state: CompositorClientState,
}

impl Client {
    pub fn compositor_state(&self) -> &CompositorClientState {
        &self.compositor_state
    }
}

impl ClientData for Client {
    #[instrument(skip(self))]
    fn initialized(&self, client_id: smithay::reexports::wayland_server::backend::ClientId) {
        trace!("client initialised");
    }

    #[instrument(skip(self))]
    fn disconnected(
        &self,
        _client_id: smithay::reexports::wayland_server::backend::ClientId,
        _reason: smithay::reexports::wayland_server::backend::DisconnectReason,
    ) {
        trace!("client disconnected");
    }
}
