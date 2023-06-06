
/*
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
    */