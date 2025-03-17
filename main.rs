//! A more nuanced implementation of 2e71828's (Eric Sumner) type-state toy
//!
//! https://users.rust-lang.org/t/conflicting-implementations-of-trait/53055/5
//! rustc 1.84.0-nightly (f7273e004 2024-11-12)
//!
//! The main goal here is a unified user-facing 'interface' for handling different types. Our
//! internal entrypoint is __Add, while the Schema structure's `add` method is the external
//! entrypoint. We want the users to be able to call `add` regardless of type as long as they
//! implement either Edge or Vertex. Thankfully, the compiler handles this without much complaint.
//!
//! The __Add implementations are absolute boilerplate, the idea is to use a macro (proc
//! or not, your choice) to generate these impls.


pub trait Entity {
    type Handler;
}

pub trait Edge: Entity {
    fn inbound() -> &'static str;
    fn outbound() -> &'static str;
}

pub trait Vertex: Entity {
    fn label() -> &'static str;
}

pub struct VertexHandler;
pub struct EdgeHandler;

pub struct Schema {
    pub vertices: Vec<&'static str>,
    pub edges: Vec<(&'static str, &'static str)>,
}

impl Schema {
    pub fn new() -> Self { Self { vertices: vec![], edges: vec![] } }

    #[allow(private_bounds)]
    pub fn add<E: Entity>(&mut self) where E::Handler: AddImpl<Self, E>  {
        E::Handler::add(self);
    }

    pub fn report(&self) {
        println!("Vertices: {:?}", self.vertices);
        println!("Edges: {:?}", self.edges);
    }
}

trait AddImpl<T, E> {
    fn add(schema: &mut T);
}

impl<E: Vertex> AddImpl<Schema, E> for VertexHandler {
    fn add(schema: &mut Schema) {
        schema.vertices.push(E::label());
    }
}

impl<E: Edge> AddImpl<Schema, E> for EdgeHandler {
    fn add(schema: &mut Schema) {
        schema.edges.push((
            E::inbound(),
            E::outbound()
        ));
    }
}

struct EdgeA;
impl Edge for EdgeA {
    fn inbound() -> &'static str {
        VertexA::label()
    }

    fn outbound() -> &'static str {
        VertexB::label()
    }
}
impl Entity for EdgeA {
    type Handler = EdgeHandler;
}
struct VertexA;
impl Vertex for VertexA {
    fn label() -> &'static str { "vertex_a" }
}
impl Entity for VertexA {
    type Handler = VertexHandler;
}
struct VertexB;
impl Vertex for VertexB {
    fn label() -> &'static str { "vertex_b" }
}
impl Entity for VertexB {
    type Handler = VertexHandler;
}
fn main() {
    let mut schema = Schema::new();

    schema.add::<VertexA>();
    schema.add::<VertexB>();
    schema.add::<EdgeA>();

    schema.report();
}
