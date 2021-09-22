// pub struct GltfLoader {
// width: u32,
// }

pub fn run() {
    // let target = std::path::Path::new("example/anime/scene.gltf");
    // let (document, buffers, images) = gltf::import(target).unwrap();
    //
    // for mesh in document.meshes() {
    // for primitive in mesh.primitives() {
    // let reader =
    // primitive.reader(|buffer| buffers.get(buffer.index()).map(|b|
    // b.0.as_slice()));
    //
    // if let Some(valid_position) = reader.read_positions() {
    // valid_position.into_iter().par_bridge().for_each(|vertex| {
    // pass the positions in a Vec
    // });
    // }
    //
    // if let Some(valid_indices) = reader.read_indices() {
    // valid_indices
    // .into_u32()
    // .into_iter()
    // .par_bridge()
    // .for_each(|indices| {
    // pass the indices in a Vec
    // });
    // }
    //
    // if let Some(valid_normal) = reader.read_normals() {
    // valid_normal.into_iter().par_bridge().for_each(|normal| {
    // pass the normal in a Vec
    // })
    // }
    //
    // match reader.read_tangents() {
    // None => {
    // Manual create tangents and add the tangent to a Vec
    // }
    // Some(tangents) => {
    // tangents.into_iter().par_bridge().for_each(|tangent| {
    // add the tangent in a Vec
    // });
    // }
    // }
    //
    // if let Some(tex_coord) = reader.read_tex_coords(0) {
    // tex_coord
    // .into_f32()
    // .into_iter()
    // .par_bridge()
    // .for_each(|coord| {});
    // }
    //
    // Calculate Bi-Tangent
    // println!("{}", primitive.indices().unwrap().index());
    // println!("{:?}", primitive.material().index());
    // println!("{:?}", primitive.mode());*/
    // }
    // println!("\n");*/
    // }
    // let load_texture = |texture: &gltf::Texture| {
    // let img = images.get(texture.index()).unwrap();
    // println!("{:?} {:?}", img.height, img.width);
    //
    // match img.format {
    // Format::R8 => {}
    // Format::R8G8 => {}
    // Format::R8G8B8 => {}
    // Format::R8G8B8A8 => {}
    // Format::B8G8R8 => {}
    // Format::B8G8R8A8 => {}
    // Format::R16 => {}
    // Format::R16G16 => {}
    // Format::R16G16B16 => {}
    // Format::R16G16B16A16 => {}
    // }
    // };
    //
    // for s in document.textures() {
    // load_texture(&s);
    // }
    //
    // for b in document.materials() {}
    // for a in document.images() {
    // match a.source() {
    // Source::View { .. } => {}
    // Source::Uri { uri, .. } => {
    // println!("{}", uri);
    // }
    // }
    // }
}
