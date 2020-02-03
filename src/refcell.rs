use super::*;

use std::cell::RefCell;
use std::rc::Rc;

impl<'a, T, U> Get<T> for &'a RefCell<U> where U: Get<T> {
    #[inline(always)]
    fn get(&self) -> T {self.borrow().get()}
}

impl<'a, T, U> Set<T> for &'a RefCell<U> where U: Set<T> {
    #[inline(always)]
    fn set(&mut self, val: T) {self.borrow_mut().set(val)}
}

impl<'a, T, U> Action<T> for &'a RefCell<U> where U: Action<T> {
    type Result = U::Result;
    #[inline(always)]
    fn action(&mut self, val: T) -> Self::Result {self.borrow_mut().action(val)}
}

impl<T, U> Get<T> for Rc<RefCell<U>> where U: Get<T> {
    #[inline(always)]
    fn get(&self) -> T {self.borrow().get()}
}

impl<T, U> Set<T> for Rc<RefCell<U>> where U: Set<T> {
    #[inline(always)]
    fn set(&mut self, val: T) {self.borrow_mut().set(val)}
}

impl<T, U> Action<T> for Rc<RefCell<U>> where U: Action<T> {
    type Result = U::Result;
    #[inline(always)]
    fn action(&mut self, val: T) -> Self::Result {self.borrow_mut().action(val)}
}
