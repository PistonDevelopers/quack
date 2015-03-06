
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{ Deref, DerefMut };

use GetFrom;
use SetAt;
use ActOn;
use Pair;
use Associative;

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

impl<'a, F, A> ActOn for (A, &'a RefCell<F>)
    where
        (A, F): Pair<Data = A, Object = F> + ActOn
{
    type Result = <(A, F) as ActOn>::Result;

    #[inline(always)]
    fn act_on(
        action: A,
        obj: &mut &'a RefCell<F>
    ) -> <(A, F) as ActOn>::Result {
        <(A, F) as ActOn>::act_on(action, obj.borrow_mut().deref_mut())
    }
}

impl<F, A> ActOn for (A, Rc<RefCell<F>>)
    where
        (A, F): Pair<Data = A, Object = F> + ActOn
{
    type Result = <(A, F) as ActOn>::Result;

    #[inline(always)]
    fn act_on(
        action: A,
        obj: &mut Rc<RefCell<F>>
    ) -> <(A, F) as ActOn>::Result {
        <(A, F) as ActOn>::act_on(action, obj.borrow_mut().deref_mut())
    }
}

impl<'a, F, A> Associative for (A, &'a RefCell<F>)
    where
        (A, F): Pair<Data = A, Object = F> + Associative
{
    type Type = <(A, F) as Associative>::Type;
}

impl<F, A> Associative for (A, Rc<RefCell<F>>)
    where
        (A, F): Pair<Data = A, Object = F> + Associative
{
    type Type = <(A, F) as Associative>::Type;
}
