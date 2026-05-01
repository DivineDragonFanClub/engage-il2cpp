
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/profilingsample/ProfilingSample.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ProfilingSample {
    pub m_cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    pub m_name: ::unity2::Il2CppString,
    pub m_disposed: bool,
    pub m_sampler: crate::unity_engine::profiling::customsampler::CustomSampler,
}

impl ::unity2::ClassIdentity for ProfilingSample {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ProfilingSample";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfilingSample {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-profilingsample")]
#[::unity2::methods(value)]
impl ProfilingSample {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        name: ::unity2::Il2CppString,
        sampler: crate::unity_engine::profiling::customsampler::CustomSampler,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        format: ::unity2::Il2CppString,
        arg: crate::system::object::Object,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();
}
