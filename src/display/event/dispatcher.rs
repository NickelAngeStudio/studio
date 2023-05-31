/*
use std::{rc::Rc, cell::RefCell};
use crate::{error::StudioError, display::error::DisplayError};

use super::WindowEvent;

/// [DisplayEventDispatcher] dispatch [Display] [DisplayEvent] to [DisplayEventReceiver].
/// 
/// [DisplayEventDispatcher::dispatch()]  from the most recent added [DisplayEventReceiver] to the last, like a [Stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)).
/// That means that most recent [DisplayEventReceiver] can mask events for older [DisplayEventReceiver] if [DisplayEventReceiver::handle_event()] returns True. 
pub struct WindowEventDispatcher {
    /// List of [DisplayEventReceiver].
    receivers : Vec<Rc<RefCell<dyn WindowEventReceiver>>>,

    /// If true, [DisplayEventDispatcher] will log unhandled events. Make sure to clear_unhandled_events() once in a while to prevent memory overload.
    log_unhandled : bool,

    /// List of unhandled [DisplayEvent]
    unhandled : Vec<WindowEvent>,
}

impl WindowEventDispatcher {
    /// Create a new [DisplayEventDispatcher] used to dispatch [DisplayEvent] to [DisplayEventReceiver] with possibility to log unhandled event.
    /// 
    /// If log_unhandled_event is True, make sure to clear_unhandled_events() once in a while to prevent memory overload.
    pub fn new(log_unhandled_event : bool) -> WindowEventDispatcher {
        WindowEventDispatcher { receivers: Vec::new(), log_unhandled: log_unhandled_event, unhandled: Vec::new() }
    }

    /// Dispatch a [DisplayEvent] to the [DisplayEventReceiver] list.
    /// 
    /// [DisplayEvent] dispatch from the most recent added [DisplayEventReceiver] to the last, like a [Stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)).
    /// That means that most recent [DisplayEventReceiver] can mask events for older [DisplayEventReceiver] if [DisplayEventReceiver::handle_event()] returns True.
    ///
    /// Returns True if the [DisplayEvent] was handled by a [DisplayEventReceiver], false otherwise.
    pub fn dispatch(&mut self, event : &WindowEvent) -> bool {
        // Iterate enabled receivers from newest to oldest
        for receiver in self.receivers.iter().rev().filter(|x| x.borrow().is_enabled() ) {          
            let mut receiver = receiver.borrow_mut();
            if receiver.handle_event(&event) {
                // Event has been handled, 
                return true
            }
        }
        if self.log_unhandled {
            // Copy event in unhandled.
            self.unhandled.push(*event);
        }
        false
    }

    /// Clear all logged unhandled [DisplayEvent].
    pub fn clear_unhandled_events(&mut self){
        self.unhandled.clear();
    }


    /// Get a immutable reference to the list of unhandled [DisplayEvent].
    pub fn get_unhandled_events(&self) -> &Vec<WindowEvent>{
        &self.unhandled
    }

    /// Add a [DisplayEventReceiver] to the [DisplayEventDispatcher] that will receive [DisplayEvent] dispatched.
    /// 
    /// [DisplayEventReceiver] must be wrapped in [Rc] [RefCell] since [Display::dispatch_events()] is MUTABLE.
    /// 
    /// Returns [OK(usize)][Ok] with index of new receiver added.
    /// 
    /// # Example(s)
    /// ```no_run
    /// // Importing RC and Refcell modules
    /// use std::{cell::RefCell, rc::Rc};
    /// 
    /// ... impl [DisplayEventReceiver] for MyReceiver { ... } ...
    /// 
    /// // Create variable for MyReceiver 
    /// let myr = Rc::new(RefCell::new(MyReceiver::new()));
    /// 
    /// // Clone MyReceiver variable when adding to Display
    /// my_display.add_event_receiver(myr.clone());
    /// 
    /// ```
    /// 
    /// # Error(s)
    /// Returns `Err(`[DisplayError::ReceiverAlreadyExists]`)` if receiver is already in list.
    /// 
    /// # Note(s)
    /// [DisplayEvent] dispatch from the most recent added [DisplayEventReceiver] to the older, like a [Stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)).
    /// That means that most recent [DisplayEventReceiver] can mask events for older [DisplayEventReceiver].
    pub fn add_event_receiver(&mut self, receiver : Rc<RefCell<dyn WindowEventReceiver>>) -> Result<usize, StudioError> {

        match self.get_receiver_index(receiver.clone()) {
            Ok(_) => Err(StudioError::Display(DisplayError::ReceiverAlreadyExists)),
            Err(_) => { self.receivers.push(receiver.clone()); Ok(self.receivers.len() - 1) },
        }

    }

    /// Remove a [DisplayEventReceiver] from the [Display] list.
    /// 
    /// Returns [OK(usize)][Ok] with index of receiver removed.
    /// 
    /// # Error(s)
    /// Returns `Err(`[StudioError::DisplayEventDispatcher(DisplayEventDispatcherError::ReceiverNotFound)]`)` if receiver not found.
    pub fn remove_event_receiver(&mut self, receiver : Rc<RefCell<dyn WindowEventReceiver>>) -> Result<usize, StudioError> {
        
        match self.get_receiver_index(receiver.clone()) {
            Ok(index) => { self.receivers.remove(index); Ok(index) },
            Err(_) => Err(StudioError::Display(DisplayError::ReceiverNotFound)),
        }

    }

    
    /// Returns the index of a receiver from the list.
    /// 
    /// # Error(s)
    /// Returns `Err(`[StudioError::DisplayEventDispatcher(DisplayEventDispatcherError::ReceiverNotFound)]`)` if receiver not found.
    fn get_receiver_index(&self, receiver : Rc<RefCell<dyn WindowEventReceiver>>)-> Result<usize, StudioError> {
        let mut found = false;
        let mut index: usize = 0;

        for i in 0..self.receivers.len() {
            if std::ptr::eq(receiver.as_ptr(), self.receivers[i].as_ptr()) {
                found = true;
                index = i;
                break;
            }
        }
        
        if found {
            Ok(index)
        }
        else {
            Err(StudioError::Display(DisplayError::ReceiverNotFound))
        }
    }

}


/// Receive [DisplayEvent] from [Display] and handle them if needed. 
/// 
/// The function [DisplayEventReceiver::handle_event()] has a mutable reference to self allowing
/// modification of object that implement [DisplayEventReceiver].
pub trait WindowEventReceiver {

    /// Handle a [DisplayEvent] received from the dispatcher.
    /// 
    /// Return True if the [DisplayEvent] has been handled, which will prevent other receiver from handling it.
    /// Return False if the [DisplayEvent] wasn't handled, giving it to the next receiver.
    fn handle_event(&mut self, event : &WindowEvent) -> bool;

    /// Returns if [DisplayEventReceiver] is enabled and ready to receive [DisplayEvent].
    /// 
    /// If False, the [DisplayEventReceiver] will NOT receive [DisplayEvent].
    fn is_enabled(&self) -> bool;
}
*/