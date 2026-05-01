
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/xrlayout/XRLayout.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct XRLayout {
    pub camera: crate::unity_engine::camera::Camera,
    pub xr_system: crate::unity_engine::rendering::universal::xrsystem::XRSystem,
}

impl ::unity2::ClassIdentity for XRLayout {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";

    const NAME: &'static str = "XRLayout";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for XRLayout {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-universal-xrlayout")]
#[::unity2::methods(value)]
impl XRLayout {
    #[method(name = "CreatePass", args = 1)]
    pub fn create_pass(
        self,
        pass_create_info : crate :: unity_engine :: rendering :: universal :: xrpasscreateinfo :: XRPassCreateInfo,
    ) -> crate::unity_engine::rendering::universal::xrpass::XRPass;

    #[method(name = "AddViewToPass", args = 2)]
    pub fn add_view_to_pass(
        self,
        view_create_info : crate :: unity_engine :: rendering :: universal :: xrviewcreateinfo :: XRViewCreateInfo,
        pass: crate::unity_engine::rendering::universal::xrpass::XRPass,
    ) -> ();
}
