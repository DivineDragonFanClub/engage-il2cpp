
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridetargetgroup/DragonRideTargetGroup_ChainSEManager.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideTargetGroup.ChainSEManager")]
#[parent(crate::system::object::Object)]
pub struct DragonRideTargetGroup_ChainSEManager {
    #[static_field]
    #[rename(name = "cBigChainSE_Normal")]
    pub c_big_chain_se_normal: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cBigChainSE_Special")]
    pub c_big_chain_se_special: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cBigChainSE_Double")]
    pub c_big_chain_se_double: ::unity2::Il2CppString,
    #[rename(name = "cLinkChainSE")]
    pub c_link_chain_se: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "cLinkSETable")]
    pub c_link_se_table: ::unity2::Array<i32>,
    #[rename(name = "m_RootID")]
    pub m_root_id: ::unity2::Il2CppString,
    #[rename(name = "m_ChainCount")]
    pub m_chain_count: i32,
    #[rename(name = "m_IsLink")]
    pub m_is_link: bool,
    #[rename(name = "m_IsSpecial")]
    pub m_is_special: bool,
    #[rename(name = "m_IsDoubleEx")]
    pub m_is_double_ex: bool,
    #[rename(name = "m_AliveLimitCounter")]
    pub m_alive_limit_counter: f32,
}

#[cfg(feature = "app-dragonridetargetgroup")]
#[::unity2::methods]
impl DragonRideTargetGroup_ChainSEManager {
    #[method(name = "get_IsAlive", args = 0)]
    pub fn get_is_alive(self) -> bool;

    #[method(name = "set_IsAlive", args = 1)]
    pub fn set_is_alive(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ResetChain", args = 0)]
    pub fn reset_chain(self) -> ();

    #[method(name = "StartChain", args = 4)]
    pub fn start_chain(
        self,
        root_id: ::unity2::Il2CppString,
        is_link: bool,
        is_special: bool,
        is_double_ex: bool,
    ) -> ();

    #[method(name = "TryPlayChainSE", args = 1)]
    pub fn try_play_chain_se(
        self,
        target_script: crate::app::dragonridetarget::DragonRideTarget,
    ) -> ();

    #[method(name = "TickLimit", args = 0)]
    pub fn tick_limit(self) -> ();
}

#[cfg(feature = "app-dragonridetargetgroup")]
impl DragonRideTargetGroup_ChainSEManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideTargetGroup_ChainSEManager),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideTargetGroup_ChainSEManagerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridetargetgroup/DragonRideTargetGroup_ChainParam.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DragonRideTargetGroup_ChainParam {
    pub script: crate::app::dragonridetarget::DragonRideTarget,
    pub is_link: bool,
    pub is_double_ex: bool,
}

impl ::unity2::ClassIdentity for DragonRideTargetGroup_ChainParam {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideTargetGroup.ChainParam";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideTargetGroup_ChainParam {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridetargetgroup/DragonRideTargetGroup.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideTargetGroup")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideTargetGroup {
    #[static_field]
    #[rename(name = "cTargetPrefabPath")]
    pub c_target_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_BillboardType")]
    pub m_billboard_type: crate::app::dragon_ride::billboardtypes::BillboardTypes,
    #[rename(name = "m_MapWidth")]
    pub m_map_width: i32,
    #[rename(name = "m_MapHeight")]
    pub m_map_height: i32,
    #[rename(name = "m_ChainIntervalTimer")]
    pub m_chain_interval_timer: f32,
    #[rename(name = "m_GameCamera")]
    pub m_game_camera: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CameraScript")]
    pub m_camera_script: crate::app::dragonridecamera::DragonRideCamera,
    #[rename(name = "m_Config")]
    pub m_config: crate::app::dragonrideconfig::DragonRideConfig,
    #[rename(name = "m_TargetSpace")]
    pub m_target_space: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_ChainPlayer")]
    pub m_chain_player: crate::system::collections::generic::list_1::List_1<
        crate::app::dragonridetargetgroup::DragonRideTargetGroup_ChainSEManager,
    >,
    #[rename(name = "m_ChainChecker")]
    pub m_chain_checker: crate::system::collections::generic::list_1::List_1<
        crate::app::dragonridetargetgroup::DragonRideTargetGroup_ChainParam,
    >,
}

