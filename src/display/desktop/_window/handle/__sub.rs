use crate::{display::desktop::window::{Window, property::SubWindowOption}, error::StudioError};

impl<'window> Window<'window> {



    /// Add a sub window.
    /// 
    /// Returns Ok(true) on success or Err(StudioError) if already added or owned.
    pub(crate) fn add_sub(&'window self, sub: &'window Window<'window>){

        // usize::MAX means subwindow was not found.
        if self.get_sub_index(sub.clone()) == usize::MAX {
            self.subs.borrow_mut().push(sub)
        }

    }

    /// Remove a subwindow from the subwindow array.
    /// 
    /// Returns index as usize of deleted.
    pub(crate) fn remove_sub(&'window mut self, sub: &'window Window<'window>)-> usize {

        let index = self.get_sub_index(sub);

        // Try to remove only if found.
        if index != usize::MAX {
            self.subs.borrow_mut().remove(index);
        }

        index
    }

    /// Get index of subwindow in array.
    /// 
    /// Returns Ok(index) on success, usize::MAX on failure.
    fn get_sub_index(&'window self, sub: &Window) -> usize {
        for i in 0..self.subs.borrow().len() {
            match self.subs.borrow().get(i){
                Some(sw) => if *sw as * const _ == sub as *const _ {
                    return i
                },
                None => {},
            }
        }

        usize::MAX
    }

}