use GetFrom;
use SetAt;
use ActOn;
use Pair;
use Associative;

impl<T, U> GetFrom for (U, Box<T>)
    where
        (U, T): Pair<Data = U, Object = T> + GetFrom
{
    #[inline(always)]
    fn get_from(obj: &Box<T>) -> U {
        <(U, T) as GetFrom>::get_from(obj)
    }
}

impl<F, T> SetAt for (T, Box<F>)
    where
        (T, F): Pair<Data = T, Object = F> + SetAt
{
    #[inline(always)]
    fn set_at(val: T, obj: &mut Box<F>) {
        <(T, F) as SetAt>::set_at(val, obj)
    }
}

impl<F, A> ActOn for (A, Box<F>)
    where
        (A, F): Pair<Data = A, Object = F> + ActOn
{
    type Result = <(A, F) as ActOn>::Result;

    #[inline(always)]
    fn act_on(
        action: A,
        obj: &mut Box<F>
    ) -> <(A, F) as ActOn>::Result {
        <(A, F) as ActOn>::act_on(action, obj)
    }
}

impl<F, A> Associative for (A, Box<F>)
    where
        (A, F): Pair<Data = A, Object = F> + Associative
{
    type Type = <(A, F) as Associative>::Type;
}
