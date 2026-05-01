
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/areainspector/AreaInspector.md")))]
#[::unity2::class(namespace = "App", name = "AreaInspector")]
#[parent(crate::app::mapinspector::MapInspector)]
pub struct AreaInspector {
    #[rename(name = "m_X1")]
    pub m_x1: i32,
    #[rename(name = "m_Z1")]
    pub m_z1: i32,
    #[rename(name = "m_X2")]
    pub m_x2: i32,
    #[rename(name = "m_Z2")]
    pub m_z2: i32,
    #[rename(name = "m_Force")]
    pub m_force: i32,
}

#[cfg(feature = "app-areainspector")]
#[::unity2::methods]
impl AreaInspector {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "IsEanble", args = 3)]
    pub fn is_eanble(self, x: i32, z: i32, force: i32) -> bool;

    #[method(name = "get_X1", args = 0)]
    pub fn get_x1(self) -> i32;

    #[method(name = "get_Z1", args = 0)]
    pub fn get_z1(self) -> i32;

    #[method(name = "get_X2", args = 0)]
    pub fn get_x2(self) -> i32;

    #[method(name = "get_Z2", args = 0)]
    pub fn get_z2(self) -> i32;

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;
}

#[cfg(feature = "app-areainspector")]
impl AreaInspector {
    pub fn new(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AreaInspector),
                ::core::stringify!(new),
            )
        });
        <Self as IAreaInspectorMethods>::ctor(this, args);
        this
    }
}
