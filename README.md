# box_shorthand

I like to write lots of tree types based on enums and test deeply nested hierarchies of them. Writing these tests is tedious because one must explicitly box nested values. This crate adds a derive macro called `box_shorthand`. It generates a module containing functions for creating boxed values of the enum type.

Given the following type declaration:

```rust
#[derive(box_shorthand)]
enum Enum {
    Unit,
    Tuple(i64, i64)
    Struct {
        x: i64,
        y: i64,
    }
}
```

The following code will be generated:

```rust
mod EnumBox {
    use super::*;

    fn Unit() -> Box<Enum> {
        Box::new(Enum::Unit)
    }

    fn Tuple(field0: i64, field1: i64) -> Box<Enum> {
        Box::new(Enum::Tuple(field0, field1))
    }

    fn Struct(x: i64, y: i64) -> Box<Enum> {
        Box::new(Enum::Struct { x, y })
    }
}
```

See integration test in `/tests`.

# Usage

Simply call the generated functions to construct boxed variant values directly. If you want even shorter shorthand, just `use EnumBox::*` to get direct access to the functions!

Please note that unit variant shorthands are called with parens because they are simply functions. The same goes for struct variant shorthands.

The macro should work properly with generic arguments and where clauses.

Please note that this macro will fail on types other than enums.

# Note on issue #50504

The compiler complains that the types imported into the module should not be visible and it will break in the future. But this doesn't appear to be correct since we have `use super::*;` at the top of every generated module. So we suppress this warning.

# Future

I may extend this to generate `new_boxed` functions for structs. It may also be a good idea to generalize this to unions. Feel free to open an issue for suggestions. I don't promise that I will look at them or respond.
