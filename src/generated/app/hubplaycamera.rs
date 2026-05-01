
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaycamera/HubPlayCamera_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubPlayCamera_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubPlayCamera_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubPlayCamera.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubPlayCamera_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubPlayCamera_Label {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn init() -> Self {
        Self { value: 1 }
    }

    pub fn main() -> Self {
        Self { value: 2 }
    }

    pub fn exit() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaycamera/HubPlayCamera.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayCamera")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubPlayCamera {
    #[rename(name = "CameraObject")]
    pub camera_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "MainCamera")]
    pub main_camera: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "AssetPath")]
    pub asset_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "DisableLodCrossFadeTag")]
    pub disable_lod_cross_fade_tag: ::unity2::Il2CppString,
    #[rename(name = "m_distance")]
    pub m_distance: crate::system::collections::generic::list_1::List_1<f32>,
}

#[cfg(feature = "app-hubplaycamera")]
#[::unity2::methods]
impl HubPlayCamera {
    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::hubdemodata::HubDemoData;

    #[method(name = "get_CameraHandle", args = 0)]
    pub fn get_camera_handle(self) -> crate::app::resourcehandle_2::ResourceHandle_2;

    #[method(name = "set_CameraHandle", args = 1)]
    pub fn set_camera_handle(self, value: crate::app::resourcehandle_2::ResourceHandle_2) -> ();

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::app::hubdemodata::HubDemoData) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Main", args = 0)]
    pub fn main(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "OpenMessage", args = 0)]
    pub fn open_message(self) -> ();

    #[method(name = "OpenTutorial", args = 0)]
    pub fn open_tutorial(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        data: crate::app::hubdemodata::HubDemoData,
    ) -> ();
}

#[cfg(feature = "app-hubplaycamera")]
impl HubPlayCamera {
    pub fn new(data: crate::app::hubdemodata::HubDemoData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayCameraMethods>::ctor(this, data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaycamera/HubPlayCamera_DemoTelop.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayCamera.DemoTelop")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubPlayCamera_DemoTelop {
    #[rename(name = "Root")]
    pub root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "UIObject")]
    pub ui_object: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "UIAssetPath")]
    pub ui_asset_path: ::unity2::Il2CppString,
    #[rename(name = "m_animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-hubplaycamera")]
#[::unity2::methods]
impl HubPlayCamera_DemoTelop {
    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::hubdemodata::HubDemoData;

    #[method(name = "get_UIHandle", args = 0)]
    pub fn get_ui_handle(self) -> crate::app::resourcehandle_2::ResourceHandle_2;

    #[method(name = "set_UIHandle", args = 1)]
    pub fn set_ui_handle(self, value: crate::app::resourcehandle_2::ResourceHandle_2) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::app::hubdemodata::HubDemoData) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateDefaultDesc", args = 0)]
    pub fn create_default_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        data: crate::app::hubdemodata::HubDemoData,
    ) -> ();
}

#[cfg(feature = "app-hubplaycamera")]
impl HubPlayCamera_DemoTelop {
    pub fn new(data: crate::app::hubdemodata::HubDemoData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayCamera_DemoTelop),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayCamera_DemoTelopMethods>::ctor(this, data);
        this
    }
}
