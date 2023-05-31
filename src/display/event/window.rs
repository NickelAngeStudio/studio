/// Enumeration of possible events for a window
#[derive(Copy, Clone)]
pub enum WindowEventWindow {

    /// Happens when Display is shown.
    Shown(),

    /// Happens when Display is hidden.
    Hidden(),

    /// Happens when Display is exposed/damaged, meaning part of drawing is lost and need to be redraw.
    /// Provides position (x, y) and size (width, height) of region exposed. 
    Exposed((i32, i32), (u32, u32)),

    /// Happens when Display is moved. Provides (x,y) of new position.
    Moved((i32, i32)),

    /// Happens when Display is moved and resized. Provides (x,y) of new position and (height, width) of new size.
    MovedResized((i32, i32), (u32, u32)),

    /// Happens when Display is Resized. Provides (height, width) of new size.
    Resized((u32, u32)),

    /// Happens when Display is minimized.
    /// 
    /// # Known issue(s)
    /// * `(Linux only)` Won't trigger if window is maximized.
    Minimized(),

    /// Happens when Display is maximized.
    Maximized(),

    /// Happens when Display is set fullscreen.
    Fullscreen(),

    /// Happens when Display is restored from minimized, maximized or fullscreen.
    Restored(),

    /// Happens when cursor enter Display.
    CursorEnter(),

    /// Happens when cursor leave Display.
    CursorLeave(),

    /// Happens when Display gain focus.
    Focus(),

    /// Happens when Display lose focus.
    Blur(),

    /// Happens when Display closes.
    Close(),
}

impl std::fmt::Debug for WindowEventWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shown() => f.debug_tuple("Shown").finish(),
            Self::Hidden() => f.debug_tuple("Hidden").finish(),
            Self::Exposed(arg0, arg1) => f.debug_tuple("Exposed").field(arg0).field(arg1).finish(),
            Self::Moved(arg0) => f.debug_tuple("Moved").field(arg0).finish(),
            Self::MovedResized(arg0, arg1) => f.debug_tuple("MovedResized").field(arg0).field(arg1).finish(),
            Self::Resized(arg0) => f.debug_tuple("Resized").field(arg0).finish(),
            Self::Minimized() => f.debug_tuple("Minimized").finish(),
            Self::Maximized() => f.debug_tuple("Maximized").finish(),
            Self::Fullscreen() => f.debug_tuple("Fullscreen").finish(),
            Self::Restored() => f.debug_tuple("Restored").finish(),
            Self::CursorEnter() => f.debug_tuple("CursorEnter").finish(),
            Self::CursorLeave() => f.debug_tuple("CursorLeave").finish(),
            Self::Focus() => f.debug_tuple("Focus").finish(),
            Self::Blur() => f.debug_tuple("Blur").finish(),
            Self::Close() => f.debug_tuple("Close").finish(),
        }
    }
}