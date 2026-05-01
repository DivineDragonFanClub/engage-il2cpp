
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/cameratypeutility/CameraTypeUtility.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "CameraTypeUtility"
)]
#[parent(crate::system::object::Object)]
pub struct CameraTypeUtility {
    #[static_field]
    #[rename(name = "s_CameraTypeNames")]
    pub s_camera_type_names: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "unity_engine-rendering-universal-cameratypeutility")]
#[::unity2::methods]
impl CameraTypeUtility {
    #[method(name = "GetName", args = 1)]
    pub fn get_name(
        r#type: crate::unity_engine::rendering::universal::camerarendertype::CameraRenderType,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
