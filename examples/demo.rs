use auto_const_array::auto_const_array;

auto_const_array! {
    // Additional attributes and docs are supported.
    /// Common array with public visibility.
    #[allow(unused)]
    pub const ARRAY_COMMON: [u8; _] = [1, 2, 4];
    /// Special array with cfg conditional compling.
    const ARRAY_WITH_ATTR: [u8; _] = [1, #[cfg(unix)] 2]
}

fn main() {
    assert_eq!(ARRAY_COMMON.len(), 3);
    #[cfg(not(unix))]
    assert_eq!(ARRAY_WITH_ATTR.len(), 1);
    #[cfg(unix)]
    assert_eq!(ARRAY_WITH_ATTR.len(), 2);
}
