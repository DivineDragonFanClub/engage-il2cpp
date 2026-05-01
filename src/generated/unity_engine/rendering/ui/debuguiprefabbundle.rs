
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguiprefabbundle/DebugUIPrefabBundle.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.UI", name = "DebugUIPrefabBundle")]
#[parent(crate::system::object::Object)]
pub struct DebugUIPrefabBundle {
    #[rename(name = "type")]
    pub r#type: ::unity2::Il2CppString,
    #[rename(name = "prefab")]
    pub prefab: crate::unity_engine::recttransform::RectTransform,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguiprefabbundle")]
#[::unity2::methods]
impl DebugUIPrefabBundle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguiprefabbundle")]
impl DebugUIPrefabBundle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIPrefabBundle),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIPrefabBundleMethods>::ctor(this);
        this
    }
}
