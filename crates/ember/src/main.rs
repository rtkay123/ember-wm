mod dbg;
mod state;

use smithay::reexports::{calloop::EventLoop, wayland_server::Display};

use self::state::{ember::Ember, CalloopData};

fn main() -> anyhow::Result<()> {
    dbg::init_logger();

    let mut event_loop = EventLoop::try_new()?;

    let display = Display::new()?;
    let display_handle = display.handle();
    let state = Ember::new(&mut event_loop, display);

    let mut data = CalloopData::new(state, display_handle);

    Ok(())
}
