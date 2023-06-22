//! Window abstraction and properties

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
use super::property::SubWindowOption;
use super::property::WindowEventWaitMode;
use super::property::WindowPositionOption;
use super::property::WindowProperty;
use super::property::WindowPropertySet;

/// Window wrapping a WindowManager.
/// 
/// Steps :
/// new()
/// set_properties
/// show()
pub struct Window<'window> {

    /// [WindowManager] of this window
    pub(crate) manager : WindowManagerType<'window>,
}

impl<'window> Window<'window> {

    /// Create a new [Window] wrapped in a reference cell and reference counter.
    /// 
    /// Returns Ok(Rc<RefCell<[Window]>>) on success, Err([StudioError]) on error.
    pub fn new() -> Result<Window<'window>, StudioError> {

        match WindowManagerType::new() {
            
            Ok(wm) => {
               Ok(Window { 
                    manager: wm })
            },
            Err(err) => Err(err),
        }        
    }

    /// Get the window provider id
    pub fn get_window_provider(&self) -> WindowProvider{
        self.manager.get_window_provider()
    }

    /// Show the window. By default, new windows are hidden and .show() must be called.
    pub fn show(&mut self) {
        if !self.manager.get_properties().visible { // Only if not visible
            self.manager.show()
        }
    }

    // Hide the window. The window ressources are not freed and can still receive events.
    pub fn hide(&mut self){
        if self.manager.get_properties().visible {  // Only if visible
            self.manager.hide();
        }
    }
 
    // Force close the window. The window ressources are freed and cannot receive events.
    pub fn close(&mut self){
        if self.manager.get_properties().created {
            self.manager.close();
        }
    }

    /// Pop a window event from the queue.
    pub fn poll_event(&mut self) -> &Event{
        self.manager.poll_event() 
    }

    /// Get window properties in a read only struct.
    pub fn get_properties(&self) -> &WindowProperty {
        &self.manager.get_properties()
    }

    /// Set a window property according to family.
    /// 
    /// Return Ok() with the count of property changed on success, Err(StudioError) on failure.
    pub fn set_property(&mut self, property : &'window WindowPropertySet<'window>) -> Result<usize, StudioError>{

        match self.set_window_property(property){
            Ok(need_recreate) => {
                if need_recreate {
                    self.manager.recreate();
                }
                Ok(1)
            },
            Err(err) => return Err(err),   
        }
        
    }

    /// Set multiple window properties from an array. When setting multiple properties, this function 
    /// is faster because some property need to recreate window. Here this will be done only one time
    /// if window recreate is needed.
    /// 
    /// Returns Ok() the count of properties changed on success, Err(WindowPropertySet, StudioError) on failure.
    pub fn set_properties(&mut self, properties : &'window [WindowPropertySet]) -> Result<usize, StudioError>{

        // Flag that indicate window need to be recreated
        let mut recreate_window = false;

        // Count of properties changed
        let mut count : usize = 0;

        for property in properties {
            match self.set_window_property(property){
                Ok(need_recreate) => {
                    // If one recreate needed, recreate_window will become true.
                    recreate_window = recreate_window || need_recreate;

                    // Increment property changes
                    count += 1;
                },
                Err(err) => return Err(err),   
            }
        }

        if recreate_window {
            self.manager.recreate();
        }

        Ok(count)
    }



    /// Handle property changes
    #[inline(always)]
    pub(in super::super) fn set_window_property(&mut self, property : &'window WindowPropertySet) -> Result<bool, StudioError> {
 
         match property {
            WindowPropertySet::SetParent(parent, option) => self.set_parent(parent, *option),
            WindowPropertySet::SetEventWaitMode(mode) => self.set_event_wait_mode(*mode),
            WindowPropertySet::RemoveParent => todo!(),
            WindowPropertySet::Title(title) => self.set_title(title),
            WindowPropertySet::Position(option) => self.set_position(option),
            WindowPropertySet::Size(size) => self.set_size(size),
            WindowPropertySet::ShowDecoration => self.show_decoration(),
            WindowPropertySet::HideDecoration => self.hide_decoration(),
            WindowPropertySet::Minimize => self.minimize(),
            WindowPropertySet::Maximized => self.maximize(),
            WindowPropertySet::Fullscreen(fsmode) => self.set_fullscreen(fsmode.clone()),
            WindowPropertySet::Restore => self.restore(),
            WindowPropertySet::Keyboard(kb_property) => self.set_keyboard_property(kb_property),
            WindowPropertySet::Pointer(p_property) => self.set_pointer_property(p_property),
         }
    }

    /// Set the window parent wrapped in [Rc][RefCell] to a subwindow wrapped in [Rc][RefCell]. 
    /// Sub window are showed according to [SubWindowOption].
    /// When closing a parent, all sub window must also be closed.
    /// 
    /// Note : A window cannot be it's own parent nor can it become the subwindow of his subwindows.
    #[inline(always)]
    pub fn set_parent(&mut self, parent : &'window Window, option : SubWindowOption) -> Result<bool, StudioError>{

        if  self as * const _ == parent as * const _ {   // Make sure parent and child aren't the same.
            Err(StudioError::Display(crate::display::DisplayError::ParentSameAsSub))
        } else if self.is_self_parent_of_parent(parent) {  // Make sure parent wasn't a child of the parent.
            Err(StudioError::Display(crate::display::DisplayError::ParentIsParent))
        } else {
            match self.remove_parent(){ // Remove current parent
                Ok(_) => {
                    // Set new parent reference with option.
                    Ok(self.manager.set_parent(parent, option))
                },
                Err(err) => Err(err),
            }
        }
        
    }

    /// Remove parent from a Subwindow. Parent function are statics since they needs
    /// an [Rc][RefCell] and not [self].
    #[inline(always)]
    pub fn remove_parent(&mut self) -> Result<bool, StudioError>{

        match self.get_properties().parent {
            Some(parent) => {   // If old parent is locked, raise error
                // If locked, return Err(ParentIsLocked)
                if parent.0.get_properties().locked {
                    Err(StudioError::Display(crate::display::DisplayError::ParentIsLocked))
                } else {
                    // Remove window parent.
                    Ok(self.manager.remove_parent())
                }
            },
            None => Ok(false), // Nothing had to be done, no parent removed.
        }
        
    }

    /// Verify if future parent is a child of current parent.
    /// 
    /// Return true if is a child, false otherwise.
    #[inline(always)]
    fn is_self_parent_of_parent(&'window self, parent : &Window) -> bool {
        let mut is_parent = false;

        let mut target = parent;
        loop {
            match target.get_properties().parent {
                Some(parent) => {
                    if self as * const _ == parent.0 as * const _ {
                        is_parent = true;
                        break;  // Break since self is parent of parent.
                    } else {
                        target = parent.0;
                    }
                },
                None => break,  // Root reached
            }
        }

        is_parent
    }

    #[inline(always)]
    fn set_event_wait_mode(&mut self, mode : WindowEventWaitMode) -> Result<bool, StudioError>{
        Ok(self.manager.set_event_wait_mode(mode))
    }


    #[inline(always)]
    fn set_title(&mut self, title : &String) -> Result<bool, StudioError>{
        Ok(self.manager.set_title(title))
    }

    #[inline(always)]
    fn set_position(&mut self, option: &WindowPositionOption) -> Result<bool, StudioError>{
        Ok(self.manager.set_position(option.clone()))
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32))  -> Result<bool, StudioError>{
        if WindowProperty::is_size_within_boundaries(size){
            Ok(self.manager.set_size(size))
        } else {    // Size incorrect.
            return Err(StudioError::Display(DisplayError::SizeError))
        }
    }

    #[inline(always)]
    fn show_decoration(&mut self) -> Result<bool, StudioError>{
        Ok(self.manager.show_decoration())
    }

    #[inline(always)]
    fn hide_decoration(&mut self) -> Result<bool, StudioError>{
        Ok(self.manager.hide_decoration())
    }

    #[inline(always)]
    fn minimize(&mut self) -> Result<bool, StudioError>{
        Ok(self.manager.minimize())
    }

    #[inline(always)]
    fn maximize(&mut self) -> Result<bool, StudioError>{
        Ok(self.manager.maximize())
    }

    #[inline(always)]
    fn set_fullscreen(&mut self, fsmode : FullScreenMode) -> Result<bool, StudioError>{
        Ok(self.manager.set_fullscreen(fsmode))
    }

    #[inline(always)]
    fn restore(&mut self) -> Result<bool, StudioError>{
        self.manager.restore();
        Ok(true)
    }

    #[inline(always)]
    fn set_keyboard_property(&mut self, property : &KeyboardPropertySet) -> Result<bool, StudioError>{
        match property {
            KeyboardPropertySet::SetMode(mode) => Ok(self.manager.set_keyboard_mode(*mode)),
        }
    }

    #[inline(always)]
    fn set_pointer_property(&mut self, property : &PointerPropertySet) -> Result<bool, StudioError>{
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
    fn set_pointer_mode(&mut self, mode : &PointerMode) -> Result<bool, StudioError>{
        Ok(self.manager.set_pointer_mode(mode))
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32, i32)) -> Result<bool, StudioError>{
        Ok(self.manager.set_pointer_position(position))
    }

    #[inline(always)]
    fn show_pointer(&mut self) -> Result<bool, StudioError>{
        if !self.get_properties().pointer.visible {
            Ok(self.manager.show_pointer())
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn hide_pointer(&mut self) -> Result<bool, StudioError>{
        if self.get_properties().pointer.visible {
            Ok(self.manager.hide_pointer())
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn confine_pointer(&mut self) -> Result<bool, StudioError>{
        if !self.get_properties().pointer.confined {
            Ok(self.manager.confine_pointer())
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn release_pointer(&mut self)-> Result<bool, StudioError>{
        if self.get_properties().pointer.confined {
            Ok(self.manager.release_pointer())
        } else {
            Ok(false)
        }
    }
    
}