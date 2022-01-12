use crate::FbxLoadError;

pub fn load_camera_handle(
    obj_handle: &fbxcel_dom::v7400::object::ObjectHandle,
) -> Result<(), FbxLoadError> {
    if let fbxcel_dom::v7400::object::TypedObjectHandle::NodeAttribute(
        fbxcel_dom::v7400::object::nodeattribute::TypedNodeAttributeHandle::Camera(camera_handle),
    ) = obj_handle.get_typed()
    {
        let direct_camera_properties = camera_handle
            .direct_properties()
            .ok_or(FbxLoadError::FBXPropertiesError("camera handle"))?;

        println!("{:?}", direct_camera_properties);

        // we now need to get the properties.

        //
    }
    Ok(())
}
