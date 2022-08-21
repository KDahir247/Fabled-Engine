use fabled_render::mesh::{Cone, Cube, Mesh};

mod core;
mod test;

fn main() {
    superluminal_perf::begin_event("start primitive");

    let a = Cone::default();

    for c in 0..1000 {
        superluminal_perf::begin_event("Primitive_call");

        let b: Mesh = Mesh::from(a);
        superluminal_perf::end_event();

        println!("{:?}", b.vertices.len());
    }

    superluminal_perf::end_event();

    // core::State::run();
}
