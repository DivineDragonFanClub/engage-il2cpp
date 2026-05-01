
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplayercontroller/HubPlayerController.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayerController")]
#[parent(crate::system::object::Object)]
pub struct HubPlayerController {
    #[rename(name = "m_PlayerRoot")]
    pub m_player_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PlayerCollider")]
    pub m_player_collider: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GroupRoot")]
    pub m_group_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ButtonNavi")]
    pub m_button_navi: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LookAt")]
    pub m_look_at: crate::app::hublookatcontroller::HubLookAtController,
    #[rename(name = "m_UnitController")]
    pub m_unit_controller: crate::app::hubunitcontroller::HubUnitController,
    #[rename(name = "m_Colliders")]
    pub m_colliders: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::collider::Collider,
    >,
    #[rename(name = "m_HubCamera")]
    pub m_hub_camera: crate::app::hubcamera::HubCamera,
    #[rename(name = "m_LastAccess")]
    pub m_last_access: crate::app::hubaccess::HubAccess,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CharacterJoints")]
    pub m_character_joints: ::unity2::Array<crate::combat::characterjoint::CharacterJoint>,
    #[rename(name = "m_Speed")]
    pub m_speed: f32,
    #[rename(name = "m_MoveTarget")]
    pub m_move_target: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_MoveDirection")]
    pub m_move_direction: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_DashStop")]
    pub m_dash_stop: bool,
    #[rename(name = "m_DashStopTime")]
    pub m_dash_stop_time: f32,
    #[rename(name = "m_DashStopDelay")]
    pub m_dash_stop_delay: f32,
    #[rename(name = "m_DashTime")]
    pub m_dash_time: f32,
    #[rename(name = "m_TimeWithNoTarget")]
    pub m_time_with_no_target: f32,
    #[rename(name = "m_QuickTurnAnimName")]
    pub m_quick_turn_anim_name: ::unity2::Il2CppString,
    #[rename(name = "m_AccessDelay")]
    pub m_access_delay: f32,
    #[rename(name = "m_CullingCollider")]
    pub m_culling_collider: crate::app::hubcullingplayercollider::HubCullingPlayerCollider,
    #[rename(name = "m_GrassManagers")]
    pub m_grass_managers: ::unity2::Array<crate::app::grassmanager::GrassManager>,
    #[rename(name = "ObjectCollisionLayerMask")]
    pub object_collision_layer_mask: i32,
    #[rename(name = "GroundCollisionLayerMask")]
    pub ground_collision_layer_mask: i32,
    #[rename(name = "SlopeCollisionLayerMask")]
    pub slope_collision_layer_mask: i32,
    #[rename(name = "m_hash")]
    pub m_hash: i32,
    #[rename(name = "results")]
    pub results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
    #[rename(name = "overlapColliders")]
    pub overlap_colliders: ::unity2::Array<crate::unity_engine::collider::Collider>,
    #[rename(name = "accessColliders")]
    pub access_colliders: ::unity2::Array<crate::unity_engine::collider::Collider>,
    #[rename(name = "accessCollider")]
    pub access_collider: crate::unity_engine::collider::Collider,
    #[rename(name = "distanceSpeed")]
    pub distance_speed: f32,
    #[rename(name = "m_procIdleCoroutin")]
    pub m_proc_idle_coroutin: crate::unity_engine::coroutine::Coroutine,
    #[rename(name = "m_enableProc")]
    pub m_enable_proc: bool,
    #[rename(name = "m_WallInterval")]
    pub m_wall_interval: f32,
}

#[cfg(feature = "app-hubplayercontroller")]
#[::unity2::methods]
impl HubPlayerController {
    #[method(name = "get_Dir", args = 0)]
    pub fn get_dir(self) -> f32;

    #[method(name = "set_Dir", args = 1)]
    pub fn set_dir(self, value: f32) -> ();

    #[method(name = "get_PrevDir", args = 0)]
    pub fn get_prev_dir(self) -> f32;

    #[method(name = "set_PrevDir", args = 1)]
    pub fn set_prev_dir(self, value: f32) -> ();

    #[method(name = "get_ZRotate", args = 0)]
    pub fn get_z_rotate(self) -> f32;

    #[method(name = "set_ZRotate", args = 1)]
    pub fn set_z_rotate(self, value: f32) -> ();

    #[method(name = "get_IsStop", args = 0)]
    pub fn get_is_stop(self) -> bool;

