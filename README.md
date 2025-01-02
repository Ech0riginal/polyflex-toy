![ferris](https://rustacean.net/assets/rustacean-flat-happy.svg)

A more nuanced implementation of 2e71828's (Eric Sumner) [type-state toy](https://users.rust-lang.org/t/conflicting-implementations-of-trait/53055/5)

* rustc 1.84.0-nightly (f7273e004 2024-11-12)

The main goal here is a unified user-facing 'interface' for handling different types. Our
internal entrypoint is `__Add`, while the Schema structure's `add` method is the external
entrypoint. We want the users to be able to call `add` regardless of type as long as they
implement either Edge or Vertex. Thankfully, the compiler handles this without much complaint.

The `__Add` implementations are absolute boilerplate, the idea is to use a macro (proc or not, your choice) to generate these impls.
