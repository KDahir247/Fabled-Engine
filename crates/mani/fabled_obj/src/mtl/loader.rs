pub struct MtlLoader;


impl MtlLoader {
    pub fn load<P: AsRef<std::path::Path>>(&self, mtl_path: P) {
        let file = std::fs::File::open(mtl_path).unwrap();
    }
}
