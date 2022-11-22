/// Declare a new const array without specify length.
/// It helps when apply conditional compilation to part of a const array.
///
/// # Syntax
/// The macro wraps any number of const array declarations(with length `_`).
///
/// ```
/// use auto_const_array::auto_const_array;
/// auto_const_array! {
///     pub const FOO: [u8; _] = [1, 2, 3];
///
///     #[allow(unused)]
///     const BAR: [u32; _] = [1];
/// }
/// ```
#[macro_export]
macro_rules! auto_const_array {
    () => {};
    ($(#[$attr:meta])* $vis:vis const $name:ident: [$t:ty; _] = [$($inner:expr,)*]; $($rest:tt)*) => {
        $(#[$attr])* $vis const $name: [$t; $crate::count!($($inner)*)] = [$($inner,)*];
        $crate::auto_const_array!($($rest)*);
    };
    ($(#[$attr:meta])* $vis:vis const $name:ident: [$t:ty; _] = [$($inner:expr),*]; $($rest:tt)*) => {
        $(#[$attr])* $vis const $name: [$t; $crate::count!($($inner)*)] = [$($inner,)*];
        $crate::auto_const_array!($($rest)*);
    };
    ($(#[$attr:meta])* $vis:vis const $name:ident: [$t:ty; _] = [$($inner:expr,)*]) => {
        $(#[$attr])* $vis const $name: [$t; $crate::count!($($inner)*)] = [$($inner,)*];
    };
    ($(#[$attr:meta])* $vis:vis const $name:ident: [$t:ty; _] = [$($inner:expr),*]) => {
        $(#[$attr])* $vis const $name: [$t; $crate::count!($($inner)*)] = [$($inner,)*];
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    () => (0usize);
    ($pre:tt $($left:tt)*) => (1usize + $crate::count!($($left)*));
}

#[cfg(test)]
mod tests {
    crate::auto_const_array! {
        #[allow(unused)]
        const ARRAY: [u8; _] = [1, 2, 3];
        #[allow(unused)]
        const ARRAY_EMPTY: [u8; _] = [];
        #[allow(unused)]
        const ARRAY_WITH_EXPR: [u8; _] = [1 + 1, 2 + 2,];
    }
}