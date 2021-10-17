use crate::LoadFlags;

pub struct ObjLoader {
    flags: u8,
}

impl Default for ObjLoader {
    fn default() -> Self {
        Self {
            flags: LoadFlags::default().bits(),
        }
    }
}

impl ObjLoader {
    pub fn new(load_option: LoadFlags) -> Self {
        Self {
            flags: load_option.bits(),
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(&self, obj_path: P) {
        let file = std::fs::File::open(obj_path).unwrap();
        let file_metadata = file.metadata().unwrap();

        let mut obj_file_buffer =
            std::io::BufReader::with_capacity(file_metadata.len() as usize, file);

        let load_flags = LoadFlags::from_bits(self.flags).unwrap_or_default();

        let load_result =
            tobj::load_obj_buf(&mut obj_file_buffer, &load_flags.into(), |mtl_path| {
                // We don't want to load mtl from this script.
                println!("{}", mtl_path.display());
                Err(tobj::LoadError::GenericFailure)
            });
    }
}


#[cfg(test)]
mod obj_loader_test {
    use crate::ObjLoader;

    #[test]
    fn load_obj() {
        let obj = ObjLoader::default();
        obj.load("")
    }
}
