
use crate::app::mapinspector::IMapInspector;
use crate::app::mapinspector::MapInspector;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cannoninspector/CannonInspector.md")))]
#[::unity2::class(namespace = "App", name = "CannonInspector")]
#[parent(crate::app::mapinspector::MapInspector)]
pub struct CannonInspector {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Z")]
    pub m_z: i32,
    #[rename(name = "m_MaxShells")]
    pub m_max_shells: i32,
    #[rename(name = "m_KeyShells")]
    pub m_key_shells: ::unity2::Il2CppString,
}

#[cfg(feature = "app-cannoninspector")]
#[::unity2::methods]
impl CannonInspector {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, x: i32, z: i32, max_shells: i32) -> ();

    #[method(name = "IsEanble", args = 0)]
    pub fn is_eanble(self) -> bool;

    #[method(name = "Completed", args = 0)]
    pub fn completed(self) -> ();

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "GetMaxShells", args = 0)]
    pub fn get_max_shells(self) -> i32;

    #[method(name = "SetShells", args = 1)]
    pub fn set_shells(self, num: i32) -> ();

    #[method(name = "GetShells", args = 0)]
    pub fn get_shells(self) -> i32;

    #[method(name = "DecreaseShell", args = 0)]
    pub fn decrease_shell(self) -> ();

    #[method(name = "GetCannonSkill", args = 1)]
    pub fn get_cannon_skill(self, is_force: bool) -> crate::app::skilldata::SkillData;

    #[method(name = "IsTerrainFlag", args = 1)]
    pub fn is_terrain_flag(self, flags: crate::app::terraindata_2::TerrainData_Flags) -> bool;

    #[method(name = "IsBowCannon", args = 0)]
    pub fn is_bow_cannon(self) -> bool;

    #[method(name = "IsMagicCannon", args = 0)]
    pub fn is_magic_cannon(self) -> bool;

    #[method(name = "IsFireCannon", args = 0)]
    pub fn is_fire_cannon(self) -> bool;

    #[method(name = "GetTerrain", args = 0)]
    pub fn get_terrain(self) -> crate::app::terraindata_2::TerrainData_2;
}

#[cfg(feature = "app-cannoninspector")]
impl CannonInspector {
    pub fn new(x: i32, z: i32, max_shells: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CannonInspector),
                ::core::stringify!(new),
            )
        });
        <Self as ICannonInspectorMethods>::ctor(this, x, z, max_shells);
        this
    }
}
