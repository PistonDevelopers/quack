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

#[macro_export]
macro_rules! quack {
    (
        $this:ident : $this_type:ty ,
        get:
        $(fn () -> $get_prop_type:ty { $e:expr })*
        set:
        $(fn ($val:ident : $set_prop_type:ty) { $f:expr })*
        action:
        $(fn ($action:ident : $action_type:ty) -> $ret_action_type:ty { $g:expr })*
    ) => {
        $(impl $crate::GetFrom for ($get_prop_type, $this_type) {
            #[inline(always)]
            fn get_from($this: &$this_type) -> $get_prop_type {
                $e
            }
        })*
        $(impl $crate::SetAt for ($set_prop_type, $this_type) {
            #[inline(always)]
            fn set_at($val : $set_prop_type, $this : &mut $this_type) {
                $f
            }
        })*
        $(impl $crate::ActOn<$ret_action_type> for ($action_type, $this_type) {
            #[inline(always)]
            fn act_on(
                $action : $action_type,
                $this: &mut $this_type
            ) -> $ret_action_type {
                $g
            }
        })*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Foo {
        x: i32,
        y: i32,
    }

    impl Foo {
        pub fn new() -> Foo {
            Foo { x: 0, y: 0 }
        }
    }

    pub struct X(pub i32);
    pub struct Y(pub i32);
    pub struct IncX;

    quack! {
        this: Foo,
        get:
            fn () -> X { X(this.x) }
            fn () -> Y { Y(this.y) }
        set:
            fn (x: X) { this.x = x.0 }
            fn (y: Y) { this.y = y.0 }
        action:
            fn (__: IncX) -> () { this.x += 1 }
    }

    #[test]
    fn test_foo() {
        let mut foo = Foo::new().set(X(1));
        let X(x) = foo.get();
        assert_eq!(x, 1);
        foo.action(IncX);
        let X(x) = foo.get();
        assert_eq!(x, 2);
    }
}
