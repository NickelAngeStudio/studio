//! Window abstraction and properties

use cfg_boost::target_cfg;

use crate::display::DisplayError;
use crate::error::StudioError;

use super::event::Event;
use super::manager::WindowManager;
use super::manager::WindowManagerType;
use super::manager::WindowProvider;
use super::property::FullScreenMode;
use super::property::KeyboardPropertySet;
use super::property::PointerMode;
use super::property::PointerPropertySet;
use super::property::WindowPositionOption;
use super::property::WindowProperty;
use super::property::WindowPropertySet;

/// Enumeration of possible option of [Window::show()]
pub enum WindowShowOption {
    /// Show a window normally.
    Normal,

    /// Show a window that is always on top of parent.
    Top,

    /// Show a window that is always on top of parent and prevent any input event on parent.
    /// This option requires a parent.
    Modal,
}

/// Window wrapping a WindowManager.
/// 
/// Steps :
/// new()
/// set_properties
/// show()
pub struct Window {

    /// [WindowManager] of this window
    pub(crate) manager : WindowManagerType,

    /// Count of sub windows
    pub(crate) sub_count : usize,

    /// Modal SubWindow opened
    pub(crate) modal_opened : bool,
}

impl Window {

    /// Create a new [Window] wrapped in a reference cell and reference counter.
    /// 
    /// Returns Ok(Rc<RefCell<[Window]>>) on success, Err([StudioError]) on error.
    pub fn new() -> Result<Window, StudioError> {

        match WindowManagerType::new() {
            
            Ok(wm) => {
               Ok(Window { 
                    manager: wm,
                    sub_count: 0,
                    modal_opened: false, })
            },
            Err(err) => Err(err),
        }        
    }

    /// Get the window provider id
    pub fn get_window_provider(&self) -> WindowProvider{
        self.manager.get_window_provider()
    }

    target_cfg! {
        !immediate:ft => { // Retained mode
            /// Show the window according to options and parent. If parent is [Option::None],
            /// the window is showed on the root window (AKA desktop).
            /// 
            /// Returns Ok(true) on success, [StudioError] on failure.
            /// 
            /// Note : By default, new windows are hidden and .show() must be called.
            /// 
            /// Errors(s)
            /// [DisplayError::ParentSameAsSub] if parent is same as sub window.
            /// [DisplayError::ModalRequiresParent] if trying to make a modal window without parent.
            /// [DisplayError::SubHasChild] if trying to make a parent window a sub window.
            /// [DisplayError::WindowAlreadyOpened] if window is already opened.
            /// [DisplayError::ParentNotOpened] if parent window is not opened.
            /// [DisplayError::ParentIsLockedByModal] if parent window is locked by a modal.
            pub fn show(&mut self, option : WindowShowOption, parent : Option<&Window>) -> Result<bool, StudioError> {

                // Verify for possible errors
                match &parent {
                    Some(p) => {
                        if !p.get_properties().created {    // Verify that parent is opened.
                            return Err(StudioError::Display(DisplayError::ParentNotOpened));
                        }

                        if *p as *const _ == self as *const _ { // Verify that parent is not the same as sub.
                            return Err(StudioError::Display(DisplayError::ParentSameAsSub));
                        }

                        if p.modal_opened {  // Verify that parent is not locked by a modal window.
                            return Err(StudioError::Display(DisplayError::ParentIsLockedByModal));
                        }
                    },
                    None => {
                        if let WindowShowOption::Modal = option {   // Modal window requires parent.
                            return Err(StudioError::Display(DisplayError::ModalRequiresParent));
                        }
                    },
                }

                if self.sub_count > 0 { // Parent can't become subwindow on their own.
                    return Err(StudioError::Display(DisplayError::SubHasChild));
                }
                
                if !self.manager.get_properties().created { // Only if not created
                    self.manager.show(option, parent);
                    Ok(true)
                } else {
                    Err(StudioError::Display(DisplayError::WindowAlreadyOpened))
                }

            }
        }, 
        immediate:ft => {          // Immediate mode
            /// Show the window in immediate mode.
            /// 
            /// Returns Ok(true) on success, [StudioError] on failure.
            /// 
            /// Note : By default, new windows are hidden and .show() must be called.
            /// 
            /// Errors(s)
            /// [DisplayError::WindowAlreadyOpened] if window is already opened.
            pub fn show(&mut self, option : WindowShowOption, parent : Option<&Window>) -> Result<bool, StudioError> {
                
                if !self.manager.get_properties().created { // Only if not created
                    self.manager.show();
                    Ok(true)
                } else {
                    Err(StudioError::Display(DisplayError::WindowAlreadyOpened))
                }

            }
        }
    }

    
 
