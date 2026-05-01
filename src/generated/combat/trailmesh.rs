
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/trailmesh/TrailMesh.md")))]
#[::unity2::class(namespace = "Combat", name = "TrailMesh")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TrailMesh {
    #[rename(name = "NumberOfJoints__ReadOnly")]
    pub number_of_joints_read_only: ::unity2::Il2CppString,
    #[rename(name = "JointsRoot")]
    pub joints_root: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::transform::Transform,
    >,
    #[rename(name = "JointsTip")]
    pub joints_tip: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::transform::Transform,
    >,
    #[rename(name = "m_Renderer")]
    pub m_renderer: crate::unity_engine::skinnedmeshrenderer::SkinnedMeshRenderer,
    #[rename(name = "m_Material")]
    pub m_material: crate::unity_engine::material::Material,
    #[rename(name = "m_HashColor")]
    pub m_hash_color: i32,
}

#[cfg(feature = "combat-trailmesh")]
#[::unity2::methods]
impl TrailMesh {
    #[method(name = "get_PosRoot", args = 0)]
    pub fn get_pos_root(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "set_PosRoot", args = 1)]
    pub fn set_pos_root(self, value: ::unity2::Array<crate::unity_engine::vector3::Vector3>) -> ();

    #[method(name = "get_PosTip", args = 0)]
    pub fn get_pos_tip(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "set_PosTip", args = 1)]
    pub fn set_pos_tip(self, value: ::unity2::Array<crate::unity_engine::vector3::Vector3>) -> ();

    #[method(name = "get_Alpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "set_Alpha", args = 1)]
    pub fn set_alpha(self, value: f32) -> ();

    #[method(name = "get_IsAlive", args = 0)]
    pub fn get_is_alive(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-trailmesh")]
impl TrailMesh {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TrailMesh),
                ::core::stringify!(new),
            )
        });
        <Self as ITrailMeshMethods>::ctor(this);
        this
    }
}
