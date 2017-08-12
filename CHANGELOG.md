# v0.2.1
- Changed output of `ContainerBuilder::build()` to return a `Result<Container, shaku::Error>` to support various registration errors.
- Added [fluent validator framework](https://github.com/U-007D/fluent_validator) to `shaku_internals`
- Added comments & tests

# v0.2.0
- Implemented `Container::resolve_ref()` and `Container::resolve_mut()`
- Added tests
- Removed dependency on external crates `anymap` and `shaku_internals` for applications using `shaku`
- Improved error management of generated code
- Moved former `shaku/examples` to a specific crate to avoid hidden external crate dependencies

# v0.1.1
# v0.1.0
- Creation