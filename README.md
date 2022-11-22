# Auto Const Array

Use this macro to declare a const array without specify its length.

```
use auto_const_array::auto_const_array;
auto_const_array! {
    const ARRAY: [u8; _] = [1, 2, 3];
}
```