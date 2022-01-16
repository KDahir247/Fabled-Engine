use crate::loader::AspectRatioLoader;
use crate::loader::CameraFormatLoader;
use crate::loader::CameraProjectionTypeLoader;
use crate::prop_proxy_getters;
type PrimitiveLoader<T> = fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader<T>;
type RgbLoader<T> = fbxcel_dom::v7400::object::property::loaders::RgbLoader<T>;

pub struct CameraProperties<'a> {
    prop_handle: fbxcel_dom::v7400::object::property::PropertiesHandle<'a>,
}

impl<'a> CameraProperties<'a> {
    pub(crate) fn new(
        camera_handle: &'a fbxcel_dom::v7400::object::nodeattribute::CameraHandle,
    ) -> Option<Self> {
        let direct_camera_properties = camera_handle.direct_properties();

        direct_camera_properties.map(|valid_camera_properties| Self {
            prop_handle: valid_camera_properties,
        })
    }

    prop_proxy_getters! {

        camera_projection -> fabled_render::camera::Projection{
            name = "CameraProjectionType",
            loader = CameraProjectionTypeLoader::default(),
            description = "camera projection type",
            default : {
                camera_projection_or_default = fabled_render::camera::Projection::Perspective(fabled_render::camera::Perspective::default())
            }
        }

        aspect_ratio_mode -> fabled_render::camera::AspectRatioMode{
            name = "AspectRatioMode",
            loader = AspectRatioLoader::default(),
            description = "camera aspect ratio mode",
            default : {
                aspect_ratio_mode_or_default  = fabled_render::camera::AspectRatioMode::WindowSize
            }
        }

        aspect_width -> f32{
            name = "AspectWidth",
            loader = PrimitiveLoader::<f32>::default(),
            description = "aspect width value or an undefined value if aspect ratio mode is set to WINDOW_SIZE",
            default : {
                aspect_width_or_default = 320.0
            }
        }

        aspect_height -> f32{
            name = "AspectHeight",
            loader = PrimitiveLoader::<f32>::default(),
            description = "aspect height value or an undefined value if aspect ratio mode is set to WINDOW_SIZE",
            default : {
                aspect_height_or_default = 200.0
            }
        }

        pixel_aspect_ratio -> f32{
            name = "PixelAspectRatio",
            loader = PrimitiveLoader::<f32>::default(),
            description = "pixel aspect ratio for the camera",
            default : {
                pixel_aspect_ratio_or_default = 1.0
            }
        }


        field_of_view -> f32{
            name = "FieldOfView",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the angle (degree) of view base on the given focal length, aperture width, and aperture height (x = diagonal of film, f = focal length) FOV = 2 arctan(x/ (2 * f))",
            default : {
                field_of_view_or_default = 90.0
            }
        }

        field_of_view_X -> f32{
            name = "FieldOfViewX",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the horizontal field of view angle (degree)",
            default : {
                field_of_view_X_or_default = 90.0
            }
        }

        field_of_view_Y -> f32{
            name = "FieldOfViewY",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the vertical field of view angle (degree)",
            default : {
                field_of_view_Y_or_default = 77.3196197066
            }
        }

        optical_center_X -> f32{
            name = "OpticalCenterX",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the optical center X (pixels)",
            default : {
                optical_center_X_or_default = 0.0
            }
        }

        optical_center_Y -> f32{
            name = "OpticalCenterY",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the optical center y (pixels)",
            default : {
                optical_center_Y_or_default = 0.0
            }
        }

        focal_length -> f32{
            name = "FocalLength",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the camera focal length (millimeters)",
            default : {
                focal_length_or_default = 15.0
            }
        }

        film_squeeze_ratio -> f32{
            name = "FilmSqueezeRatio",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the camera aperture squeeze ratio",
            default : {
                film_squeeze_ratio_or_default = 1.0
            }
        }

        film_aspect_ratio -> f32{
            name = "FilmAspectRatio",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the camera aperture aspect ratio (film width / film height)",
            default : {
                //we will use a SUPER_35MM as default
                // 24.89 mm width
                // 18.66 mm height
                // 24.89 mm / 18.66 mm
                film_aspect_ratio_or_default =1.333869239
            }
        }

        film_width -> f32{
            name = "FilmWidth",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the camera film aperture width in inches",
            default : {
                //we will use a SUPER_35MM as default
                film_width_or_default = 0.97992126
            }
        }

        film_height -> f32{
            name = "FilmHeight",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the camera film aperture height in inches",
            default : {
                //we will use a SUPER_35MM as default
                film_height_or_default = 0.73464567
            }
        }

        ortho_zoom -> f32{
            name = "OrthoZoom",
            loader = PrimitiveLoader::<f32>::default(),
            description = "the camera ortho zoom",
            default : {
                ortho_zoom_or_default = 1.0
            }
        }

        near_plane -> f32{
            name = "NearPlane",
            loader = PrimitiveLoader::<f32>::default(),
            description = "The near plane is the minimum distance to render a scene on the camera display",
            default : {
                near_plane_or_default = 0.01
            }
        }

        far_plane -> f32{
            name = "FarPlane",
            loader = PrimitiveLoader::<f32>::default(),
            description = "The far plane is the maximum distance to render a scene on the camera display",
            default : {
                far_plane_or_default = 1000.0
            }
        }

        camera_format -> fabled_render::camera::CameraFormat{
            name = "CameraFormat",
            loader = CameraFormatLoader::default(),
            description = "camera format type",
            default : {
                camera_format_or_default =  fabled_render::camera::CameraFormat::FullScreen
            }
        }

    }
}
