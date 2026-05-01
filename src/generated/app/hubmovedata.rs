
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmovedata/HubMoveData.md")))]
#[::unity2::class(namespace = "App", name = "HubMoveData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: hubmovedata :: HubMoveData >)]
pub struct HubMoveData {}

#[cfg(feature = "app-hubmovedata")]
#[::unity2::methods]
impl HubMoveData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_MoveType", args = 0)]
    pub fn get_move_type(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MoveType", args = 1)]
    pub fn set_move_type(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Locator", args = 0)]
    pub fn get_locator(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Locator", args = 1)]
    pub fn set_locator(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BodyName", args = 0)]
    pub fn get_body_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BodyName", args = 1)]
    pub fn set_body_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FaceName", args = 0)]
    pub fn get_face_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FaceName", args = 1)]
    pub fn set_face_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsTurn", args = 0)]
    pub fn get_is_turn(self) -> bool;

    #[method(name = "set_IsTurn", args = 1)]
    pub fn set_is_turn(self, value: bool) -> ();

    #[method(name = "get_StartSec", args = 0)]
    pub fn get_start_sec(self) -> f32;

    #[method(name = "set_StartSec", args = 1)]
    pub fn set_start_sec(self, value: f32) -> ();

    #[method(name = "get_EndSec", args = 0)]
    pub fn get_end_sec(self) -> f32;

    #[method(name = "set_EndSec", args = 1)]
    pub fn set_end_sec(self, value: f32) -> ();

    #[method(name = "get_MoveSpeed", args = 0)]
    pub fn get_move_speed(self) -> f32;

    #[method(name = "set_MoveSpeed", args = 1)]
    pub fn set_move_speed(self, value: f32) -> ();

    #[method(name = "get_MoveSec", args = 0)]
    pub fn get_move_sec(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MoveSec", args = 1)]
    pub fn set_move_sec(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmovedata")]
impl HubMoveData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMoveData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMoveDataMethods>::ctor(this);
        this
    }
}
