#![deny(missing_docs)]
#![unstable]

//! A library for setting current values for stack scope,
//! such as application structure.

pub use get::Get;
pub use set::Set;
pub use action::Action;

mod refcell;
mod get;
mod set;
mod action;

/// Something that can be set at an object.
///
/// Must be implemented on a `(Property, Object)`
#[unstable]
pub trait SetAt: Pair {
    /// Modify `F` with self.
    fn set_at(
        val: <Self as Pair>::Data,
        obj: &mut <Self as Pair>::Object
    );
}

/// Something that can be retrieved from another object.
#[unstable]
pub trait GetFrom: Pair {
    /// Gets value from object.
    fn get_from(
        obj: &<Self as Pair>::Object
    ) -> <Self as Pair>::Data;
}

/// Does something to an object.
#[unstable]
pub trait ActOn<T>: Pair {
    /// Does something to an object.
    fn act_on(
        action: <Self as Pair>::Data,
        obj: &mut <Self as Pair>::Object
    ) -> T;
}

/// Used to reduce the need for associated types.
pub trait Pair {
    type Data;
    type Object;
}

impl<T, U> Pair for (T, U) {
    type Data = T;
    type Object = U;
}
