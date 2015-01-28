use SetAt;
use Pair;

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
        (U, T): Pair<Data = U, Object = T> + SetAt,
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
