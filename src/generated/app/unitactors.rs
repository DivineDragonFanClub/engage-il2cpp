
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitactors/UnitActors.md")))]
#[::unity2::class(namespace = "App", name = "UnitActors")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: unitactors :: UnitActors >)]
pub struct UnitActors {
    #[rename(name = "m_Actors")]
    pub m_actors:
        crate::system::collections::generic::list_1::List_1<crate::app::unitactor::UnitActor>,
    #[rename(name = "m_ChangeEnableInfoList")]
    pub m_change_enable_info_list:
        crate::system::collections::generic::list_1::List_1<crate::app::mapinforoot::MapInfoRoot>,
    #[rename(name = "m_ChangeDisableInfoList")]
    pub m_change_disable_info_list:
        crate::system::collections::generic::list_1::List_1<crate::app::mapinforoot::MapInfoRoot>,
    #[rename(name = "m_UnitPositionScales")]
    pub m_unit_position_scales: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    #[static_field]
    #[rename(name = "CanvasVisibilityChangeCountParFrame")]
    pub canvas_visibility_change_count_par_frame: i32,
}

#[cfg(feature = "app-unitactors")]
#[::unity2::methods]
impl UnitActors {
    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "CreateActorImpl", args = 1)]
    pub fn create_actor_impl(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "DeleteActorImpl", args = 1)]
    pub fn delete_actor_impl(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CreateActor", args = 1)]
    pub fn create_actor(unit: crate::app::unit::Unit) -> ();

    #[method(name = "DeleteActor", args = 1)]
    pub fn delete_actor(unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitactors")]
impl UnitActors {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitActors),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitActorsMethods>::ctor(this);
        this
    }
}
