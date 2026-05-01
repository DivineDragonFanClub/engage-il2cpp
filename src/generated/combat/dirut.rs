
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/dirut/DirUt.md")))]
#[::unity2::class(namespace = "Combat", name = "DirUt")]
#[parent(crate::system::object::Object)]
pub struct DirUt {}

#[cfg(feature = "combat-dirut")]
#[::unity2::methods]
impl DirUt {
    #[method(name = "VecToDir2", args = 3)]
    pub fn vec_to_dir2(
        slash_dir: crate::unity_engine::vector3::Vector3,
        out_dir1: crate::combat::dir::Dir,
        out_dir2: crate::combat::dir::Dir,
    ) -> ();

    #[method(name = "VecToDir", args = 1)]
    pub fn vec_to_dir(v: crate::unity_engine::vector3::Vector3) -> crate::combat::dir::Dir;

    #[method(name = "ToJapanese", args = 1)]
    pub fn to_japanese(v: crate::unity_engine::vector3::Vector3) -> ::unity2::Il2CppString;
}