    /// Force close the window. The window ressources are freed and cannot receive events.
    /// 
    /// Returns Ok(true) on success, [StudioError] on failure.
    /// 
    /// Error(s)
    /// [DisplayError::WindowNotOpened] if window was not showed prior to closing.
    pub fn close(&mut self) -> Result<bool, StudioError> {

        if self.manager.get_properties().created {
            self.manager.close();
            Ok(true)
        } else {
            Err(StudioError::Display(DisplayError::WindowNotOpened))
        }
    }

    target_cfg! {
        !immediate:ft => {  // Retained mode
            /// Pop a window event from the queue. Event will be handle by window before returning it.
            pub fn poll_event(&mut self) -> Event{
                let event = self.manager.poll_event();
                self.handle_window_events(event)
            }
        },
        immediate:ft => {   // Immediate mode
            /// Pop a window event from the queue.
            pub fn poll_event(&mut self) -> &Event{
                self.manager.poll_event()
            }
        }
    }

    /// Get window properties in a read only struct.
    pub fn get_properties(&self) -> &WindowProperty {
        &self.manager.get_properties()
    }

    /// Set a window property according to family.
    /// 
    /// Return Ok() with the count of property changed on success, Err(StudioError) on failure.
    pub fn set_property(&mut self, property : &WindowPropertySet) -> Result<usize, StudioError>{

        if let Err(err) = self.set_window_property(property) {
            Err(err)
        } else {
            self.manager.refresh(); // Refresh window properties.
            Ok(1)
        }
        
    }

    /// Set multiple window properties from an array. When setting multiple properties, this function 
    /// is faster because some property need to recreate window. Here this will be done only one time
    /// if window recreate is needed.
    /// 
    /// Returns Ok() the count of properties changed on success, Err(WindowPropertySet, StudioError) on failure.
    pub fn set_properties(&mut self, properties : &[WindowPropertySet]) -> Result<usize, StudioError>{

        for property in properties {
            if let Err(err) = self.set_window_property(property) {
                return Err(err);
            }
        }

        self.manager.refresh(); // Refresh window properties.
        Ok(properties.len())
    }



    /// Handle property changes
    #[inline(always)]
    pub(in super::super) fn set_window_property(&mut self, property : &WindowPropertySet) -> Result<bool, StudioError> {
 
         match property {
            WindowPropertySet::Size(size) => return self.set_size(size),

            WindowPropertySet::Title(title) => self.set_title(title),
            WindowPropertySet::Position(option) => self.set_position(option),
            WindowPropertySet::ShowDecoration => self.show_decoration(),
            WindowPropertySet::HideDecoration => self.hide_decoration(),
            WindowPropertySet::Minimize => self.minimize(),
            WindowPropertySet::Maximized => self.maximize(),
            WindowPropertySet::Fullscreen(fsmode) => self.set_fullscreen(fsmode.clone()),
            WindowPropertySet::Restore => self.restore(),
            WindowPropertySet::Keyboard(kb_property) => self.set_keyboard_property(kb_property),
            WindowPropertySet::Pointer(p_property) => self.set_pointer_property(p_property),
         }

         Ok(true)

    }

    #[inline(always)]
    fn set_title(&mut self, title : &String){
        self.manager.set_title(title);
    }

