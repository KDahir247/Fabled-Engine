pub fn load_light_handle(obj_handle: &fbxcel_dom::v7400::object::ObjectHandle) {
    if let fbxcel_dom::v7400::object::TypedObjectHandle::NodeAttribute(
        fbxcel_dom::v7400::object::nodeattribute::TypedNodeAttributeHandle::Light(light_handle),
    ) = obj_handle.get_typed()
    {
        let light_handle = crate::properties::LightProperties::new(&light_handle).unwrap();
        let intesity = light_handle.light_decay_type().unwrap().unwrap();
        // let diffuse_factor = light_handle.diffuse_factor().unwrap();
        println!("{:?}", intesity);
        // calculate
    }
}
