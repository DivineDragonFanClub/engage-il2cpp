
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/meshgroundpaintgetter/MeshGroundPaintGetter.md")))]
#[::unity2::class(namespace = "App", name = "MeshGroundPaintGetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MeshGroundPaintGetter {
    #[rename(name = "m_PaintDataArray")]
    pub m_paint_data_array: ::unity2::Array<crate::app::meshgroundpaintdata::MeshGroundPaintData>,
}

#[cfg(feature = "app-meshgroundpaintgetter")]
#[::unity2::methods]
impl MeshGroundPaintGetter {
    #[method(name = "Get", args = 2)]
    pub fn get(self, x: f32, y: f32) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-meshgroundpaintgetter")]
impl MeshGroundPaintGetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MeshGroundPaintGetter),
                ::core::stringify!(new),
            )
        });
        <Self as IMeshGroundPaintGetterMethods>::ctor(this);
        this
    }
}
