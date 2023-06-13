//! Linux implementations of [Window].



use crate::{display::{ desktop::{ event::Event, property::{WindowProperty}, manager::WindowManager}, DisplayError}, error::StudioError};
use self::{wayland::WaylandWindowManager, x11::X11WindowManager};
use super::WindowProvider;

/// Wayland DisplayManager
pub mod wayland;

/// X11 DisplayManager
pub mod x11;

/// Macro that redirect function to correct window manager. 
macro_rules! wmfn {
    ($self : ident, $funct : ident ( $($param : tt)* )) => {

        if $self.use_wayland {
            $self.x11.unwrap().$funct($($param)*)
        } else {
            $self.wayland.unwrap().$funct($($param)*)
        }
    };
}

pub(crate) struct LinuxWindowManager {

    use_wayland : bool,

    wayland : Option<WaylandWindowManager>,
    x11 : Option<X11WindowManager>,

}

impl WindowManager for LinuxWindowManager {
    fn new() -> Result<Self, StudioError> where Self : Sized {
        
        if wayland::WaylandWindowManager::is_supported() {
            Ok(LinuxWindowManager{ 
                wayland: Some(wayland::WaylandWindowManager::new().unwrap()), 
                x11: Option::None,
                use_wayland: true,
            })
        } else if x11::X11WindowManager::is_supported() {
            Ok(LinuxWindowManager{ 
                wayland: Option::None, 
                x11: Some(x11::X11WindowManager::new().unwrap()),
                use_wayland: false,
            })
        } else {    // No supported display server available
            Err(StudioError::Display(DisplayError::NoDisplayServer))
        }

    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        wmfn!(self, get_window_provider())
    }

    #[inline(always)]
    fn poll_event(&mut self) -> Event  {
         wmfn!(self, poll_event())
    }

    #[inline(always)]
    fn show(&mut self, property : &WindowProperty) {
        wmfn!(self, show(property))
    }

    #[inline(always)]
    fn restore(&mut self) {
         wmfn!(self, restore());
    }

    #[inline(always)]
    fn close(&mut self) {
         wmfn!(self, close());
    }

    #[inline(always)]
    fn hide(&mut self) {
         wmfn!(self, hide());
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String) -> bool {
         wmfn!(self, set_title(title))
    }

    #[inline(always)]
    fn set_position(&mut self, position : (i32,i32)) -> bool {
         wmfn!(self, set_position(position))
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32)) -> bool {
         wmfn!(self, set_size(size))
    }

    #[inline(always)]
    fn show_decoration(&mut self) -> bool {
         wmfn!(self, show_decoration())
    }

    #[inline(always)]
    fn hide_decoration(&mut self) -> bool {
         wmfn!(self, hide_decoration())
    }

    #[inline(always)]
    fn minimize(&mut self) -> bool {
         wmfn!(self, minimize())
    }

    #[inline(always)]
    fn maximize(&mut self) -> bool {
         wmfn!(self, maximize())
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) -> bool {
         wmfn!(self, enable_autorepeat())
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) -> bool {
         wmfn!(self, disable_autorepeat())
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : &(i32, i32)) -> bool {
         wmfn!(self, set_pointer_position(position))
    }

    #[inline(always)]
    fn show_pointer(&mut self) -> bool {
         wmfn!(self, show_pointer())
    }

    #[inline(always)]
    fn hide_pointer(&mut self) -> bool {
         wmfn!(self, hide_pointer())
    }

    #[inline(always)]
    fn confine_pointer(&mut self) -> bool {
         wmfn!(self, confine_pointer())
    }

    #[inline(always)]
    fn release_pointer(&mut self) -> bool {
         wmfn!(self, release_pointer())
    }

    #[inline(always)]
    fn get_window_handle(&self) -> Option<*const usize> {
        wmfn!(self, get_window_handle())
    }  

    #[inline(always)]
    fn get_display_handle(&self) -> Option<*const usize> {
        wmfn!(self, get_display_handle())
    }

}



