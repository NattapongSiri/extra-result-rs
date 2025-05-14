# extra-result
Add extra methods to `Result` by use a trait `ExtraResult`.

Current methods added is
- `map_fut`
- `map_or_fut`
- `map_or_else_fut`
- `map_err_fut`
- `inspect_fut`
- `inspect_err_fut`
- `and_then_fut`
- `or_else_fut`
- `unwrap_or_else_fut`
- `is_ok_and_fut`
- `is_err_and_fut`

All these methods is a mirror of a method of regular `Result` but it accept async function instead.

## How to use.
Simply add `use extra_result::*;` to a source file that going to use above methods.

Note: The crate only provide single trait name `ExtraResult`. It is also possible to `use extra_result::ExtraResult;` instead of using wildcard but it won't make any different.
