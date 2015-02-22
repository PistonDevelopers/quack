use ActOn;
use Pair;

/// Automatically implemented through the `ActOn` trait.
#[unstable]
pub trait Action<A> {
    type Result;

    /// Does something.
    fn action(&mut self, val: A) -> Self::Result;
}

impl<T, A> Action<A> for T
    where
        (A, T): Pair<Data = A, Object = T> + ActOn
{
    type Result = <(A, T) as ActOn>::Result;

    #[inline(always)]
    fn action(&mut self, action: A) -> <Self as Action<A>>::Result {
        <(A, T) as ActOn>::act_on(action, self)
    }
}
