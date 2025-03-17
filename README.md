![ferris](https://rustacean.net/assets/rustacean-flat-happy.svg)

A more nuanced implementation of 2e71828's (Eric Sumner) [type-state toy](https://users.rust-lang.org/t/conflicting-implementations-of-trait/53055/5)

* rustc 1.84.0-nightly (f7273e004 2024-11-12)

The main goal here is a unified method entrypoint into multiple routes of behavior (i.e. polymorphism). In our toy, we borrow some ideas from graph theory re: edges, vertices, etc. We'll define behavior which appends either a vertex or an edge to a Schema structure separately, then pulling it together on the Schema's `add` method. While developing a library you'd want to limit the creation of these different handlers to your macros and such, as well as sealing any trait which defines a handler to avoid undefined behavior in the overall system.