/*
/// Linux implementation of a window.
pub(crate) struct LinuxWindow<'window> {

    use_wayland : bool,
    
    wayland : Option<X11Window<'window>>,
    x11 : Option<X11Window<'window>>,

}

impl<'window> LinuxWindow<'window> {
    pub(crate) fn get_properties_mut(&mut self) -> &mut WindowProperty{
         wmfn!(self, get_properties_mut()
    }
}

impl<'window> Window for LinuxWindow<'window> {

    fn new() -> Result<Self, StudioError> {

        let mut linux_window = LinuxWindow{ use_wayland: false, wayland: None, x11: None };
        
        match unsafe { DefaultLinuxWindowProvider } {
            Some(provider) => {
                match provider{
                    WindowProvider::Wayland => todo!(),
                    WindowProvider::X11 => todo!(),
                    _ => Err(StudioError::Display(DisplayError::NotSupported)),
                }
            },
            None => {
                // TODO: Verify is Wayland is supported, if not, fall back to X11.
                unsafe { DefaultLinuxWindowProvider = Some(WindowProvider::X11) };
                
                match X11Window::new(){
                    Ok(mut x11win) => {
                        Ok(linux_window)
                    },
                    Err(err) => Err(err),
                }
            },
        }
    }

    fn show(&mut self) -> Result<bool, StudioError> {
         wmfn!(self, show()
    }

    fn hide(&mut self) {
         wmfn!(self, hide();
    }

    fn close(&mut self) {
         wmfn!(self, close();
    }

    fn poll_event(&mut self) -> Event {
         wmfn!(self, poll_event()
    }

    fn get_provider(&self) -> WindowProvider {
         wmfn!(self, get_provider()
    }

    fn get_properties(&self) -> &WindowProperty {
         wmfn!(self, get_properties()
    }

    fn set_property(&mut self, property : WindowPropertySet) -> Result<usize, StudioError> {
         wmfn!(self, set_property(property)
    }

    fn set_properties(&mut self, properties : &[WindowPropertySet]) -> Result<usize, StudioError> {
         wmfn!(self, set_properties(properties)
    }

    fn get_window_handle(&self) -> Option<*const usize> {
         wmfn!(self, get_window_handle()
    }

    fn get_display_handle(&self) -> Option<*const usize> {
         wmfn!(self, get_display_handle()
    }

    

    

}
*/
/*
/// Redirect call to the correct window manager
macro_rules! linux_wm {
    ($self:ident) => {
        unsafe {
            match DefaultLinuxWindowProvider{
                Some(provider) => match provider {
                    WindowProvider::Wayland => todo!(),
                    WindowProvider::X11 => $self.x11manager.unwrap(),
                    _ => { panic!("Provider not compatible!") }
                },
                None => panic!("Linux Provider not specified!"),
            }
        }
    };
}


/// Linux [WindowManager] managing Wayland and X11 calls.
pub(crate) struct LinuxWindowManager {
    x11manager : Option<X11WindowManager>,
}

impl LinuxWindowManager {
   
    pub fn from_provider(provider: WindowProvider) -> Result<LinuxWindowManager, StudioError> {
        match provider {
            WindowProvider::Wayland => todo!(),
            WindowProvider::X11 => match X11WindowManager::new() {
                Ok(manager) => {
                    Ok(LinuxWindowManager { x11manager: Some(manager) })
                },
                Err(err) => Err(err),
            },
            _ => Err(StudioError::Display(DisplayError::NotSupported)),
        }

    }
}

impl<'window> WindowManager<'window> for LinuxWindowManager {
    /// Create a new LinuxWindowManager according to DefaultLinuxWindowProvider.
    /// If DefaultLinuxWindowProvider is None, will try to see if Wayland is compatible
    /// then fallback to X11. 
    fn new() -> Result<Self, StudioError> {
        match DefaultLinuxWindowProvider {
            Some(provider) => LinuxWindowManager::from_provider(provider),
            None => {
                // TODO:Test if wayland compatible then fallback to X11 if not
                DefaultLinuxWindowProvider = Some(WindowProvider::X11);
                LinuxWindowManager::new()
            },
        }
    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        linux_wm!(self).get_window_provider()
    }

    #[inline(always)]
    fn poll_event(&mut self) -> Event  {
        linux_wm!(self).poll_event()
    }

    #[inline(always)]
    fn send_event(&mut self, event : Event) {
        linux_wm!(self).send_event(event);
    }

    #[inline(always)]
    fn show(&mut self, parameters : WindowManagerParameter) {
        linux_wm!(self).show(parameters);
    }

    #[inline(always)]
    fn show_child(&mut self, parent : &dyn WindowManager, parameters : WindowManagerParameter, option : WindowChildDisplayOption) {
        linux_wm!(self).show_child(parent, parameters, option);
    }

    #[inline(always)]
    fn restore(&mut self)  {
        linux_wm!(self).restore();
    }

    #[inline(always)]
    fn close(&mut self) {
        linux_wm!(self).close();
    }

    #[inline(always)]
    fn hide(&mut self) {
        linux_wm!(self).hide();
    }

    #[inline(always)]
    fn get_screen_list(&self) -> Result<&ScreenList, StudioError> {

        // Cache screen list.
        match &LinuxScreenList {
            Some(screens) => {
                Ok(&screens)
            },
            None => {
                match linux_wm!(self).get_screen_list(){
                    Ok(screens) => {
                        LinuxScreenList = Some(*screens);
                        Ok(screens)
                    },
                    Err(err) => Err(err),
                }
            } 
        }
    }

    #[inline(always)]
    fn show_decoration(&mut self) {
        linux_wm!(self).show_decoration();
    }

    #[inline(always)]
    fn hide_decoration(&mut self) {
        linux_wm!(self).hide_decoration();
    }

    #[inline(always)]
    fn set_title(&mut self, title:&str) {
        linux_wm!(self).set_title(title);
    }

    #[inline(always)]
    fn set_size(&mut self, size : (u32, u32)) {
        linux_wm!(self).set_size(size);
    }

    #[inline(always)]
    fn set_position(&mut self, position : (i32,i32)) {
        linux_wm!(self).set_position(position);
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32,i32)) {
        linux_wm!(self).set_pointer_position(position);
    }

    #[inline(always)]
    fn hide_pointer(&mut self) {
        linux_wm!(self).hide_pointer();
    }

    #[inline(always)]
    fn show_pointer(&mut self) {
        linux_wm!(self).show_pointer();
    }

    #[inline(always)]
    fn confine_pointer(&mut self) {
        linux_wm!(self).confine_pointer();
    }

    #[inline(always)]
    fn release_pointer(&mut self) {
        linux_wm!(self).release_pointer();
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) {
        linux_wm!(self).enable_autorepeat();
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) {
        linux_wm!(self).disable_autorepeat();
    }

    #[inline(always)]
    fn get_left_button_index(&self) -> u32 {
        linux_wm!(self).get_left_button_index()
    }

    #[inline(always)]
    fn get_right_button_index(&self) -> u32 {
        linux_wm!(self).get_right_button_index()
    }

    #[inline(always)]
    fn get_middle_button_index(&self) -> u32 {
        linux_wm!(self).get_middle_button_index()
    }

    #[inline(always)]
    fn get_next_button_index(&self) -> u32 {
        linux_wm!(self).get_next_button_index()
    }

    #[inline(always)]
    fn get_previous_button_index(&self) -> u32 {
        linux_wm!(self).get_previous_button_index()
    }

    #[inline(always)]
    fn get_scroll_up_index(&self) -> u32 {
        linux_wm!(self).get_scroll_up_index()
    }

    #[inline(always)]
    fn get_scroll_down_index(&self) -> u32 {
        linux_wm!(self).get_scroll_down_index()
    }

    #[inline(always)]
    fn get_scroll_left_index(&self) -> u32 {
        linux_wm!(self).get_scroll_left_index()
    }

    #[inline(always)]
    fn get_scroll_right_index(&self) -> u32 {
        linux_wm!(self).get_scroll_right_index()
    }

    fn as_any(&'window self) -> &dyn Any {
        todo!()
    }



}
*/