    #[method(name = "set_IsStop", args = 1)]
    pub fn set_is_stop(self, value: bool) -> ();

    #[method(name = "get_Speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "set_Speed", args = 1)]
    pub fn set_speed(self, value: f32) -> ();

    #[method(name = "get_Character", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "set_Character", args = 1)]
    pub fn set_character(self, value: crate::combat::character::Character) -> ();

    #[method(name = "get_UnitController", args = 0)]
    pub fn get_unit_controller(self) -> crate::app::hubunitcontroller::HubUnitController;

    #[method(name = "get_CullingCollider", args = 0)]
    pub fn get_culling_collider(
        self,
    ) -> crate::app::hubcullingplayercollider::HubCullingPlayerCollider;

    #[method(name = "get_ControlType", args = 0)]
    pub fn get_control_type(self) -> i32;

    #[method(name = "get_RotateAngle", args = 0)]
    pub fn get_rotate_angle(self) -> f32;

    #[method(name = "get_DefaultRadius", args = 0)]
    pub fn get_default_radius(self) -> f32;

    #[method(name = "Setup", args = 3)]
    pub fn setup(
        self,
        player_root: crate::unity_engine::gameobject::GameObject,
        group_root: crate::unity_engine::gameobject::GameObject,
        button_navi: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "SetCollider", args = 1)]
    pub fn set_collider(self, collider: crate::app::hubplayercollider::HubPlayerCollider) -> ();

    #[method(name = "SetCullingCollider", args = 1)]
    pub fn set_culling_collider(
        self,
        collider: crate::app::hubcullingplayercollider::HubCullingPlayerCollider,
    ) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "LoadCharacter", args = 0)]
    pub fn load_character(self) -> ();

    #[method(name = "SaveAccessory", args = 0)]
    pub fn save_accessory(self) -> ();

    #[method(name = "RestoreAccessory", args = 0)]
    pub fn restore_accessory(self) -> ();

    #[method(name = "Reload", args = 0)]
    pub fn reload(self) -> ();

    #[method(name = "get_IsCharacterLoading", args = 0)]
    pub fn get_is_character_loading(self) -> bool;

    #[method(name = "set_IsCharacterLoading", args = 1)]
    pub fn set_is_character_loading(self, value: bool) -> ();

    #[method(name = "InitLookAtTarget", args = 0)]
    pub fn init_look_at_target(self) -> ();

    #[method(name = "UpdateCharacterLookAt", args = 0)]
    pub fn update_character_look_at(self) -> ();

    #[method(name = "Tick", args = 2)]
    pub fn tick(self, lx: f32, ly: f32) -> ();

    #[method(name = "StartProcIdle", args = 0)]
    pub fn start_proc_idle(self) -> ();

    #[method(name = "StopProcIdle", args = 1)]
    pub fn stop_proc_idle(self, soon: bool) -> ();

    #[method(name = "Persistent", args = 0)]
    pub fn persistent(self) -> ();

    #[method(name = "ProcIdle", args = 0)]
    pub fn proc_idle(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "LatePersistent", args = 0)]
    pub fn late_persistent(self) -> ();

    #[method(name = "UpdateAccess", args = 1)]
    pub fn update_access(self, force: bool) -> ();

    #[method(name = "UpdateLookAt", args = 0)]
    pub fn update_look_at(self) -> ();

    #[method(name = "FindNearestLookAtPoint", args = 0)]
    pub fn find_nearest_look_at_point(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetTransform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_CanAccess", args = 0)]
    pub fn get_can_access(self) -> bool;

    #[method(name = "BeginAccess", args = 0)]
    pub fn begin_access(self) -> ();

    #[method(name = "EndAccess", args = 2)]
    pub fn end_access(self, with_reset_anim: bool, delay: f32) -> ();

    #[method(name = "ResetLook", args = 0)]
    pub fn reset_look(self) -> ();

    #[method(name = "TryGetLastAccess", args = 0)]
    pub fn try_get_last_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "TryGetNowAccess", args = 0)]
    pub fn try_get_now_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "AddCollider", args = 1)]
    pub fn add_collider(self, c: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "RemoveCollider", args = 1)]
    pub fn remove_collider(self, c: crate::unity_engine::collider::Collider) -> ();

    #[method(name = "IsMoving", args = 0)]
    pub fn is_moving(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubplayercontroller")]
impl HubPlayerController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayerController),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayerControllerMethods>::ctor(this);
        this
    }
}
