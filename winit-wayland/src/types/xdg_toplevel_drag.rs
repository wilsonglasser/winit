//! Handling of xdg toplevel drag, which lets a toplevel be carried by a drag-and-drop operation:
//! the compositor moves the attached window with the cursor until the drag ends, and that window
//! takes no part in choosing the drop target.

use sctk::globals::GlobalData;
use wayland_client::globals::{BindError, GlobalList};
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle, delegate_dispatch};
use wayland_protocols::xdg::toplevel_drag::v1::client::xdg_toplevel_drag_manager_v1::XdgToplevelDragManagerV1;
use wayland_protocols::xdg::toplevel_drag::v1::client::xdg_toplevel_drag_v1::XdgToplevelDragV1;

use crate::state::WinitState;

#[derive(Debug)]
pub struct XdgToplevelDragState {
    manager: XdgToplevelDragManagerV1,
}

impl XdgToplevelDragState {
    pub fn bind(
        globals: &GlobalList,
        queue_handle: &QueueHandle<WinitState>,
    ) -> Result<Self, BindError> {
        let manager = globals.bind(queue_handle, 1..=1, GlobalData)?;
        Ok(Self { manager })
    }

    pub fn global(&self) -> &XdgToplevelDragManagerV1 {
        &self.manager
    }
}

/// The `xdg_toplevel_drag_v1` of a running drag.
///
/// The protocol only allows destroying it once the drag has ended, so it lives with the drag
/// source and is destroyed when that is dropped.
#[derive(Debug)]
pub struct ToplevelDrag(pub(crate) XdgToplevelDragV1);

impl Drop for ToplevelDrag {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

impl Dispatch<XdgToplevelDragManagerV1, GlobalData, WinitState> for XdgToplevelDragState {
    fn event(
        _: &mut WinitState,
        _: &XdgToplevelDragManagerV1,
        _: <XdgToplevelDragManagerV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<WinitState>,
    ) {
        // No events.
    }
}

impl Dispatch<XdgToplevelDragV1, GlobalData, WinitState> for XdgToplevelDragState {
    fn event(
        _: &mut WinitState,
        _: &XdgToplevelDragV1,
        _: <XdgToplevelDragV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<WinitState>,
    ) {
        // No events.
    }
}

delegate_dispatch!(WinitState: [XdgToplevelDragManagerV1: GlobalData] => XdgToplevelDragState);
delegate_dispatch!(WinitState: [XdgToplevelDragV1: GlobalData] => XdgToplevelDragState);