/*
/// Get linux Display. Will try wayland as provider first then X11.
/// 
/// # Error(s)
/// Returns [StudioError::Display(DisplayError::NoDisplayServer)] if no compatible display server found on Linux.
pub(crate) fn get_linux_window(width:u32, height:u32) -> Result<impl Window, StudioError> {

    // TODO: Replace with Wayland first
    get_x11_window(width, height)

    
     // Try Wayland first
     match Window::from_provider(WindowProvider::Wayland, width, height) {
        Ok(window) => {
            Ok(window)
        },
        Err(_) => {
            // Then try x11
            match Window::from_provider(WindowProvider::X11, width, height) {
                Ok(window) => {
                    Ok(window)
                },
                Err(_) => {
                    // Return error that no display server were found.
                    Err(StudioError::Display(DisplayError::NoDisplayServer))
                },
            }
        },
    }
    
}


pub fn get_x11_window(width:u32, height:u32) -> Result<impl Window, StudioError> {
    if x11::X11Window::is_supported() {
        Ok(x11::X11Window::new(width, height))
    } else {
        Err(StudioError::Display(DisplayError::NotSupported))
    }
}

/// Get linux screen list. Will try wayland as provider first then X11.
/// 
/// # Error(s)
/// eturns Err([StudioError::Display(DisplayError::ScreenDetailError)]) if an error occurred while creating screen list.
pub(crate) fn get_linux_screen_list() -> Result<ScreenList, StudioError> {
    // Try Wayland first
    match ScreenList::from_provider(WindowProvider::Wayland) {
       Ok(window) => {
           Ok(window)
       },
       Err(_) => {
           // Then try x11
           match ScreenList::from_provider(WindowProvider::X11) {
               Ok(window) => {
                   Ok(window)
               },
               Err(_) => {
                   // Return error that no display server were found.
                   Err(StudioError::Display(DisplayError::ScreenDetailError))
               },
           }
       },
   }
}


/// Macro shortcut to execute either wayland or x11 function.
#[doc(hidden)]
#[macro_export]
macro_rules! wayland_or_x11 {
    ($provider:expr, $if_wayland:block, $else:block) => {
        match $provider {
            KLinuxDisplayServerProvider::Wayland => $if_wayland,
            _ => $else,
        }
    }
}


/// Implementation of privates elements relatives to linux distributions
#[doc(hidden)]
impl Display {
    /// Create new Display
    pub(super) fn __new(width:u32, height:u32, provider : super::linux::server::KLinuxDisplayServerProvider) -> Result<Display, StudioError> {
        // Default cursor.
        let cursor = KCursorProperty { mode: super::KCursorMode::Pointer, position: (0,0), visible: true, confined: false };
        
        // Default center position
        let center = ((width as i32 / 2), (height as i32 / 2));

        match  super::linux::server::KLinuxDisplayServer::new(width, height, provider){
            Ok(display_server) => {
                match ScreenList::new(display_server.provider){
                    Ok(screen_list) => {
                        let mut property = DisplayProperty { title : String::from(""), cursor, position: (0,0), size: (width, height), center, minimized: false, maximized: false, fullscreen: false };
                        match display_server.provider {     // Fetch window position according to provider
                            KLinuxDisplayServerProvider::Wayland => todo!(),
                            KLinuxDisplayServerProvider::X11 => {
                                // Set correct x11 window position
                                property.position = Display::get_x11_window_position(display_server.display, display_server.window);
                                Ok(Display { screen_list, property, display_server })
                            },
                            _ => Err(StudioError::Display(DisplayError::NoDisplayServer)),
                        }
                    },
                    Err(_) => Err(StudioError::Display(DisplayError::ScreenDetailError)),
                }
            },
            Err(err) => Err(err),
        }  
    }
        

    // Pop an event from the queue
    #[inline(always)]
    pub(super) fn __poll_event(&mut self) -> KEvent {
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_poll_event() 
            }, { 
                self.x11_poll_event() 
            }
        }
    }

    // Sync an event from the queue
    #[inline(always)]
    pub(super) fn __sync_events(&self) {
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_sync_events();
            }, { 
                self.x11_sync_events();
            }
        }
    }

    /// Get the count of events that need handling.
    #[inline(always)]
    pub(super) fn __get_event_count(&self) -> usize {
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_get_event_count() 
            }, { 
                self.x11_get_event_count() 
            }
        }
    }

    /// Set the cursor position
    #[inline(always)]
    pub(super) fn __set_cursor_position(&mut self, position : (i32, i32)){
        wayland_or_x11!{self.display_server.provider, { 
                self.wayland_set_cursor_position(position);
                
            }, { 
                self.x11_set_cursor_position(position);
            }
        }
    }

    /// Hide system default cursor.
    #[inline(always)]
    pub(super) fn __hide_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_hide_cursor();
            
            }, { 
                self.x11_hide_cursor();
            }
        }
    }

    /// Show system default cursor.
    #[inline(always)]
    pub(super) fn __show_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_show_cursor();
            
            }, { 
                self.x11_show_cursor();
            }
        }
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[inline(always)]
    pub(super) fn __confine_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_confine_cursor();
            
            }, { 
                self.x11_confine_cursor();
            }
        }
    }


    /// Release cursor from window, allowing it to exit boundaries.
    #[inline(always)]
    pub(super) fn __release_cursor(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_release_cursor();
            
            }, { 
                self.x11_release_cursor();
            }
        }
    }


    /// Restore the [Display], undoing any minimized, maximized and/or fullscreen status.
    #[inline(always)]
    pub(super) fn __restore(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_restore();
            
            }, { 
                self.x11_restore();
            }
        }
    }

    /// Set a new title for the [Display].
    #[inline(always)]
    pub(super) fn __set_title(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_title();
            
            }, { 
                self.x11_set_title();
            }
        }
    }

    /// Set a size of [Display].
    #[inline(always)]
    pub(super) fn __set_size(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_size();
            
            }, { 
                self.x11_set_size();
            }
        }
    }

    /// Set a position of [Display].
    #[inline(always)]
    pub(super) fn __set_position(&mut self) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_position();
            
            }, { 
                self.x11_set_position();
            }
        }
    }

    /// Set the [Display] as fullscreen.
    #[inline(always)]
    pub(super) fn __set_fullscreen(&mut self, mode : DisplayFullscreenMode) {
        wayland_or_x11!{self.display_server.provider, { 
            self.wayland_set_fullscreen(mode);
            
            }, { 
                self.x11_set_fullscreen(mode);
            }
        }
    }
    

}
*/