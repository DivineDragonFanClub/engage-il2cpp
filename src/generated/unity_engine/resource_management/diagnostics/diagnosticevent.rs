
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/diagnostics/diagnosticevent/DiagnosticEvent.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DiagnosticEvent {
    pub m_graph: ::unity2::Il2CppString,
    pub m_dependencies: ::unity2::Array<i32>,
    pub m_object_id: i32,
    pub m_display_name: ::unity2::Il2CppString,
    pub m_stream: i32,
    pub m_frame: i32,
    pub m_value: i32,
}

impl ::unity2::ClassIdentity for DiagnosticEvent {
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Diagnostics";

    const NAME: &'static str = "DiagnosticEvent";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DiagnosticEvent {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-resource_management-diagnostics-diagnosticevent")]
#[::unity2::methods(value)]
impl DiagnosticEvent {
    #[method(name = "get_Graph", args = 0)]
    pub fn get_graph(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ObjectId", args = 0)]
    pub fn get_object_id(self) -> i32;

    #[method(name = "get_DisplayName", args = 0)]
    pub fn get_display_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Dependencies", args = 0)]
    pub fn get_dependencies(self) -> ::unity2::Array<i32>;

    #[method(name = "get_Stream", args = 0)]
    pub fn get_stream(self) -> i32;

    #[method(name = "get_Frame", args = 0)]
    pub fn get_frame(self) -> i32;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        graph: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        id: i32,
        stream: i32,
        frame: i32,
        value: i32,
        deps: ::unity2::Array<i32>,
    ) -> ();

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ::unity2::Array<u8>;

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(
        data: ::unity2::Array<u8>,
    ) -> crate::unity_engine::resource_management::diagnostics::diagnosticevent::DiagnosticEvent;
}
