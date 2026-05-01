
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/scriptplayableoutput/ScriptPlayableOutput.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ScriptPlayableOutput {
    pub m_handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
}

impl ::unity2::ClassIdentity for ScriptPlayableOutput {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "ScriptPlayableOutput";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ScriptPlayableOutput {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-playables-scriptplayableoutput")]
#[::unity2::methods(value)]
impl ScriptPlayableOutput {
    #[method(name = "Create", args = 2)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::playables::scriptplayableoutput::ScriptPlayableOutput;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> ();

    #[method(name = "get_Null", args = 0)]
    pub fn get_null() -> crate::unity_engine::playables::scriptplayableoutput::ScriptPlayableOutput;

    #[method(name = "GetHandle", args = 0)]
    pub fn get_handle(
        self,
    ) -> crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        output: crate::unity_engine::playables::scriptplayableoutput::ScriptPlayableOutput,
    ) -> crate::unity_engine::playables::playableoutput::PlayableOutput;
}
