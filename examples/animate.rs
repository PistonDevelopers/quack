extern crate quack;

use quack::*;

pub struct X(pub f64);
pub struct Y(pub f64);
pub struct NewPoint;
pub struct Move(pub [f64; 2]);

impl Get<X> for Vec<[f64; 2]> {
    fn get(&self) -> X {
        X(self.last().map(|n| n[0]).unwrap_or(0.0))
    }
}

impl Get<Y> for Vec<[f64; 2]> {
    fn get(&self) -> Y {
        Y(self.last().map(|n| n[1]).unwrap_or(0.0))
    }
}

impl Set<X> for Vec<[f64; 2]> {
    fn set(&mut self, val: X) {
        let n = self.len();
        if n == 0 {return} else {self[n-1][0] = val.0}
    }
}

impl Set<Y> for Vec<[f64; 2]> {
    fn set(&mut self, val: Y) {
        let n = self.len();
        if n == 0 {return} else {self[n-1][1] = val.0}
    }
}

impl Action<NewPoint> for Vec<[f64; 2]> {
    type Result = ();
    fn action(&mut self, _: NewPoint) {
        if let Some(v) = self.last() {
            let v = v.clone();
            self.push(v)
        } else {
            self.push([0.0; 2])
        }
    }
}

impl Action<Move> for Vec<[f64; 2]> {
    type Result = ();
    fn action(&mut self, Move(p): Move) {
        let X(x) = self.get();
        let Y(y) = self.get();
        self.set(X(x + p[0]));
        self.set(Y(y + p[1]));
    }
}

fn main() {
    let mut p: Vec<[f64; 2]> = vec![];
    let mut t: f64 = 0.0;
    loop {
        p.action(NewPoint);
        p.action(Move([t.cos(), t.sin()]));
        t += 1.0;
        if t >= 10.0 {break}
    }

    println!("{:?}", p);
}
