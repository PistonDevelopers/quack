use GetFrom;
use Pair;

/// Automatically implemented through the `GetFrom` trait.
#[unstable]
pub trait Get<T> {
    /// Returns new value.
    fn get(&self) -> T;
}

impl<T, U> Get<U> for T
    where
        (U, T): Pair<Data = U, Object = T> + GetFrom
{
    #[inline(always)]
    fn get(&self) -> U {
        <(U, T) as GetFrom>::get_from(self)
    }
}
