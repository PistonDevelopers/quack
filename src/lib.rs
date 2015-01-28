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

macro_rules! items { ($($x:item)+) => ($($x)+) }

#[macro_export]
macro_rules! quack_get {
    (
        $this:ident : $this_type:ident [$($t:tt),*]
        fn () -> $get_prop_type:path { $e:expr }
    ) => {items!{
        impl<$($t),*> $crate::GetFrom for ($get_prop_type, $this_type<$($t),*>) {
            #[inline(always)]
            fn get_from($this: &$this_type<$($t),*>) -> $get_prop_type {
                $e
            }
        }
    }}
}

#[macro_export]
macro_rules! quack_set {
    (
        $this:ident : $this_type:ident [$($t:tt),*]
        fn ($val:ident : $set_prop_type:path) { $f:expr }
    ) => {items!{
        impl<$($t),*> $crate::SetAt for ($set_prop_type, $this_type<$($t),*>) {
            #[inline(always)]
            fn set_at($val : $set_prop_type, $this : &mut $this_type<$($t),*>) {
                $f
            }
        }
    }}
}

#[macro_export]
macro_rules! quack_action {
    (
        $this: ident : $this_type:ident [$($t:tt),*]
        fn ($action:ident : $action_type:path) -> $ret_action_type:ty { $g:expr }
    ) => {items!{
        impl<$($t),*> $crate::ActOn<$ret_action_type>
        for ($action_type, $this_type<$($t),*>) {
            #[inline(always)]
            fn act_on(
                $action : $action_type,
                $this: &mut $this_type<$($t),*>
            ) -> $ret_action_type {
                $g
            }
        }
    }}
}

#[macro_export]
macro_rules! quack {
    (
        $this:ident : $this_type:ident $t:tt
        get:
        $(fn () -> $get_prop_type:path { $e:expr })*
        set:
        $(fn ($val:ident : $set_prop_type:path) { $f:expr })*
        action:
        $(fn ($action:ident : $action_type:path) -> $ret_action_type:ty { $g:expr })*
    ) => {items!{
        $(quack_get!{
            $this : $this_type $t
            fn () -> $get_prop_type { $e }
        })*
        $(quack_set!{
            $this: $this_type $t
            fn ($val : $set_prop_type) { $f }
        })*
        $(quack_action!{
            $this: $this_type $t
            fn ($action : $action_type) -> $ret_action_type { $g }
        })*
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Foo<'a, 'b, A, B> {
        x: i32,
        y: i32,
    }

    impl<'a, 'b, A, B> Foo<'a, 'b, A, B> {
        pub fn new() -> Foo<'a, 'b, A, B> {
            Foo { x: 0, y: 0 }
        }
    }

    pub struct X<'a>(pub i32);
    pub struct Y<A>(pub i32);
    pub struct IncX;

    quack! {
        this: Foo['a, 'b, A, B]
        get:
            fn () -> X<'a> { X(this.x) }
            fn () -> Y<A> { Y(this.y) }
        set:
            fn (x: X<'a>) { this.x = x.0 }
            fn (y: Y<A>) { this.y = y.0 }
        action:
            fn (__: IncX) -> () { this.x += 1 }
    }

    pub struct Bar;

    #[test]
    fn test_foo() {
        let mut foo: Foo<Bar, Bar> = Foo::new().set(X(1));
        let X(x) = foo.get();
        assert_eq!(x, 1);
        foo.action(IncX);
        let X(x) = foo.get();
        assert_eq!(x, 2);
    }
}
