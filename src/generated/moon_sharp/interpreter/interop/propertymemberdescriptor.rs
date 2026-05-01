
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/propertymemberdescriptor/PropertyMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "PropertyMemberDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct PropertyMemberDescriptor {
    #[rename(name = "m_Getter")]
    pub m_getter: crate::system::reflection::methodinfo::MethodInfo,
    #[rename(name = "m_Setter")]
    pub m_setter: crate::system::reflection::methodinfo::MethodInfo,
    #[rename(name = "m_OptimizedGetter")]
    pub m_optimized_getter:
        crate::system::func_2::Func_2<crate::system::object::Object, crate::system::object::Object>,
    #[rename(name = "m_OptimizedSetter")]
    pub m_optimized_setter: crate::system::action_2::Action_2<
        crate::system::object::Object,
        crate::system::object::Object,
    >,
}

#[cfg(feature = "moon_sharp-interpreter-interop-propertymemberdescriptor")]
#[::unity2::methods]
impl PropertyMemberDescriptor {
    #[method(name = "get_PropertyInfo", args = 0)]
    pub fn get_property_info(self) -> crate::system::reflection::propertyinfo::PropertyInfo;

    #[method(name = "set_PropertyInfo", args = 1)]
    pub fn set_property_info(
        self,
        value: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> ();

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

    #[method(name = "get_CanRead", args = 0)]
    pub fn get_can_read(self) -> bool;

    #[method(name = "get_CanWrite", args = 0)]
    pub fn get_can_write(self) -> bool;

    #[method(name = "TryCreateIfVisible", args = 2)]
    pub fn try_create_if_visible(
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> crate::moon_sharp::interpreter::interop::propertymemberdescriptor::PropertyMemberDescriptor;

    #[method(name = "TryCreate", args = 4)]
    pub fn try_create(
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        getter: crate::system::reflection::methodinfo::MethodInfo,
        setter: crate::system::reflection::methodinfo::MethodInfo,
    ) -> crate::moon_sharp::interpreter::interop::propertymemberdescriptor::PropertyMemberDescriptor;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_2(
        self,
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        getter: crate::system::reflection::methodinfo::MethodInfo,
        setter: crate::system::reflection::methodinfo::MethodInfo,
    ) -> ();

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "OptimizeGetter", args = 0)]
    pub fn optimize_getter(self) -> ();

    #[method(name = "OptimizeSetter", args = 0)]
    pub fn optimize_setter(self) -> ();

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

#[cfg(feature = "moon_sharp-interpreter-interop-propertymemberdescriptor")]
impl PropertyMemberDescriptor {
    pub fn new(
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PropertyMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IPropertyMemberDescriptorMethods>::ctor(this, pi, access_mode);
        this
    }

    pub fn new_2(
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        getter: crate::system::reflection::methodinfo::MethodInfo,
        setter: crate::system::reflection::methodinfo::MethodInfo,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PropertyMemberDescriptor),
                ::core::stringify!(new_2),
            )
        });
        <Self as IPropertyMemberDescriptorMethods>::ctor_2(this, pi, access_mode, getter, setter);
        this
    }
}
