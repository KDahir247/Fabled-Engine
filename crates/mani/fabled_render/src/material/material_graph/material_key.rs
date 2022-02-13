slotmap::new_key_type! {
   pub struct MaterialKey;
}

unsafe impl smallvec::Array for MaterialKey {
    type Item = MaterialKey;

    // todo got to re-look at this.
    fn size() -> usize {
        usize::MAX
    }
}
