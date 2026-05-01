
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/sphericalharmonicsl2/SphericalHarmonicsL2.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SphericalHarmonicsL2 {
    pub shr0: f32,
    pub shr1: f32,
    pub shr2: f32,
    pub shr3: f32,
    pub shr4: f32,
    pub shr5: f32,
    pub shr6: f32,
    pub shr7: f32,
    pub shr8: f32,
    pub shg0: f32,
    pub shg1: f32,
    pub shg2: f32,
    pub shg3: f32,
    pub shg4: f32,
    pub shg5: f32,
    pub shg6: f32,
    pub shg7: f32,
    pub shg8: f32,
    pub shb0: f32,
    pub shb1: f32,
    pub shb2: f32,
    pub shb3: f32,
    pub shb4: f32,
    pub shb5: f32,
    pub shb6: f32,
    pub shb7: f32,
    pub shb8: f32,
}

impl ::unity2::ClassIdentity for SphericalHarmonicsL2 {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "SphericalHarmonicsL2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SphericalHarmonicsL2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-sphericalharmonicsl2")]
#[::unity2::methods(value)]
impl SphericalHarmonicsL2 {
    #[method(name = "get_Item", args = 2)]
    pub fn get_item(self, rgb: i32, coefficient: i32) -> f32;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(
        self,
        other: crate::unity_engine::rendering::sphericalharmonicsl2::SphericalHarmonicsL2,
    ) -> bool;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::rendering::sphericalharmonicsl2::SphericalHarmonicsL2,
        rhs: crate::unity_engine::rendering::sphericalharmonicsl2::SphericalHarmonicsL2,
    ) -> bool;
}
