use std::{ffi::OsString, time::Instant};

use smithay::{
    desktop::{Space, Window},
    reexports::{
        calloop::{EventLoop, LoopSignal},
        wayland_server::{Display, DisplayHandle},
    },
    wayland::{
        compositor::CompositorState, output::OutputManagerState, shell::xdg::XdgShellState,
        shm::ShmState,
    },
};

use super::CalloopData;

pub struct Ember {
    start_time: Instant,
    socket_name: OsString,
    display_handle: DisplayHandle,

    space: Space<Window>,
    loop_signal: LoopSignal,

    compositor_state: CompositorState,
    xdg_shell_state: XdgShellState,
    shm_state: ShmState,
    output_manager_state: OutputManagerState,
}

impl Ember {
    pub fn new(event_loop: &mut EventLoop<CalloopData>, display: Display<Self>) -> Self {
        let start_time = Instant::now();

        let display_handle = display.handle();

        todo!()
    }
}
