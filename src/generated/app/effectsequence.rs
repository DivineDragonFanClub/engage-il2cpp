
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/effectsequence/EffectSequence_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EffectSequence_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EffectSequence_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EffectSequence.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EffectSequence_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EffectSequence_Kind {
    pub fn active() -> Self {
        Self { value: 0 }
    }

    pub fn shoot() -> Self {
        Self { value: 1 }
    }

    pub fn hit() -> Self {
        Self { value: 2 }
    }

    pub fn num() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/effectsequence/EffectSequence.md")))]
#[::unity2::class(namespace = "App", name = "EffectSequence")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: effectsequence :: EffectSequence >)]
pub struct EffectSequence {
    #[rename(name = "m_Effects")]
    pub m_effects: ::unity2::Array<crate::app::effectdata::EffectData>,
}

#[cfg(feature = "app-effectsequence")]
#[::unity2::methods]
impl EffectSequence {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Sequence", args = 0)]
    pub fn get_sequence(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Sequence", args = 1)]
    pub fn set_sequence(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Active", args = 0)]
    pub fn get_active(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Active", args = 1)]
    pub fn set_active(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Shoot", args = 0)]
    pub fn get_shoot(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Shoot", args = 1)]
    pub fn set_shoot(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Hit", args = 0)]
    pub fn get_hit(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Hit", args = 1)]
    pub fn set_hit(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetData", args = 1)]
    pub fn get_data(
        self,
        kind: crate::app::effectsequence::EffectSequence_Kind,
    ) -> crate::app::effectdata::EffectData;

    #[method(name = "PlaySequence", args = 3)]
    pub fn play_sequence(
        self,
        mode: crate::app::effectdata::EffectData_Modes,
        parent: crate::unity_engine::gameobject::GameObject,
        position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "PlaySequence", args = 4)]
    pub fn play_sequence_2(
        self,
        mode: crate::app::effectdata::EffectData_Modes,
        parent: crate::unity_engine::gameobject::GameObject,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "PlayShoot", args = 2)]
    pub fn play_shoot(
        self,
        mode: crate::app::effectdata::EffectData_Modes,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "PlayImpl", args = 4)]
    pub fn play_impl(
        self,
        kind: crate::app::effectsequence::EffectSequence_Kind,
        mode: crate::app::effectdata::EffectData_Modes,
        parent: crate::unity_engine::gameobject::GameObject,
        delay_time: f32,
    ) -> crate::app::resourceobject::ResourceObject;

    #[method(name = "TryGetData", args = 1)]
    pub fn try_get_data(self, name: ::unity2::Il2CppString) -> crate::app::effectdata::EffectData;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-effectsequence")]
impl EffectSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EffectSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IEffectSequenceMethods>::ctor(this);
        this
    }
}
