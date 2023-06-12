use std::{cell::RefCell, ffi::CString, rc::Rc};

use crate::{display::desktop::{property::{WindowPropertySet, WindowPositionOption, FullScreenMode, KeyboardPropertySet, PointerPropertySet, PointerMode, WindowProperty, SubWindowOption}, provider::linux::LinuxWindow, window::Window}, error::StudioError};
use crate::display::desktop::window::WindowType;

use super::{X11Window, cbind::{constants::{PropModeReplace, GrabModeAsync, CurrentTime}}};
use super::cbind::functs::*;



impl X11Window {
    /// Handle property changes
    #[inline(always)]
    pub(crate) fn set_window_properties(&mut self, properties : &[WindowPropertySet]) -> Result<usize, (WindowPropertySet, StudioError)> {

        // Flag that indicate if window needs to be recreated.
        let mut recreate_window = false;

        // Count of properties changed
        let mut count : usize = 0;

        // For each property
        for property in properties {
            match self.set_window_property(property){
                Ok(need_recreate) => {
                    // If one recreate needed, recreate_window will become true.
                    recreate_window = recreate_window || need_recreate;

                    // Increment property changes
                    count += 1;
                },
                Err(err) => return Err((*property, err)),   // On error, return property and error 
            }
        }

        // Recreate window if needed
        if recreate_window {
            self.recreate_window();
        }

        Ok(count)

    }

    /// Handle property changes
    #[inline(always)]
    pub(crate) fn set_window_property(&mut self, property : &WindowPropertySet) -> Result<bool, StudioError> {

        // Flag that indicate if window need to be recreated.
        let mut recreate_window = false;

        match property{
            WindowPropertySet::SetParent(parent, option) => {
                match self.set_parent(parent, option) {
                    Ok(new_parent) =>  recreate_window = new_parent,               // If new parent, recreate window.
                    Err(err) => return Err(err),       // Parent change failed.
                }
            },
            WindowPropertySet::RemoveParent =>  match self.remove_parent() {
                Ok(had_parent) =>  recreate_window = had_parent,   // Parent removed if any, recreate window.
                Err(err) => return Err(err),                // Parent removal failed.
            },
            WindowPropertySet::Title(title) => self.set_title(&title),
            WindowPropertySet::Position(position) => match self.set_relative_position(position){
                Ok(_) => {},
                Err(err) => return Err(err),
            },
            WindowPropertySet::Size(size) => match self.set_size(size){
                Ok(_) => {},
                Err(err) => return Err(err),    // Size changed failed, return error.
            },
            WindowPropertySet::ShowDecoration => self.show_decoration(),
            WindowPropertySet::HideDecoration => self.hide_decoration(),
            WindowPropertySet::Minimize => {
                self.minimize();

                // Minimizing need to recreate window.
                recreate_window = true;
            },
            WindowPropertySet::Maximized => self.maximize(),
            WindowPropertySet::Fullscreen(fsmode) => {
                self.set_fullscreen(fsmode);

                // Full screen need to recreate window.
                recreate_window = true;
            },
            WindowPropertySet::Keyboard(kb_property) => self.set_keyboard_property(kb_property),
            WindowPropertySet::Pointer(p_property) => self.set_pointer_property(p_property),
            
            
            
        }

        Ok(recreate_window)
    }

    #[inline(always)]
    fn set_parent(&mut self, parent : &WindowType, option : &SubWindowOption) -> Result<bool, StudioError>{
        
        if Rc::ptr_eq(&parent.get_window_rcref(), &self.property.parent.unwrap().borrow().get_window_rcref()){  // If same parent, do nothing
            Ok(false)
        } else if Rc::ptr_eq(&parent.get_window_rcref(), &self.get_window_rcref()) {       // Make sure parent and child aren't the same.
            Err(StudioError::Display(crate::display::DisplayError::ParentSameAsSub))
        } else if self.is_parent_sub_of_self(parent) {                 // Make sure parent wasn't a child of the parent.
            Err(StudioError::Display(crate::display::DisplayError::ParentIsSubOfWindow))
        } else {
            match self.remove_parent(){ // Remove current parent
                Ok(_) => {
                    // If remove successful, add to new parent.
                    parent.get_window_rcref().borrow_mut().get_properties_mut().add_sub(self.get_window_rcref().clone());

                    // Set new parent reference.
                    self.property.parent = Some(parent.get_window_rcref().clone());

                    // Return parent changed
                    Ok(true)
                },
                Err(err) => Err(err),
            }
        }
    }

    /// Remove parent of window, making it parentless.
    /// 
    /// Returns True if a parent is removed, false otherwise.
    /// 
    /// Returns StudioError ParentIsLocked if parent is locked by modal.
    #[inline(always)]
    fn remove_parent(&mut self) -> Result<bool, StudioError>{
        match self.property.parent {
            Some(old_parent) => {   // If old parent is locked, raise error
                // If locked, return Err(ParentIsLocked)
                if old_parent.borrow().get_properties().locked {
                    Err(StudioError::Display(crate::display::DisplayError::ParentIsLocked))
                } else {

                    // Remove sub window from parent.
                    old_parent.borrow_mut().get_properties_mut().remove_sub(self.get_window_rcref());

                    // Remove window parent.
                    self.property.parent = Option::None;

                    // Return that parent was removed.
                    Ok(true)
                }
            },
            None => Ok(false), // Nothing had to be done, no parent removed.
        }
    }

