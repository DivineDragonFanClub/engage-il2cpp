
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/fieldmemberdescriptor/FieldMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "FieldMemberDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct FieldMemberDescriptor {
    #[rename(name = "m_ConstValue")]
    pub m_const_value: ::unity2::IlInstance,
    #[rename(name = "m_OptimizedGetter")]
    pub m_optimized_getter:
        crate::system::func_2::Func_2<crate::system::object::Object, crate::system::object::Object>,
}

#[cfg(feature = "moon_sharp-interpreter-interop-fieldmemberdescriptor")]
#[::unity2::methods]
impl FieldMemberDescriptor {
    #[method(name = "get_FieldInfo", args = 0)]
    pub fn get_field_info(self) -> crate::system::reflection::fieldinfo::FieldInfo;

    #[method(name = "set_FieldInfo", args = 1)]
    pub fn set_field_info(self, value: crate::system::reflection::fieldinfo::FieldInfo) -> ();

    #[method(name = "get_AccessMode", args = 0)]
    pub fn get_access_mode(
        self,
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "set_AccessMode", args = 1)]
    pub fn set_access_mode(
        self,
        value: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "set_IsStatic", args = 1)]
    pub fn set_is_static(self, value: bool) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsConst", args = 0)]
    pub fn get_is_const(self) -> bool;

    #[method(name = "set_IsConst", args = 1)]
    pub fn set_is_const(self, value: bool) -> ();

    #[method(name = "get_IsReadonly", args = 0)]
    pub fn get_is_readonly(self) -> bool;

    #[method(name = "set_IsReadonly", args = 1)]
    pub fn set_is_readonly(self, value: bool) -> ();

    #[method(name = "TryCreateIfVisible", args = 2)]
    pub fn try_create_if_visible(
        fi: crate::system::reflection::fieldinfo::FieldInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> crate::moon_sharp::interpreter::interop::fieldmemberdescriptor::FieldMemberDescriptor;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        fi: crate::system::reflection::fieldinfo::FieldInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "OptimizeGetter", args = 0)]
    pub fn optimize_getter(self) -> ();

    #[method(name = "SetValue", args = 3)]
    pub fn set_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        v: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "get_MemberAccess", args = 0)]
    pub fn get_member_access (self ,) -> crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess ;

    #[method(
        name = "MoonSharp.Interpreter.Interop.BasicDescriptors.IOptimizableDescriptor.Optimize",
        args = 0
    )]
    pub fn moon_sharp_interpreter_interop_basic_descriptors_i_optimizable_descriptor_optimize(
        self,
    ) -> ();

    #[method(name = "PrepareForWiring", args = 1)]
    pub fn prepare_for_wiring(self, t: crate::moon_sharp::interpreter::table::Table) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-fieldmemberdescriptor")]
impl FieldMemberDescriptor {
    pub fn new(
        fi: crate::system::reflection::fieldinfo::FieldInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FieldMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IFieldMemberDescriptorMethods>::ctor(this, fi, access_mode);
        this
    }
}
