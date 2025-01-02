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


pub struct EdgeHandler;
pub trait Edge {
    fn inbound() -> &'static str;
    fn outbound() -> &'static str;
}
pub trait Edges {
    fn edges(&mut self) -> &mut Vec<(&'static str, &'static str)>;
}

pub struct VertexHandler;
pub trait Vertex {
    fn label() -> &'static str;
}
pub trait Vertexes {
    fn vertexes(&mut self) -> &mut Vec<&'static str>;
}

pub trait Handle<Any, Entity: Sized> {
    fn handle(_impl: &mut Any);
}

impl<Any, Entity> Handle<Any, Entity> for EdgeHandler where Any: Edges, Entity: Edge
{
    fn handle(_impl: &mut Any) {
        _impl.edges().push((
            Entity::inbound(),
            Entity::outbound()
        ));
    }
}

impl<Any, Entity> Handle<Any, Entity> for VertexHandler where Any: Vertexes, Entity: Vertex
{
    fn handle(_impl: &mut Any) {
        _impl.vertexes().push(Entity::label());
    }
}

trait __Add<Any, E>: Sized {
    type Handler: Handle<Any, E>;

    fn add(_impl: &mut Any) {
        Self::Handler::handle(_impl);
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
impl __Add<Schema, Self> for EdgeA { type Handler = EdgeHandler; }

struct VertexA;
impl Vertex for VertexA {
    fn label() -> &'static str { "vertex_a" }
}
impl __Add<Schema, Self> for VertexA { type Handler = VertexHandler; }

struct VertexB;
impl Vertex for VertexB {
    fn label() -> &'static str { "vertex_b" }
}
impl __Add<Schema, Self> for VertexB { type Handler = VertexHandler; }

pub struct Schema {
    pub vertices: Vec<&'static str>,
    pub edges: Vec<(&'static str, &'static str)>,
}

impl Schema {
    pub fn new() -> Self { Self { vertices: vec![], edges: vec![] } }
}

impl Edges for Schema {
    fn edges(&mut self) -> &mut Vec<(&'static str, &'static str)> {
        &mut self.edges
    }
}

impl Vertexes for Schema {
    fn vertexes(&mut self) -> &mut Vec<&'static str> {
        self.vertices.as_mut()
    }
}

impl Schema {
    #[allow(private_bounds)]
    pub fn add<E>(&mut self) where E: __Add<Self, E> {
        E::add(self);
    }

    pub fn report(&self) {
        println!("Vertices: {:?}", self.vertices);
        println!("Edges: {:?}", self.edges);
    }
}

fn main() {
    let mut schema = Schema::new();

    schema.add::<VertexA>();
    schema.add::<VertexB>();
    schema.add::<EdgeA>();

    schema.report();
}