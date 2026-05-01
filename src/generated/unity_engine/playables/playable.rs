
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/playable/Playable.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Playable {
    pub m_handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
}

impl ::unity2::ClassIdentity for Playable {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "Playable";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Playable {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-playables-playable")]
#[::unity2::methods(value)]
impl Playable {
    #[method(name = "get_Null", args = 0)]
    pub fn get_null() -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "Create", args = 2)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        input_count: i32,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, handle: crate::unity_engine::playables::playablehandle::PlayableHandle)
        -> ();

    #[method(name = "GetHandle", args = 0)]
    pub fn get_handle(self) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "GetPlayableType", args = 0)]
    pub fn get_playable_type(self) -> ::unity2::SystemType;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::unity_engine::playables::playable::Playable) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
