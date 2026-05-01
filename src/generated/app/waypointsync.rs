
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/waypointsync/WaypointSync.md")))]
#[::unity2::class(namespace = "App", name = "WaypointSync")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct WaypointSync {
    #[rename(name = "m_TargetAsset")]
    pub m_target_asset: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "_pastTargetAsset")]
    pub past_target_asset: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "waitTime")]
    pub wait_time: f64,
    #[rename(name = "m_relatives")]
    pub m_relatives: ::unity2::Array<crate::app::waypointsync::WaypointSync_Relative>,
}

#[cfg(feature = "app-waypointsync")]
#[::unity2::methods]
impl WaypointSync {
    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-waypointsync")]
impl WaypointSync {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WaypointSync),
                ::core::stringify!(new),
            )
        });
        <Self as IWaypointSyncMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/waypointsync/WaypointSync_Relative.md")))]
#[::unity2::class(namespace = "App", name = "WaypointSync.Relative")]
#[parent(crate::system::object::Object)]
pub struct WaypointSync_Relative {
    #[rename(name = "m_targetID")]
    pub m_target_id: i32,
    #[rename(name = "m_recieverID")]
    pub m_reciever_id: i32,
    #[rename(name = "m_offset")]
    pub m_offset: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Enable")]
    pub m_enable: bool,
    #[rename(name = "_pastTargetID")]
    pub past_target_id: i32,
    #[rename(name = "_pastRecieverID")]
    pub past_reciever_id: i32,
    #[rename(name = "_pastRecieverPos")]
    pub past_reciever_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "_pastEnable")]
    pub past_enable: bool,
    #[rename(name = "_targetPos")]
    pub target_pos: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "app-waypointsync")]
#[::unity2::methods]
impl WaypointSync_Relative {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-waypointsync")]
impl WaypointSync_Relative {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WaypointSync_Relative),
                ::core::stringify!(new),
            )
        });
        <Self as IWaypointSync_RelativeMethods>::ctor(this);
        this
    }
}
