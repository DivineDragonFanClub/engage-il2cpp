
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/iuserdatamemberdescriptor/IUserDataMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "IUserDataMemberDescriptor"
)]
pub struct IUserDataMemberDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-iuserdatamemberdescriptor")]
#[::unity2::methods]
impl IUserDataMemberDescriptor {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> ::unity2::SystemType;

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
    ) -> bool;

    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(
        self,
    ) -> crate::moon_sharp::interpreter::interop::userdatamembertype::UserDataMemberType;

    #[method(name = "Optimize", args = 0)]
    pub fn optimize(self) -> ();

    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;
}
