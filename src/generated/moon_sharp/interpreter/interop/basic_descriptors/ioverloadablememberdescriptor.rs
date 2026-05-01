
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/basic_descriptors/ioverloadablememberdescriptor/IOverloadableMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.BasicDescriptors",
    name = "IOverloadableMemberDescriptor"
)]
pub struct IOverloadableMemberDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-basic_descriptors-ioverloadablememberdescriptor")]
#[::unity2::methods]
impl IOverloadableMemberDescriptor {
    #[method(name = "Execute", args = 4)]
    pub fn execute(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "get_ExtensionMethodType", args = 0)]
    pub fn get_extension_method_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Parameters", args = 0)]
    pub fn get_parameters (self ,) -> :: unity2 :: Array < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: parameterdescriptor :: ParameterDescriptor > ;

    #[method(name = "get_VarArgsArrayType", args = 0)]
    pub fn get_var_args_array_type(self) -> ::unity2::SystemType;

    #[method(name = "get_VarArgsElementType", args = 0)]
    pub fn get_var_args_element_type(self) -> ::unity2::SystemType;

    #[method(name = "get_SortDiscriminant", args = 0)]
    pub fn get_sort_discriminant(self) -> ::unity2::Il2CppString;
}
