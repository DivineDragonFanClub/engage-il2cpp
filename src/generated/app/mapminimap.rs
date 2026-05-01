
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapminimap/MapMiniMap_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapMiniMap_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapMiniMap_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapMiniMap.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapMiniMap_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapMiniMap_Label {
    pub fn stay() -> Self {
        Self { value: 0 }
    }

    pub fn measure() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapminimap/MapMiniMap.md")))]
#[::unity2::class(namespace = "App", name = "MapMiniMap")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mapminimap :: MapMiniMap >)]
pub struct MapMiniMap {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_MiniMapObject")]
    pub m_mini_map_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ModeStack")]
    pub m_mode_stack: crate::system::collections::generic::stack_1::Stack_1<
        crate::app::minimapcontroller::MiniMapController_Mode,
    >,
    #[rename(name = "m_MiniMapMode")]
    pub m_mini_map_mode: crate::app::minimapcontroller::MiniMapController_Mode,
}

#[cfg(feature = "app-mapminimap")]
#[::unity2::methods]
impl MapMiniMap {
    #[method(name = "CreateAsync", args = 1)]
    pub fn create_async(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy() -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CreateImpl", args = 0)]
    pub fn create_impl(self) -> ();

    #[method(name = "InitMiniMapObject", args = 0)]
    pub fn init_mini_map_object(self) -> ();

    #[method(name = "SetModeImpl", args = 1)]
    pub fn set_mode_impl(self, mode: crate::app::minimapcontroller::MiniMapController_Mode) -> ();

    #[method(name = "GetModeImpl", args = 0)]
    pub fn get_mode_impl(self) -> crate::app::minimapcontroller::MiniMapController_Mode;

    #[method(name = "PushModeImpl", args = 0)]
    pub fn push_mode_impl(self) -> ();

    #[method(name = "PopModeImpl", args = 0)]
    pub fn pop_mode_impl(self) -> ();

    #[method(name = "SetDirtyImpl", args = 0)]
    pub fn set_dirty_impl(self) -> ();

    #[method(name = "SetMode", args = 1)]
    pub fn set_mode(mode: crate::app::minimapcontroller::MiniMapController_Mode) -> ();

    #[method(name = "SetModeHide", args = 0)]
    pub fn set_mode_hide() -> ();

    #[method(name = "SetModeShow", args = 0)]
    pub fn set_mode_show() -> ();

    #[method(name = "SetModeMenu", args = 0)]
    pub fn set_mode_menu() -> ();

    #[method(name = "PushModeHide", args = 0)]
    pub fn push_mode_hide() -> ();

    #[method(name = "PopMode", args = 0)]
    pub fn pop_mode() -> ();

    #[method(name = "GetMode", args = 0)]
    pub fn get_mode(self) -> crate::app::minimapcontroller::MiniMapController_Mode;

    #[method(name = "SetDirty", args = 0)]
    pub fn set_dirty() -> ();

    #[method(name = "IsControl", args = 0)]
    pub fn is_control() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapminimap")]
impl MapMiniMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapMiniMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMiniMapMethods>::ctor(this);
        this
    }
}
