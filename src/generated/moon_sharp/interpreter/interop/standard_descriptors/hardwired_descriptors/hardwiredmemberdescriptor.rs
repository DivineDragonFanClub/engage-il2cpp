
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/standard_descriptors/hardwired_descriptors/hardwiredmemberdescriptor/HardwiredMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.StandardDescriptors.HardwiredDescriptors",
    name = "HardwiredMemberDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct HardwiredMemberDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-hardwired_descriptors-hardwiredmemberdescriptor")]
#[::unity2::methods]
impl HardwiredMemberDescriptor {
    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(self) -> ::unity2::SystemType;

    #[method(name = "set_MemberType", args = 1)]
    pub fn set_member_type(self, value: ::unity2::SystemType) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        member_type: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
        is_static: bool,
        access : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess,
    ) -> ();

    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "set_IsStatic", args = 1)]
    pub fn set_is_static(self, value: bool) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MemberAccess", args = 0)]
    pub fn get_member_access (self ,) -> crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess ;

    #[method(name = "set_MemberAccess", args = 1)]
    pub fn set_member_access(
        self,
        value : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess,
    ) -> ();

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "GetValueImpl", args = 2)]
    pub fn get_value_impl(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::system::object::Object;

    #[method(name = "SetValueImpl", args = 3)]
    pub fn set_value_impl(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-hardwired_descriptors-hardwiredmemberdescriptor")]
impl HardwiredMemberDescriptor {
    pub fn new(
        member_type: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
        is_static: bool,
        access : crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HardwiredMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IHardwiredMemberDescriptorMethods>::ctor(
            this,
            member_type,
            name,
            is_static,
            access,
        );
        this
    }
}
