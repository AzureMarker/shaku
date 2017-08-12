# SHAKU don't support structs with references
- Can't store references into an AnyMap unless lifetime is static
- I might be able to transmute it into a static value but would it make sense?

# SHAKU removes all registration data once a component is resolved
- Because Clone is not enforced on Parameters / dependencies