
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{ Deref, DerefMut };

use GetFrom;
use SetAt;
use ActOn;
use Pair;

impl<'a, T, U> GetFrom for (U, &'a RefCell<T>)
    where
        (U, T): Pair<Data = U, Object = T> + GetFrom
{
    #[inline(always)]
    fn get_from(obj: &&'a RefCell<T>) -> U {
        <(U, T) as GetFrom>::get_from(obj.borrow().deref())
    }
}

impl<T, U> GetFrom for (U, Rc<RefCell<T>>)
    where
        (U, T): Pair<Data = U, Object = T> + GetFrom
{
    #[inline(always)]
    fn get_from(obj: &Rc<RefCell<T>>) -> U {
        <(U, T) as GetFrom>::get_from(obj.borrow().deref())
    }
}

impl<'a, F, T> SetAt for (T, &'a RefCell<F>)
    where
        (T, F): Pair<Data = T, Object = F> + SetAt
{
    #[inline(always)]
    fn set_at(val: T, obj: &mut &'a RefCell<F>) {
        <(T, F) as SetAt>::set_at(val, obj.borrow_mut().deref_mut())
    }
}

impl<F, T> SetAt for (T, Rc<RefCell<F>>)
    where
        (T, F): Pair<Data = T, Object = F> + SetAt
{
    #[inline(always)]
    fn set_at(val: T, obj: &mut Rc<RefCell<F>>) {
        <(T, F) as SetAt>::set_at(val, obj.borrow_mut().deref_mut())
    }
}


impl<'a, F, A, V> ActOn<V> for (A, &'a RefCell<F>)
    where
        (A, F): Pair<Data = A, Object = F> + ActOn<V>
{
    #[inline(always)]
    fn act_on(action: A, obj: &mut &'a RefCell<F>) -> V {
        <(A, F) as ActOn<V>>::act_on(action, obj.borrow_mut().deref_mut())
    }
}

impl<F, A, V> ActOn<V> for (A, Rc<RefCell<F>>)
    where
        (A, F): Pair<Data = A, Object = F> + ActOn<V>
{
    #[inline(always)]
    fn act_on(action: A, obj: &mut Rc<RefCell<F>>) -> V {
        <(A, F) as ActOn<V>>::act_on(action, obj.borrow_mut().deref_mut())
    }
}
