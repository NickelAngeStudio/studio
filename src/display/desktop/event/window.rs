
/// Enumeration of possible events for a window
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventWindow {

    /// Happens when window is created.
    Created,

    /// Happens when Window is shown.
    Shown,

    /// Happens when Window is hidden.
    Hidden,

    /// Happens when Window is exposed/damaged, meaning part of drawing is lost and need to be redraw.
    /// Provides position (x, y) and size (width, height) of region exposed. 
    Exposed((i32, i32), (u32, u32)),

    /// Happens when Window is moved. Provides (x,y) of new position.
    Moved((i32, i32)),

    /// Happens when Window is moved and resized. Provides (x,y) of new position and (height, width) of new size.
    MovedResized((i32, i32), (u32, u32)),

    /// Happens when Window is Resized. Provides (height, width) of new size.
    Resized((u32, u32)),

    /// Happens when Window is minimized.
    /// 
    /// # Known issue(s)
    /// * `(Linux only)` Won't trigger if window is maximized.
    Minimized,

    /// Happens when Window is maximized.
    Maximized,

    /// Happens when Window is set fullscreen.
    Fullscreen,

    /// Happens when Window is restored from minimized, maximized or fullscreen.
    Restored,

    /// Happens when cursor enter Window.
    CursorEnter,

    /// Happens when cursor leave Window.
    CursorLeave,

    /// Happens when Window gain focus.
    Focus,

    /// Happens when Window lose focus.
    Blur,

    /// Happens when a close request is sent from the client.
    CloseRequest,

    /// Happens when a window was closed.
    Closed,

    /// Happens when a sub window closed
    SubWindowClosed,

    /// Happens when a Modal subwindow showed
    ModalShowed,

    /// Happens when a Modal subwindow closed
    ModalClosed,



}