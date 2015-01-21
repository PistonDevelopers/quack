#![deny(missing_docs)]
#![unstable]

//! A library for setting current values for stack scope,
//! such as application structure.

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{ Deref, DerefMut };

impl<'a, T, U> GetFrom for (U, &'a RefCell<T>)
    where
        (U, T): GetFrom<Property = U, Object = T>
{
    type Property = U;
    type Object = &'a RefCell<T>;

    #[inline(always)]
    fn get_from(obj: &&'a RefCell<T>) -> U {
        <(U, T) as GetFrom>::get_from(obj.borrow().deref())
    }
}

impl<T, U> GetFrom for (U, Rc<RefCell<T>>)
    where
        (U, T): GetFrom<Property = U, Object = T>
{
    type Property = U;
    type Object = Rc<RefCell<T>>;

    #[inline(always)]
    fn get_from(obj: &Rc<RefCell<T>>) -> U {
        <(U, T) as GetFrom>::get_from(obj.borrow().deref())
    }
}

impl<'a, F, T> SetAt for (T, &'a RefCell<F>)
    where
        (T, F): SetAt<Property = T, Object = F>
{
    type Property = T;
    type Object = &'a RefCell<F>;

    #[inline(always)]
    fn set_at(val: T, obj: &mut &'a RefCell<F>) {
        <(T, F) as SetAt>::set_at(val, obj.borrow_mut().deref_mut())
    }
}

impl<F, T> SetAt for (T, Rc<RefCell<F>>)
    where
        (T, F): SetAt<Property = T, Object = F>
{
    type Property = T;
    type Object = Rc<RefCell<F>>;

    #[inline(always)]
    fn set_at(val: T, obj: &mut Rc<RefCell<F>>) {
        <(T, F) as SetAt>::set_at(val, obj.borrow_mut().deref_mut())
    }
}


impl<'a, F, A, V> ActOn<V> for (A, &'a RefCell<F>)
    where
        (A, F): ActOn<V, Action = A, Object = F>
{
    type Action = A;
    type Object = &'a RefCell<F>;

    #[inline(always)]
    fn act_on(action: A, obj: &mut &'a RefCell<F>) -> V {
        <(A, F) as ActOn<V>>::act_on(action, obj.borrow_mut().deref_mut())
    }
}

impl<F, A, V> ActOn<V> for (A, Rc<RefCell<F>>)
    where
        (A, F): ActOn<V, Action = A, Object = F>
{
    type Action = A;
    type Object = Rc<RefCell<F>>;

    #[inline(always)]
    fn act_on(action: A, obj: &mut Rc<RefCell<F>>) -> V {
        <(A, F) as ActOn<V>>::act_on(action, obj.borrow_mut().deref_mut())
    }
}

/// Something that can be set at an object.
///
/// Must be implemented on a `(Property, Object)`
#[unstable]
pub trait SetAt {
    type Property;
    type Object;

    /// Modify `F` with self.
    fn set_at(val: Self::Property, obj: &mut Self::Object);
}

/// Automatically implemented through the `SetAt` trait.
#[unstable]
pub trait Set<T> {
    /// Set value.
    fn set(mut self, val: T) -> Self;

    /// Set value through mutable reference.
    fn set_mut(&mut self, val: T) -> &mut Self;
}

impl<T, U> Set<U> for T
    where
        (U, T): SetAt<Property = U, Object = T>,
{
    #[inline(always)]
    fn set(mut self, val: U) -> T {
        <(U, T) as SetAt>::set_at(val, &mut self);
        self
    }

    #[inline(always)]
    fn set_mut(&mut self, val: U) -> &mut T {
        <(U, T) as SetAt>::set_at(val, self);
        self
    }
}

/// Something that can be retrieved from another object.
#[unstable]
pub trait GetFrom {
    type Property;
    type Object;

    /// Gets value from object.
    fn get_from(obj: &Self::Object) -> Self::Property;
}

/// Automatically implemented through the `GetFrom` trait.
#[unstable]
pub trait Get<T> {
    /// Returns new value.
    fn get(&self) -> T;
}

impl<T, U> Get<U> for T
    where
        (U, T): GetFrom<Property = U, Object = T>
{
    #[inline(always)]
    fn get(&self) -> U {
        <(U, T) as GetFrom>::get_from(self)
    }
}

/// Does something to an object.
#[unstable]
pub trait ActOn<T> {
    type Action;
    type Object;

    /// Does something to an object.
    fn act_on(action: Self::Action, obj: &mut Self::Object) -> T;
}

/// Automatically implemented through the `ActOn` trait.
#[unstable]
pub trait Action<A, V> {
    /// Does something.
    fn action(&mut self, val: A) -> V;
}

impl<T, A, V> Action<A, V> for T
    where
        (A, T): ActOn<V, Action = A, Object = T>
{
    #[inline(always)]
    fn action(&mut self, action: A) -> V {
        <(A, T) as ActOn<V>>::act_on(action, self)
    }
}

