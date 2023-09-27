`rustc_parse/src/parser/ty.rs:1024`
```rust
/// Recover from `Fn`-family traits (Fn, FnMut, FnOnce) with lifetime arguments
/// (e.g. `FnOnce<'a>(&'a str) -> bool`). Up to generic arguments have already
/// been eaten.
fn recover_fn_trait_with_lifetime_params(
    &mut self,
    fn_path: &mut ast::Path,
    lifetime_defs: &mut Vec<GenericParam>,
) -> PResult<'a, ()>
```

`rustc_parse/src/parser/ty.rs:1010`
```rust
/// Optionally parses `for<$generic_params>`.
pub(super) fn parse_late_bound_lifetime_defs(&mut self) -> PResult<'a, Vec<GenericParam>>
```

`rystc_parse/src/parser/expr.rs:2058`
```rust
/// Parses a closure expression (e.g., `move |args| expr`).
fn parse_closure_expr(&mut self) -> PResult<'a, P<Expr>>
```