
use crate::app::hubmovestate::HubMoveState;
use crate::app::hubmovestate::IHubMoveState;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmovestatemovesplineloop/HubMoveStateMoveSplineLoop.md")))]
#[::unity2::class(namespace = "App", name = "HubMoveStateMoveSplineLoop")]
#[parent(crate::app::hubmovestate::HubMoveState)]
pub struct HubMoveStateMoveSplineLoop {
    #[rename(name = "m_unit")]
    pub m_unit: crate::app::hubunitcontroller::HubUnitController,
    #[rename(name = "m_data")]
    pub m_data: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_term")]
    pub m_term: f32,
    #[rename(name = "m_bodyAnim")]
    pub m_body_anim: ::unity2::Il2CppString,
    #[rename(name = "m_faceAnim")]
    pub m_face_anim: ::unity2::Il2CppString,
    #[rename(name = "m_isTurn")]
    pub m_is_turn: bool,
    #[rename(name = "m_resume")]
    pub m_resume: bool,
    #[rename(name = "m_speed")]
    pub m_speed: f32,
}

#[cfg(feature = "app-hubmovestatemovesplineloop")]
#[::unity2::methods]
impl HubMoveStateMoveSplineLoop {
    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        unit: crate::app::hubunitcontroller::HubUnitController,
        data: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        body_anim: ::unity2::Il2CppString,
        face_anim: ::unity2::Il2CppString,
        is_turn: bool,
        speed: f32,
    ) -> ();

    #[method(name = "IsEnd", args = 0)]
    pub fn is_end(self) -> bool;

    #[method(name = "Start", args = 1)]
    pub fn start(self, resume: bool) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnDrawGizmos", args = 0)]
    pub fn on_draw_gizmos(self) -> ();
}

#[cfg(feature = "app-hubmovestatemovesplineloop")]
impl HubMoveStateMoveSplineLoop {
    pub fn new(
        unit: crate::app::hubunitcontroller::HubUnitController,
        data: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        body_anim: ::unity2::Il2CppString,
        face_anim: ::unity2::Il2CppString,
        is_turn: bool,
        speed: f32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMoveStateMoveSplineLoop),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMoveStateMoveSplineLoopMethods>::ctor(
            this, unit, data, body_anim, face_anim, is_turn, speed,
        );
        this
    }
}
