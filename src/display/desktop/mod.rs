/// Cursor properties and mode
pub mod cursor;

/// Window abtraction and properties
pub mod window;


/*
impl Window {
    /// Create a new sized [Display] in the middle of the main default screen.
    /// 
    /// Return New [`Display`].
    /// 
    /// 
    /// # Error(s)
    /// Returns [StudioError::Display(DisplayError::NoDisplayServer)] if no display server found on Linux.
    /// 
    /// Returns [StudioError::Display(DisplayError::SizeError)] if width and/or height aren't within allowed boundaries.
    pub fn new(width:u32, height:u32) -> Result<Window, StudioError> {

        match_cfg! {
            linux => crate::display::provider::linux::get_linux_window(width, height),
            windows => Self::from_provider(WindowProvider::Windows),
            _ => todo!()
        }

    }

    /// Create a [Display] from specified provider in the middle of primary screen.
    /// 
    /// # Error(s)
    /// Returns [StudioError::Display(DisplayError::NotSupported)] if display server not supported.
    /// 
    /// Returns [StudioError::Display(DisplayError::SizeError)] if width and/or height aren't within allowed boundaries.
    pub fn from_provider(provider : WindowProvider, width:u32, height:u32) -> Result<Window, StudioError> {   

        if Window::is_size_within_boundaries(width, height) {
            match provider {
                WindowProvider::Wayland => todo!(),
                WindowProvider::X11 => {
                    let window = Window{ dispatcher: WindowEventDispatcher::new(false), 
                        manager: Box::new(super::provider::linux::x11::WindowManagerX11::new(width, height)) };
                    Ok(window)
                },
                // Anything else is not supported.
                _ => Err(StudioError::Display(DisplayError::NotSupported)),
            }

        } else {
            Err(StudioError::Display(DisplayError::SizeError))
        }
        
    }

    /// Confine cursor to window, preventing it from exiting boundaries.  
    pub fn confine_cursor(&mut self) {        
        self.manager.confine_cursor();
    }

    /// Get the cursor position with as a pair (x,y).
    pub fn get_cursor_position(&self) -> (i32, i32) {
        self.manager.get_window_property().cursor.position
    }

    /// Get the [CursorMode] for the [Display] [KEventMouse](enum.KEventMouse.html) events.
    pub fn get_cursor_mode(&self) -> CursorMode{
        self.manager.get_window_property().cursor.mode
    }

    /// Returns position (x,y) of the [Display].
    pub fn get_position(&self) -> (i32, i32) {
        self.manager.get_window_property().position
    }

    /// Returns dimension (width, height) of the [Display].
    pub fn get_size(&self) -> (u32, u32) {
        self.manager.get_window_property().size
    }

    /// Hide system default cursor.
    pub fn hide_cursor(&mut self) {
        self.manager.hide_cursor();
    }

    /// Get if the cursor is confined to the window, preventing it from going further than window boundaries.
    pub fn is_cursor_confined(&self) -> bool {
        self.manager.get_window_property().cursor.confined
    }

    /// Get if the default operating system cursor is visible.
    pub fn is_cursor_visible(&self) -> bool {
        self.manager.get_window_property().cursor.visible
    }

    /// Returns if the [Display] is fullscreen or not.
    pub fn is_fullscreen(&self) -> bool {
        self.manager.get_window_property().is_fullscreen
    }
    

    /// Returns if the [Display] is maximized or not.
    pub fn is_maximized(&self) -> bool {
        self.manager.get_window_property().is_maximized
    }

    /// Returns if the [Display] is minimized or not.
    pub fn is_minimized(&self) -> bool {
        self.manager.get_window_property().is_minimized
    }

    /// Release cursor from window, allowing it to exit boundaries.
    /// 
    /// Cursor will ALWAYS be released if the window loses focus.
    pub fn release_cursor(&mut self) {
        // Release only if confined.
        if self.manager.get_window_property().cursor.confined {
            self.manager.release_cursor();
        }
    }

    /// Restore the [Display], undoing any minimized, maximized and/or fullscreen status.
    pub fn restore(&mut self) {
        self.manager.restore();
    }

    /// Show system default cursor.
    pub fn show_cursor(&mut self) {
        // Show only if not visible.
        if !self.manager.get_window_property().cursor.visible {
            self.manager.show_cursor();
        }
    }

    /// Set position of [Display] according to position (x,y).
    pub fn set_position(&mut self, position : (i32, i32)){
        self.manager.set_position(position);
    }

    /// Set dimension of [Display] according to size (width, height).
    /// 
    /// Returns Ok(0) if successful.
    /// 
    /// # Error(s)
    /// Returns [StudioError::Display(DisplayError::SizeError)] if width and/or height not within allowed boundaries.
    pub fn set_size(&mut self, size : (u32, u32)) -> Result<u8, StudioError>{
        // Make sure dimension are within boundaries.
        if Window::is_size_within_boundaries(size.0, size.1) {
            self.manager.set_size(size);
            Ok(0)
        } else {
            Err(StudioError::Display(DisplayError::SizeError))
        }
    }

    /// Set the [Display] as fullscreen according to [DisplayFullscreenMode] parameter.
    pub fn set_fullscreen(&mut self, fs_mode : FullscreenMode) {
        if !self.manager.get_window_property().is_fullscreen {
            self.manager.set_fullscreen(fs_mode);
        }
    }

    /// Set the cursor position with a pair (x,y).
    pub fn set_cursor_position(&mut self, position : (i32, i32)){
        self.manager.set_cursor_position(position); 
    }

    /// Set the cursor mode for the [Display] [KEventMouse](enum.KEventMouse.html) events.
    pub fn set_cursor_mode(&mut self, mode : CursorMode) {
        self.manager.set_cursor_mode(mode);
    }

    /// Return True if width and size are between boundaries.
    #[doc(hidden)]
    fn is_size_within_boundaries(width:u32, height:u32) -> bool {

        if width >= WINDOW_MIN_WIDTH && width <= WINDOW_MAX_WIDTH && height >= WINDOW_MIN_HEIGHT && height <= WINDOW_MAX_HEIGHT {
            // Withing boundaries
            true
        } else {
            // Boundaries overflow
            false
        }

    }

    /// Returns the [KWindow] title. 
    pub fn get_title(&self) -> &str {
        &self.manager.get_window_property().title
    }

    /// Set a new title for the [KWindow].
    pub fn set_title(&mut self, title : &str) {
        self.manager.set_title(title);
    }

}
        

*/