use thiserror::*;
#[derive(Error, Debug)]
pub enum FbxLoadError {
    #[error("Fbx header failed to parse due to invalid format or missing magic binary")]
    FbxHeaderError,
    #[error("Failed to get control point cpi = {0:?}")]
    ControlPointError(fbxcel_dom::v7400::data::mesh::ControlPointIndex),
    #[error("Unsupported reference mode for normals in the normal layer")]
    FBXLayerNormalError,
    #[error("Unsupported reference mode for uvs in the uv layer")]
    FBXLayerUVError,
    #[error("Unsupported reference mode for materials in the material layer")]
    FBXLayerMaterialError,
    #[error("Fbx parse error")]
    FBXParseError,
    #[error("FBX invalid version only 7400 and 7500 are supported")]
    FBXInvalidVersionError,
    #[error("Failed to retrieve direct properties from handle : {0:?}")]
    FBXPropertiesError(&'static str),
    #[error("")]
    FBXPropertyRetrievalError {
        property_name: &'static str,
        desc: &'static str,
    },
    #[error(transparent)]
    FBXIOError(#[from] std::io::Error),
}