    /// Verify if future parent is a sub of current parent window.
    /// 
    /// Return true if is a child, false otherwise.
    /// 
    /// Note : No inline since recursive.
    pub(crate) fn is_parent_sub_of_self(&self, parent : &WindowType) -> bool {

        let mut is_parent = false;
        
        for sub in self.get_properties().subs {

            if Rc::ptr_eq(&sub, &parent.get_window_rcref()){
                is_parent = true;
            } else {
                is_parent = is_parent || sub.borrow().x11.unwrap().is_parent_sub_of_self(parent);
            }

        }

        is_parent
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String){
        unsafe {
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
    }

    /// Set window position relatively to another element.
    #[inline(always)]
    fn set_relative_position(&mut self, option: &WindowPositionOption) -> Result<bool, StudioError>{

        // Update relative position option.
        self.property.relative_position = *option;

        match WindowPositionOption::get_absolute_position_from_relative(&self.get_window_rcref().clone().borrow(), option) {
            Ok(position) => {
                self.set_absolute_position(position);

                Ok(true)
            },
            Err(err) => Err(err),
        }
    }

    

    /// Set window absolute position
    #[inline(always)]
    fn set_absolute_position(&mut self, position : (i32,i32)) {
        unsafe {
            XMoveWindow(self.display, self.window, position.0, position.1);
        }
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32)) -> Result<bool, StudioError>{

        if WindowProperty::is_size_within_boundaries(size) {
            unsafe {
                // Keep real window position
                let position = X11Window::get_x11_window_position(self.display, self.window);
    
                XResizeWindow(self.display, self.window, size.0, size.1);
                
                // Reposition window since resize put it back at 0,0
                self.set_absolute_position(position);
    
                Ok(true)
            }
        } else {
            Err(StudioError::Display(crate::display::DisplayError::SizeError))
        }

    }

    #[inline(always)]
    fn show_decoration(&mut self){
        if !self.property.decoration {
            unsafe {
                XChangeProperty(self.display, self.window, self.atoms._MOTIF_WM_HINTS, self.atoms._MOTIF_WM_HINTS,
                    32, PropModeReplace, std::mem::transmute(&[2, 0, 0, 0, 0]), 5);
            }
            self.property.decoration = true;
        }
    }

    #[inline(always)]
    fn hide_decoration(&mut self){
        if self.property.decoration {
            unsafe {
                XChangeProperty(self.display, self.window, self.atoms._MOTIF_WM_HINTS, self.atoms._MOTIF_WM_HINTS,
                    32, PropModeReplace, std::mem::transmute(&[2, 0, 0, 0, 0]), 5);
            }
            self.property.decoration = false;
        }
    }

    #[inline(always)]
    fn minimize(&mut self){
        todo!()
    }

    #[inline(always)]
    fn maximize(&mut self){
        todo!()
    }

    #[inline(always)]
    fn set_fullscreen(&mut self, fsmode : &FullScreenMode){
        todo!()
    }

    #[inline(always)]
    fn set_keyboard_property(&mut self, property : &KeyboardPropertySet){
        match property {
            KeyboardPropertySet::EnableAutoRepeat => self.enable_autorepeat(),
            KeyboardPropertySet::DisableAutoRepeat => self.disable_autorepeat(),
        }
    }

    #[inline(always)]
    fn enable_autorepeat(&self){
        self.property.keyboard.auto_repeat = true;
    }

    #[inline(always)]
    fn disable_autorepeat(&self){
        self.property.keyboard.auto_repeat = false;
    }

    #[inline(always)]
    fn set_pointer_property(&mut self, property : &PointerPropertySet){
        match property {
            PointerPropertySet::Mode(mode) => self.set_pointer_mode(mode),
            PointerPropertySet::Position(position) => self.set_pointer_position(position),
            PointerPropertySet::Show => self.show_pointer(),
            PointerPropertySet::Hide => self.hide_pointer(),
            PointerPropertySet::Confine => self.confine_pointer(),
            PointerPropertySet::Release => self.release_pointer(),
        }
    }

    #[inline(always)]
    fn set_pointer_mode(&self, mode : &PointerMode){
        self.property.pointer.mode = *mode;

        match mode {
            // Set cursor to center if Acceleration
            PointerMode::Acceleration => self.set_pointer_position(&self.property.center),
            _ => {},
        }
    }

    #[inline(always)]
    fn set_pointer_position(&self, position : &(i32, i32)){
        unsafe {
            self.property.pointer.position = *position;
            XWarpPointer(self.display, self.window, self.window, 0, 0, 
                0, 0, position.0,  position.1);
        }
    }

    #[inline(always)]
    pub(crate) fn show_pointer(&self){
        unsafe {
            if !self.property.pointer.visible {    // Make sure X hide cursor was called prior to show.
                XFixesShowCursor(self.display, self.window);
                self.property.pointer.visible = true;
            }       
        }

    }

    #[inline(always)]
    pub(crate) fn hide_pointer(&self){
        unsafe {
            if self.property.pointer.visible {
                self.property.pointer.visible = false;
                XFixesHideCursor(self.display, self.window);
            }
        }

    }

    #[inline(always)]
    pub(crate) fn confine_pointer(&self){
        unsafe {
            self.property.pointer.visible = true;
            XGrabPointer(self.display, self.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.window, 0, CurrentTime);
        }

    }

    #[inline(always)]
    pub(crate) fn release_pointer(&self){
        unsafe {
            self.property.pointer.visible = false;
            XUngrabPointer(self.display, CurrentTime);
        }

    }

}