use GetFrom;
use SetAt;
use ActOn;
use Pair;
use Associative;

impl<'a, T, U> GetFrom for (U, &'a mut T)
    where
        (U, T): Pair<Data = U, Object = T> + GetFrom
{
    #[inline(always)]
    fn get_from(obj: &&'a mut T) -> U {
        <(U, T) as GetFrom>::get_from(obj)
    }
}

impl<'a, F, T> SetAt for (T, &'a mut F)
    where
        (T, F): Pair<Data = T, Object = F> + SetAt
{
    #[inline(always)]
    fn set_at(val: T, obj: &mut &'a mut F) {
        <(T, F) as SetAt>::set_at(val, obj)
    }
}

impl<'a, F, A> ActOn for (A, &'a mut F)
    where
        (A, F): Pair<Data = A, Object = F> + ActOn
{
    type Result = <(A, F) as ActOn>::Result;

    #[inline(always)]
    fn act_on(
        action: A,
        obj: &mut &'a mut F
    ) -> <(A, F) as ActOn>::Result {
        <(A, F) as ActOn>::act_on(action, obj)
    }
}

impl<'a, F, A> Associative for (A, &'a mut F)
    where
        (A, F): Pair<Data = A, Object = F> + Associative
{
    type Type = <(A, F) as Associative>::Type;
}
