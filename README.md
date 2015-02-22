quack
=====

Duck typing traits

[Why Quack?](https://github.com/PistonDevelopers/quack/issues/10)

```Rust
quack! {
    obj: Button[],
    get:
        fn () -> Position [] { Position(obj.pos) }
    set:
        fn (val: Position) [] { obj.pos = val.0 }
    action:
        fn (__: Enable) -> () [] { obj.enabled = true; }
        fn (__: Disable) -> () [] { obj.enabled = false; }
}

// Implement trait for all types that can get/set position and with enable action
impl<T> Foo for T
    where
        (Position, T): Pair<Data = Position, Object = T> + GetFrom + SetAt,
        (Enable, T): Pair<Data = Enable, Object = T> + ActOn<()>
{
    ...
}

use quack::{ Get, Set, Action };

// Build button
let button = Button::new().set(Position([0, 0]));

// Set position
button.set_mut(Position([0, 0]));

// Get position
let Position([x, y]) = button.get();

// Enable
button.action(Enable);
```