#[cfg(feature = "app-dragonridetargetgroup")]
#[::unity2::methods]
impl DragonRideTargetGroup {
    #[method(name = "get_IsSearchGroup", args = 0)]
    pub fn get_is_search_group(self) -> bool;

    #[method(name = "set_IsSearchGroup", args = 1)]
    pub fn set_is_search_group(self, value: bool) -> ();

    #[method(name = "get_IsExecuteChain", args = 0)]
    pub fn get_is_execute_chain(self) -> bool;

    #[method(name = "set_IsExecuteChain", args = 1)]
    pub fn set_is_execute_chain(self, value: bool) -> ();

    #[method(name = "get_IsEventMode", args = 0)]
    pub fn get_is_event_mode(self) -> bool;

    #[method(name = "set_IsEventMode", args = 1)]
    pub fn set_is_event_mode(self, value: bool) -> ();

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = "set_IsVisible", args = 1)]
    pub fn set_is_visible(self, value: bool) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "PostUpdate", args = 0)]
    pub fn post_update(self) -> ();

    #[method(name = "SetEventMode", args = 1)]
    pub fn set_event_mode(self, flag: bool) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "GetChildScript", args = 2)]
    pub fn get_child_script(self, h: i32, w: i32)
        -> crate::app::dragonridetarget::DragonRideTarget;

    #[method(name = "ExecuteChild", args = 4)]
    pub fn execute_child(self, h: i32, w: i32, flag: i32, root_id: ::unity2::Il2CppString) -> ();

    #[method(name = "CheckStartChain", args = 0)]
    pub fn check_start_chain(self) -> bool;

    #[method(name = "UpdateChain", args = 0)]
    pub fn update_chain(self) -> ();

    #[method(name = "SetChainExecute", args = 0)]
    pub fn set_chain_execute(self) -> ();

    #[method(name = "GetNearestTarget", args = 4)]
    pub fn get_nearest_target(
        self,
        return_pos: crate::unity_engine::vector3::Vector3,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
    ) -> f32;

    #[method(name = "DestroyBombTarget", args = 3)]
    pub fn destroy_bomb_target(
        self,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
    ) -> ();

    #[method(name = "DestroySpecialTarget", args = 3)]
    pub fn destroy_special_target(
        self,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
    ) -> ();

    #[method(name = "ExecuteSelectTypeTarget", args = 4)]
    pub fn execute_select_type_target(
        self,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
        select: crate::app::dragonridetarget::DragonRideTarget_TargetType,
    ) -> ();

    #[method(name = "CheckEnableBombTarget", args = 3)]
    pub fn check_enable_bomb_target(
        self,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
    ) -> bool;

    #[method(name = "CheckEnableSpecialTarget", args = 3)]
    pub fn check_enable_special_target(
        self,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
    ) -> bool;

    #[method(name = "CheckEnableSelectTypeTarget", args = 4)]
    pub fn check_enable_select_type_target(
        self,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
        select: crate::app::dragonridetarget::DragonRideTarget_TargetType,
    ) -> bool;

    #[method(name = "SetEnableSelectTypeTargetList", args = 5)]
    pub fn set_enable_select_type_target_list(
        self,
        camera_pos: crate::unity_engine::vector3::Vector3,
        camera_forward: crate::unity_engine::vector3::Vector3,
        enable_rate: f32,
        select: crate::app::dragonridetarget::DragonRideTarget_TargetType,
        ref_list: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonridetargetgroup")]
impl DragonRideTargetGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideTargetGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideTargetGroupMethods>::ctor(this);
        this
    }
}