    #[inline(always)]
    fn set_position(&mut self, option: &WindowPositionOption){
        self.manager.set_position(option.clone());
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32))  -> Result<bool, StudioError>{
        if WindowProperty::is_size_within_boundaries(size){
            self.manager.set_size(size);
            Ok(true)
        } else {    // Size incorrect.
            return Err(StudioError::Display(DisplayError::SizeError))
        }
    }

    #[inline(always)]
    fn show_decoration(&mut self){
        self.manager.show_decoration();
    }

    #[inline(always)]
    fn hide_decoration(&mut self){
        self.manager.hide_decoration();
    }

    #[inline(always)]
    fn minimize(&mut self){
        self.manager.minimize();
    }

    #[inline(always)]
    fn maximize(&mut self){
        self.manager.maximize();
    }

    #[inline(always)]
    fn set_fullscreen(&mut self, fsmode : FullScreenMode){
        self.manager.set_fullscreen(fsmode);
    }

    #[inline(always)]
    fn restore(&mut self){
        self.manager.restore();
    }

    #[inline(always)]
    fn set_keyboard_property(&mut self, property : &KeyboardPropertySet){
        match property {
            KeyboardPropertySet::SetMode(mode) => self.manager.set_keyboard_mode(*mode),
            KeyboardPropertySet::EnableAutoRepeat => self.manager.set_keyboard_auto_repeat(true),
            KeyboardPropertySet::DisableAutoRepeat => self.manager.set_keyboard_auto_repeat(false),
        }
    }

    #[inline(always)]
    fn set_pointer_property(&mut self, property : &PointerPropertySet){
        match property {
            PointerPropertySet::Mode(mode) => self.set_pointer_mode(mode),
            PointerPropertySet::Position(position) => self.set_pointer_position(*position),
            PointerPropertySet::Show => self.show_pointer(),
            PointerPropertySet::Hide => self.hide_pointer(),
            PointerPropertySet::Confine => self.confine_pointer(),
            PointerPropertySet::Release => self.release_pointer(),
        }
    }

    #[inline(always)]
    fn set_pointer_mode(&mut self, mode : &PointerMode){
        self.manager.set_pointer_mode(mode);
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32, i32)){
        self.manager.set_pointer_position(position);
    }

    #[inline(always)]
    fn show_pointer(&mut self){
        if !self.get_properties().pointer.visible {
            self.manager.show_pointer();
        } 
    }

    #[inline(always)]
    fn hide_pointer(&mut self){
        if self.get_properties().pointer.visible {
            self.manager.hide_pointer();
        }
    }

    #[inline(always)]
    fn confine_pointer(&mut self){
        if !self.get_properties().pointer.confined {
            self.manager.confine_pointer();
        }
    }

    #[inline(always)]
    fn release_pointer(&mut self){
        if self.get_properties().pointer.confined {
            self.manager.release_pointer();
        }
    }

    target_cfg! {
        !immediate:ft => {  // Retained mode
            /// Handle events targeted to this window.
            /// 
            /// If consumed, will poll the next event.
            #[inline(always)]
            fn handle_window_events(&mut self, event : Event) -> Event {
                match &event{
                    Event::SubWindow(sub_event) => {
                        match sub_event {
                            super::event::sub::EventSubWindow::SubAdded => self.sub_count +=1,
                            super::event::sub::EventSubWindow::SubRemoved => self.sub_count -=1,
                            super::event::sub::EventSubWindow::SubModalOpened => self.modal_opened = true,
                            super::event::sub::EventSubWindow::SubModalClosed => self.modal_opened = false,
                        }
                        self.poll_event()   // Event is consumed and next event is polled.
                    },
                    Event::None => event,   // None event are always returned.
                    _ => {  // All other events are only given if a modal window is not opened.
                        if self.modal_opened {
                            self.poll_event()   // Event is ignored when modal is opened and next event is polled.
                        } else {
                            event   // Event is returned.
                        }
                    }
                }
            }
        }
    }
    
}