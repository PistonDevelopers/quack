use ActOn;
use Pair;

/// Automatically implemented through the `ActOn` trait.
#[unstable]
pub trait Action<A, V> {
    /// Does something.
    fn action(&mut self, val: A) -> V;
}

impl<T, A, V> Action<A, V> for T
    where
        (A, T): Pair<Data = A, Object = T> + ActOn<V>
{
    #[inline(always)]
    fn action(&mut self, action: A) -> V {
        <(A, T) as ActOn<V>>::act_on(action, self)
    }
}
