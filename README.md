# Quack - Duck typing traits

[Duck typing](https://en.wikipedia.org/wiki/Duck_typing) is a method
where code works when certain methods and properties are present,
instead of requiring a certain type.

### Design

An duck typing abstraction is composed from the following building blocks:

- Get properties (implements the `Get` trait)
- Set properties (implements the `Set` trait)
- Actions (implements the `Action` trait)

Is is common to declare a newtype for each property and action.

For example, declare `pub struct X(pub f64);` for `Get<X>` and `Set<X>`.

The `quack!` macro can be used to implement simple get/set properties.

`Get`, `Set` and `Action` traits are auto implemented for `&RefCell<T>` and `Rc<RefCell<T>>`.
This simplifies working with dynamic borrowing in a single thread.

### Example

```rust
extern crate quack;

use quack::*;

#[derive(Copy, Clone, Debug)]
pub struct Tire {pub winter: bool}

#[derive(Debug, Clone)]
pub struct Car {pub tires: [Tire; 4]}

pub struct LeftFrontTire(pub Tire);
pub struct RightFrontTire(pub Tire);
pub struct LeftBackTire(pub Tire);
pub struct RightBackTire(pub Tire);

quack!{
    for Car {
        get_set LeftFrontTire(self.tires[0]),
        get_set RightFrontTire(self.tires[1]),
        get_set LeftBackTire(self.tires[2]),
        get_set RightBackTire(self.tires[3]),
    }
}

pub struct ShiftToWinterTires;
impl Action<ShiftToWinterTires> for Car {
    type Result = ();
    fn action(&mut self, _: ShiftToWinterTires) -> () {
        for i in 0..4 {self.tires[i].winter = true}
    }
}

pub struct ShiftToSummerTires;
impl Action<ShiftToSummerTires> for Car {
    type Result = ();
    fn action(&mut self, _: ShiftToSummerTires) -> () {
        for i in 0..4 {self.tires[i].winter = false}
    }
}

// Implement trait on top of duck type object.
pub trait GenericCar:
    GetSet<LeftFrontTire> +
    GetSet<RightFrontTire> +
    GetSet<LeftBackTire> +
    GetSet<RightBackTire> +
    Action<ShiftToSummerTires> +
    Action<ShiftToWinterTires>
{
    fn left_front_tire(&self) -> Tire {Get::<LeftFrontTire>::get(self).0}
    fn right_front_tire(&self) -> Tire {Get::<RightFrontTire>::get(self).0}
    fn left_back_tire(&self) -> Tire {Get::<LeftBackTire>::get(self).0}
    fn right_back_tire(&self) -> Tire {Get::<RightBackTire>::get(self).0}

    fn set_left_front_tire(&mut self, val: Tire) {self.set(LeftFrontTire(val))}
    fn set_right_front_tire(&mut self, val: Tire) {self.set(RightFrontTire(val))}
    fn set_left_back_tire(&mut self, val: Tire) {self.set(LeftBackTire(val))}
    fn set_right_back_tire(&mut self, val: Tire) {self.set(RightBackTire(val))}

    fn shift_to_winter_tires(&mut self) {self.action(ShiftToWinterTires);}
    fn shift_to_summer_tires(&mut self) {self.action(ShiftToSummerTires);}
}

// Auto implement `GenericCar`.
impl<T> GenericCar for T where T:
    GetSet<LeftFrontTire> +
    GetSet<RightFrontTire> +
    GetSet<LeftBackTire> +
    GetSet<RightBackTire> +
    Action<ShiftToSummerTires> +
    Action<ShiftToWinterTires>
{}

fn main() {
    let mut car = Car {tires: [Tire {winter: false}; 4]};

    car.shift_to_winter_tires();
    println!("{:?}", car);

    car.set_left_front_tire(Tire {winter: false});
    println!("Left front tire: {:?}", car.left_front_tire());
}
```
