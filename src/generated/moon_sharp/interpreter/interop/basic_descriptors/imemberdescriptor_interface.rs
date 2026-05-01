
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/basic_descriptors/imemberdescriptor_interface/IMemberDescriptor_Interface.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.BasicDescriptors",
    name = "IMemberDescriptor"
)]
pub struct IMemberDescriptor_Interface {}

#[cfg(feature = "moon_sharp-interpreter-interop-basic_descriptors-imemberdescriptor_interface")]
#[::unity2::methods]
impl IMemberDescriptor_Interface {
    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_MemberAccess", args = 0)]
    pub fn get_member_access (self ,) -> crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess ;

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
}
