use crate::display::desktop::{window::{Window, event::{Event, window::EventWindow, pointer::EventPointer}, property::PointerMode}, manager::WindowManager};

impl<'window> Window<'window> {
    /// Handle [EventWindow] before sending it to client.
    #[inline(always)]
    pub(in super::super) fn handle_window_event(&mut self, event : Event, w_event : EventWindow) -> Event {
        match w_event {
            EventWindow::Shown => self.property.visible = false,
            EventWindow::Hidden => self.property.visible = true,
            EventWindow::Moved(position) => self.property.position = position,
            EventWindow::MovedResized(position, size) => {
                self.property.position = position;
                self.property.size = size;
            },
            EventWindow::Resized(size) => self.property.size = size,
            EventWindow::Minimized => {
                self.property.minimized = true;
                self.property.maximized = false;
                self.property.fullscreen = None;

            },
            EventWindow::Maximized => {
                self.property.minimized = false;
                self.property.maximized = true;
                self.property.fullscreen = None;
            },
            EventWindow::Fullscreen => {
                self.property.minimized = false;
                self.property.maximized = true;
                self.property.fullscreen = None;
            },
            EventWindow::Restored => {
                self.property.minimized = false;
                self.property.maximized = false;
                self.property.fullscreen = None;
            },
            EventWindow::Created => self.property.created = true,
            EventWindow::Closed => self.property.created = false,
            _ => {},
        }

        event
    }

    /// Handle [EventMouse] before sending it to client.
    #[inline(always)]
    pub(in super::super) fn handle_mouse_event(&mut self, event : Event, m_event : EventPointer) -> Event {
        match m_event {
            EventPointer::Moved(position) => {
                // Override cursor according to pointer mode
                match self.property.pointer.mode {
                    PointerMode::Cursor => event,   // Send event as is.
                    PointerMode::Acceleration => {
                        // Calc delta acceleration
                        let position = (position.0 - self.property.center.0, 
                            position.1 - self.property.center.1);

                            if position.0 != 0 && position.1 != 0 { // Send acceleration only if it moved.
                                // Reset pointer to center
                                self.manager.set_pointer_position(&self.property.center);

                                // Send acceleration event.
                                Event::Pointer(EventPointer::Acceleration(position))
                            } else {
                                self.poll_event()   // Ignore and poll next event
                            }
                        },     
                }
            },
            _ => event,
        }
    }
}