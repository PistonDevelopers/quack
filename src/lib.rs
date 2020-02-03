
pub trait Get<T> {
    fn get(&self) -> T;
}

pub trait Set<T> {
    fn set(&mut self, val: T);
}

pub trait GetSet<T>: Get<T> + Set<T> {}

impl<T, U> GetSet<T> for U where U: Get<T> + Set<T> {}

pub trait Action<T> {
    type Result;
    fn action(&mut self, val: T) -> Self::Result;
}

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
