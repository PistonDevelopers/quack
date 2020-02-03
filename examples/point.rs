extern crate quack;

use quack::*;

pub struct X(pub f64);
pub struct Y(pub f64);
pub struct Z(pub f64);

pub struct Norm(pub f64);

quack!{
    for [f64; 3] {
        get_set X(self[0]),
        get_set Y(self[1]),
        get_set Z(self[2]),
    }
}

impl Get<Norm> for [f64; 3] {
    fn get(&self) -> Norm {
        Norm((self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt())
    }
}

fn add<T>(a: T, b: T) -> T
    where T: GetSet<X> + GetSet<Y> + GetSet<Z> + Default
{
    let X(ax) = a.get();
    let X(bx) = b.get();
    let Y(ay) = a.get();
    let Y(by) = b.get();
    let Z(az) = a.get();
    let Z(bz) = b.get();
    let mut c = T::default();
    c.set(X(ax + bx));
    c.set(Y(ay + by));
    c.set(Z(az + bz));
    c
}

fn main() {
    let X(x) = [0.5, 0.3, 0.2].get();
    println!("{}", x);

    println!("{:?}", add([0.1, 0.2, 0.3], [0.3, 0.2, 0.1]));

    let Norm(n) = [0.5, 0.3, 0.2].get();
    println!("Norm {}", n);
}
