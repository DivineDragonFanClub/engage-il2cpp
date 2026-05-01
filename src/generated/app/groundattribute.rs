
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/groundattribute/GroundAttribute.md")))]
#[::unity2::class(namespace = "App", name = "GroundAttribute")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: groundattribute :: GroundAttribute >)]
pub struct GroundAttribute {}

#[cfg(feature = "app-groundattribute")]
#[::unity2::methods]
impl GroundAttribute {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Label", args = 1)]
    pub fn set_label(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Sound", args = 0)]
    pub fn get_sound(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Sound", args = 1)]
    pub fn set_sound(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Particle", args = 0)]
    pub fn get_particle(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Particle", args = 1)]
    pub fn set_particle(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-groundattribute")]
impl GroundAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GroundAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IGroundAttributeMethods>::ctor(this);
        this
    }
}
