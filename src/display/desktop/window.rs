//! Window abstraction and properties

target_cfg! {
    linux => {
        pub type WindowManagerType = super::provider::linux::LinuxWindowManager;
    }
}


use std::cell::{RefCell};
use std::rc::Rc;

use cfg_boost::{ target_cfg};

use crate::display::DisplayError;
use crate::error::StudioError;


use super::manager::WindowManager;
use super::property::{WindowPropertySet, WindowProperty, WindowPositionOption, SubWindowOption, KeyboardPropertySet, FullScreenMode, PointerPropertySet, PointerMode};
use super::event::{Event, EventWindow, EventMouse};
use super::provider::WindowProvider;

/// Window with a manager.
/// 
/// Steps :
/// new()
/// set_properties
/// show()
pub struct Window {

    /// [WindowManager] of this window
    manager : WindowManagerType,

    /// [Window] properties
    pub(super) property : WindowProperty,

    /// Self contained Rc Ref
    refcell : Option<Rc<RefCell<Window>>>,
}


impl Window {

    /// Create a new [Window] wrapped in a reference cell and reference counter.
    /// 
    /// Returns Ok(Rc<RefCell<[Window]>>) on success, Err([StudioError]) on error.
    pub fn new() -> Result<Rc<RefCell<Window>>, StudioError> {

        match WindowManagerType::new() {
            
            Ok(wm) => {
                let window = Rc::new(RefCell::new(Window { 
                    manager: wm, 
                    property: WindowProperty::new(),
                    refcell: None }));


                // Add self Rc reference to self.
                window.borrow_mut().refcell = Some(window.clone());

                Ok(window)
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
        if !self.property.visible { // Only if not visible
            self.manager.show(&self.property);
            self.property.visible = true;
        }
    }

    // Hide the window. The window ressources are not freed and can still receive events.
    pub fn hide(&mut self){
        if self.property.visible {  // Only if visible
            self.manager.hide();
            self.property.visible = false;
        }
    }
 
    // Force close the window. The window ressources are freed and cannot receive events.
    pub fn close(&mut self){
        if self.property.created {
            self.manager.close();
            self.property.created = false;
        }
    }

    /// Pop a window event from the queue.
    pub fn poll_event(&mut self) -> Event{
        
        let event = self.manager.poll_event();
        match event {
            Event::Window(w_event) => self.handle_window_event(event, w_event),
            Event::Mouse(m_event) => self.handle_mouse_event(event, m_event),
            _ => event,
        }       

    }

    /// Get window properties in a read only struct.
    pub fn get_properties(&self) -> &WindowProperty {
        &self.property
    }

    /// Set a window property according to family.
    /// 
    /// Return Ok() with the count of property changed on success, Err(StudioError) on failure.
    pub fn set_property(&mut self, property : WindowPropertySet) -> Result<usize, StudioError>{
        self.set_properties(&[property])
    }

    /// Set multiple window properties from an array. When setting multiple properties, this function 
    /// is faster because some property need to recreate window. Here this will be done only one time
    /// if window recreate is needed.
    /// 
    /// Returns Ok() the count of properties changed on success, Err(WindowPropertySet, StudioError) on failure.
    pub fn set_properties(&mut self, properties : &[WindowPropertySet]) -> Result<usize, StudioError>{

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

        if recreate_window && self.property.created {   // If window is created, recreate window
            self.close();   // Close window to destroy
            self.show();    // Show window to create
        }

        Ok(count)
    }

    /// Get the OS Window manager window handle.
    pub fn get_window_handle(&self) -> Option<*const usize>{
        self.manager.get_window_handle()
    }

    target_cfg! {
        linux => {
            /// Get the OS Window manager display handle.
            pub fn get_display_handle(&self) -> Option<*const usize>{
                self.manager.get_display_handle()
            }
        }
    }

    /// Handle [EventWindow] before sending it to client.
    #[inline(always)]
    fn handle_window_event(&mut self, event : Event, w_event : EventWindow) -> Event {
        match w_event {
            EventWindow::Closed => {  // Window has been closed.
                match &self.property.parent {
                    Some(parent) => {
                        // Remove child from parent list
                        parent.borrow_mut().property.remove_sub(self.get_refcell());
                        self.property.parent=None;
                    },
                    None => todo!(),
                }
            }
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
            _ => {},
        }

        event
    }

    /// Handle [EventMouse] before sending it to client.
    #[inline(always)]
    fn handle_mouse_event(&mut self, event : Event, m_event : EventMouse) -> Event {
        match m_event {
            EventMouse::Moved(position) => {
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
                                Event::Mouse(EventMouse::Acceleration(position))
                            } else {
                                self.poll_event()   // Ignore and poll next event
                            }
                        },     
                }
            },
            _ => event,
        }
    }


    /// Handle property changes
    #[inline(always)]
    fn set_window_property(&mut self, property : &WindowPropertySet) -> Result<bool, StudioError> {
 
         match property {
            WindowPropertySet::SetParent(parent, option) => self.set_parent(parent.clone(), option),
            WindowPropertySet::RemoveParent =>  self.remove_parent(),
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

    /// Small shortcut to get RefCell without consuming it.
    fn get_refcell(&self) -> Rc<RefCell<Window>> {
        match &self.refcell {
            Some(rc) => rc.clone(),
            None => panic!(""),
        }
    }

    #[inline(always)]
    fn set_parent(&mut self, parent : Rc<RefCell<Window>>, option : &SubWindowOption) -> Result<bool, StudioError>{

        if  Rc::ptr_eq(&self.get_refcell(), &parent) {   // Make sure parent and child aren't the same.
            Err(StudioError::Display(crate::display::DisplayError::ParentSameAsSub))
        } else if self.is_parent_sub_of_self(parent.clone()) {  // Make sure parent wasn't a child of the parent.
            Err(StudioError::Display(crate::display::DisplayError::ParentIsSubOfWindow))
        } else {
            match self.remove_parent(){ // Remove current parent
                Ok(_) => {
                    // If remove successful, add to new parent.
                    parent.borrow_mut().property.add_sub(self.get_refcell());

                    // Set new parent reference.
                    self.property.parent = Some(parent);

                    // Set self subwindow option
                    self.property.subwindow_option = Some(*option);

                    // Return parent changed
                    Ok(true)
                },
                Err(err) => Err(err),
            }
        }
        
    }

    #[inline(always)]
    fn remove_parent(&mut self) -> Result<bool, StudioError>{

        match &self.property.parent {
            Some(old_parent) => {   // If old parent is locked, raise error
                // If locked, return Err(ParentIsLocked)
                if old_parent.borrow().get_properties().locked {
                    Err(StudioError::Display(crate::display::DisplayError::ParentIsLocked))
                } else {

                    // Remove sub window from parent.
                    old_parent.borrow_mut().property.remove_sub(self.get_refcell());

                    // Remove window parent.
                    self.property.parent = Option::None;

                    // Remove show option
                    self.property.subwindow_option = Option::None;

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
    fn is_parent_sub_of_self(&self, parent : Rc<RefCell<Window>>) -> bool {
        let mut is_parent = false;

        self.property.subs.iter().for_each(|sub| 
            if Rc::ptr_eq(sub, &parent) {
                is_parent = true;
            } else {
                is_parent = is_parent || sub.borrow().is_parent_sub_of_self(parent.clone());     
            }
        );

        is_parent
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String) -> Result<bool, StudioError>{
        Ok(self.manager.set_title(title))
    }

    #[inline(always)]
    fn set_position(&mut self, option: &WindowPositionOption) -> Result<bool, StudioError>{
        match  self.property.get_absolute_position_from_relative(option){
            Ok(position) => {
                self.property.relative_position = option.clone();
                Ok(self.manager.set_position(position))
            },
            Err(err) => return Err(err),
        }
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
        self.property.decoration = true;
        Ok(self.manager.show_decoration())
    }

    #[inline(always)]
    fn hide_decoration(&mut self) -> Result<bool, StudioError>{
        self.property.decoration = false;
        Ok(self.manager.hide_decoration())
    }

    #[inline(always)]
    fn minimize(&mut self) -> Result<bool, StudioError>{
        self.property.minimized = true;
        self.property.maximized = false;
        self.property.fullscreen = None;
        Ok(self.manager.minimize())
    }

    #[inline(always)]
    fn maximize(&mut self) -> Result<bool, StudioError>{
        self.property.minimized = false;
        self.property.maximized = true;
        self.property.fullscreen = None;
        Ok(self.manager.maximize())
    }

    #[inline(always)]
    fn set_fullscreen(&mut self, fsmode : FullScreenMode) -> Result<bool, StudioError>{
        self.property.fullscreen = Some(fsmode);
                
        // Full screen need to recreate window.
        Ok(self.manager.set_fullscreen())
    }

    #[inline(always)]
    fn restore(&mut self) -> Result<bool, StudioError>{
    
        self.property.minimized = false;
        self.property.maximized = false;
        self.property.fullscreen = None;

        Ok(true)
    }

    #[inline(always)]
    fn set_keyboard_property(&mut self, property : &KeyboardPropertySet) -> Result<bool, StudioError>{
        match property {
            KeyboardPropertySet::EnableAutoRepeat => self.enable_autorepeat(),
            KeyboardPropertySet::DisableAutoRepeat => self.disable_autorepeat(),
        }
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) -> Result<bool, StudioError>{
        self.property.keyboard.auto_repeat = true;
        Ok(self.manager.enable_autorepeat())
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) -> Result<bool, StudioError>{
        self.property.keyboard.auto_repeat = false;
        Ok(self.manager.disable_autorepeat())
    }

    #[inline(always)]
    fn set_pointer_property(&mut self, property : &PointerPropertySet) -> Result<bool, StudioError>{
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
    fn set_pointer_mode(&mut self, mode : &PointerMode) -> Result<bool, StudioError>{
        self.property.pointer.mode = mode.clone();

        match mode {
            PointerMode::Acceleration => {
                self.manager.set_pointer_position(&self.property.center);
            },
            _ => {},
        }

        Ok(false)

    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : &(i32, i32)) -> Result<bool, StudioError>{
        Ok(self.manager.set_pointer_position(position))
    }

    #[inline(always)]
    fn show_pointer(&mut self) -> Result<bool, StudioError>{
        if !self.property.pointer.visible {
            self.property.pointer.visible = true;
            Ok(self.manager.show_pointer())
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn hide_pointer(&mut self) -> Result<bool, StudioError>{
        if self.property.pointer.visible {
            self.property.pointer.visible = false;
            Ok(self.manager.hide_pointer())
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn confine_pointer(&mut self) -> Result<bool, StudioError>{
        if !self.property.pointer.confined {
            self.property.pointer.confined = true;
            Ok(self.manager.confine_pointer())
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn release_pointer(&mut self)-> Result<bool, StudioError>{
        if self.property.pointer.confined {
            self.property.pointer.confined = false;
            Ok(self.manager.release_pointer())
        } else {
            Ok(false)
        }
    }
}