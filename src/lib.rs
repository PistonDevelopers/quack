#![deny(missing_docs)]

//! # Quack - Duck typing traits
//!
//! [Duck typing](https://en.wikipedia.org/wiki/Duck_typing) is a method
//! where code works when certain methods and properties are present,
//! instead of requiring a certain type.
//!
//! ### Design
//!
//! An duck typing abstraction is composed from the following building blocks:
//!
//! - Get properties (implements the `Get` trait)
//! - Set properties (implements the `Set` trait)
//! - Actions (implements the `Action` trait)
//!
//! Is is common to declare a newtype for each property and action.
//!
//! For example, declare `pub struct X(pub f64);` for `Get<X>` and `Set<X>`.
//!
//! The `quack!` macro can be used to implement simple get/set properties.
//!
//! ### Example
//!
//! ```rust
//! extern crate quack;
//!
//! use quack::*;
//!
//! #[derive(Copy, Clone, Debug)]
//! pub struct Tire {pub winter: bool}
//!
//! #[derive(Debug, Clone)]
//! pub struct Car {pub tires: [Tire; 4]}
//!
//! pub struct LeftFrontTire(pub Tire);
//! pub struct RightFrontTire(pub Tire);
//! pub struct LeftBackTire(pub Tire);
//! pub struct RightBackTire(pub Tire);
//!
//! quack!{
//!     for Car {
//!         get_set LeftFrontTire(self.tires[0]),
//!         get_set RightFrontTire(self.tires[1]),
//!         get_set LeftBackTire(self.tires[2]),
//!         get_set RightBackTire(self.tires[3]),
//!     }
//! }
//!
//! pub struct ShiftToWinterTires;
//! impl Action<ShiftToWinterTires> for Car {
//!     type Result = ();
//!     fn action(&mut self, _: ShiftToWinterTires) -> () {
//!         for i in 0..4 {self.tires[i].winter = true}
//!     }
//! }
//!
//! pub struct ShiftToSummerTires;
//! impl Action<ShiftToSummerTires> for Car {
//!     type Result = ();
//!     fn action(&mut self, _: ShiftToSummerTires) -> () {
//!         for i in 0..4 {self.tires[i].winter = false}
//!     }
//! }
//!
//! // Implement trait on top of duck type object.
//! pub trait GenericCar:
//!     GetSet<LeftFrontTire> +
//!     GetSet<RightFrontTire> +
//!     GetSet<LeftBackTire> +
//!     GetSet<RightBackTire> +
//!     Action<ShiftToSummerTires> +
//!     Action<ShiftToWinterTires>
//! {
//!     fn left_front_tire(&self) -> Tire {Get::<LeftFrontTire>::get(self).0}
//!     fn right_front_tire(&self) -> Tire {Get::<RightFrontTire>::get(self).0}
//!     fn left_back_tire(&self) -> Tire {Get::<LeftBackTire>::get(self).0}
//!     fn right_back_tire(&self) -> Tire {Get::<RightBackTire>::get(self).0}
//!
//!     fn set_left_front_tire(&mut self, val: Tire) {self.set(LeftFrontTire(val))}
//!     fn set_right_front_tire(&mut self, val: Tire) {self.set(RightFrontTire(val))}
//!     fn set_left_back_tire(&mut self, val: Tire) {self.set(LeftBackTire(val))}
//!     fn set_right_back_tire(&mut self, val: Tire) {self.set(RightBackTire(val))}
//!
//!     fn shift_to_winter_tires(&mut self) {self.action(ShiftToWinterTires);}
//!     fn shift_to_summer_tires(&mut self) {self.action(ShiftToSummerTires);}
//! }
//!
//! // Auto implement `GenericCar`.
//! impl<T> GenericCar for T where T:
//!     GetSet<LeftFrontTire> +
//!     GetSet<RightFrontTire> +
//!     GetSet<LeftBackTire> +
//!     GetSet<RightBackTire> +
//!     Action<ShiftToSummerTires> +
//!     Action<ShiftToWinterTires>
//! {}
//!
//! fn main() {
//!     let mut car = Car {tires: [Tire {winter: false}; 4]};
//!
//!     car.shift_to_winter_tires();
//!     println!("{:?}", car);
//!
//!     car.set_left_front_tire(Tire {winter: false});
//!     println!("Left front tire: {:?}", car.left_front_tire());
//! }
//! ```

/// Get property.
pub trait Get<T> {
    /// Gets property value.
    fn get(&self) -> T;
}

/// Set property.
pub trait Set<T> {
    /// Sets property value.
    fn set(&mut self, val: T);
}

/// An auto implemented trait for get/set properties.
pub trait GetSet<T>: Get<T> + Set<T> {}

impl<T, U> GetSet<T> for U where U: Get<T> + Set<T> {}

/// Represents an action on objects.
pub trait Action<T> {
    /// The action result type.
    type Result;
    /// Performs action on object.
    fn action(&mut self, val: T) -> Self::Result;
}

/// Helper macro for simple get/set properties.
///
/// This macro supports 4 different syntaxes:
///
/// 1. Get/set property e.g. `quack!{get_set X(self.x) for Foo}`
/// 2. Get property e.g. `quack!{get X(self.x) for Foo}`
/// 3. Set property e.g. `quack!{set X(self.x) for Foo}`
/// 4. Multiple properties e.g. `quack!{for Foo {get X(self.x), get Y(self.y)}}`
#[macro_export]
macro_rules! quack {
    (get_set $prop:ident ( $($code:tt)* ) for $Self:ty) => {
        quack!(get $prop($($code)*) for $Self);
        quack!(set $prop($($code)*) for $Self);
    };
    (set $prop:ident ( self . $($code:tt)* ) for $Self:ty) => {
        impl Set<$prop> for $Self {
            #[inline(always)]
            fn set(&mut self, val: $prop) {self.$($code)* = val.0}
        }
    };
    (set $prop:ident ( self [ $($code:tt)* ] ) for $Self:ty) => {
        impl Set<$prop> for $Self {
            #[inline(always)]
            fn set(&mut self, val: $prop) {self[$($code)*] = val.0}
        }
    };
    (get $prop:ident ( self . $($code:tt)* ) for $Self:ty) => {
        impl Get<$prop> for $Self {
            #[inline(always)]
            fn get(&self) -> $prop {$prop(self.$($code)*)}
        }
    };
    (get $prop:ident ( self [ $($code:tt)* ] ) for $Self:ty) => {
        impl Get<$prop> for $Self {
            #[inline(always)]
            fn get(&self) -> $prop {$prop(self[$($code)*])}
        }
    };
    (
        for $Self:ty {
            $($cmd:tt $get_set_prop:ident ( $($get_set:tt)* )),* $(,)?
        }
    ) => {
        $(quack!($cmd $get_set_prop($($get_set)*) for $Self);)*
    }
}
