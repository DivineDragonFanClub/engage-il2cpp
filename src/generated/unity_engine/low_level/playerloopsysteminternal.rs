
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/low_level/playerloopsysteminternal/PlayerLoopSystemInternal.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct PlayerLoopSystemInternal {
    pub r#type: ::unity2::SystemType,
    pub update_delegate:
        crate::unity_engine::low_level::playerloopsystem::PlayerLoopSystem_UpdateFunction,
    pub update_function: ::unity2::IntPtr,
    pub loop_condition_function: ::unity2::IntPtr,
    pub num_sub_systems: i32,
}

impl ::unity2::ClassIdentity for PlayerLoopSystemInternal {
    const NAMESPACE: &'static str = "UnityEngine.LowLevel";

    const NAME: &'static str = "PlayerLoopSystemInternal";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PlayerLoopSystemInternal {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
