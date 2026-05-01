
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitlistformapcombatviewer/UnitListForMapCombatViewer.md")))]
#[::unity2::class(namespace = "App", name = "UnitListForMapCombatViewer")]
#[parent(crate::system::object::Object)]
pub struct UnitListForMapCombatViewer {}

#[cfg(feature = "app-unitlistformapcombatviewer")]
#[::unity2::methods]
impl UnitListForMapCombatViewer {
    #[method(name = "get_UnitNameWithoutNone", args = 0)]
    pub fn get_unit_name_without_none() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_UnitNameWithoutNone", args = 1)]
    pub fn set_unit_name_without_none(value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_UnitName", args = 0)]
    pub fn get_unit_name() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_UnitName", args = 1)]
    pub fn set_unit_name(value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_EmblemName", args = 0)]
    pub fn get_emblem_name() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_EmblemName", args = 1)]
    pub fn set_emblem_name(value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "Pid2DispName", args = 1)]
    pub fn pid2_disp_name(pid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "DispNameToPid", args = 1)]
    pub fn disp_name_to_pid(disp_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;
}
