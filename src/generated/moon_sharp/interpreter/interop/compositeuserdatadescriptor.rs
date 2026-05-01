
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/compositeuserdatadescriptor/CompositeUserDataDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "CompositeUserDataDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct CompositeUserDataDescriptor {
    #[rename(name = "m_Descriptors")]
    pub m_descriptors: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    >,
    #[rename(name = "m_Type")]
    pub m_type: ::unity2::SystemType,
}

#[cfg(feature = "moon_sharp-interpreter-interop-compositeuserdatadescriptor")]
#[::unity2::methods]
impl CompositeUserDataDescriptor {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        descriptors: crate::system::collections::generic::list_1::List_1<
            crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
        >,
        r#type: ::unity2::SystemType,
    ) -> ();

    #[method(name = "get_Descriptors", args = 0)]
    pub fn get_descriptors(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    >;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> ::unity2::SystemType;

    #[method(name = "Index", args = 4)]
    pub fn index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_name_index: bool,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetIndex", args = 5)]
    pub fn set_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_name_index: bool,
    ) -> bool;

    #[method(name = "AsString", args = 1)]
    pub fn as_string(self, obj: crate::system::object::Object) -> ::unity2::Il2CppString;

    #[method(name = "MetaIndex", args = 3)]
    pub fn meta_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        metaname: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "IsTypeCompatible", args = 2)]
    pub fn is_type_compatible(
        self,
        r#type: ::unity2::SystemType,
        obj: crate::system::object::Object,
    ) -> bool;
}

#[cfg(feature = "moon_sharp-interpreter-interop-compositeuserdatadescriptor")]
impl CompositeUserDataDescriptor {
    pub fn new(
        descriptors: crate::system::collections::generic::list_1::List_1<
            crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
        >,
        r#type: ::unity2::SystemType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CompositeUserDataDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as ICompositeUserDataDescriptorMethods>::ctor(this, descriptors, r#type);
        this
    }
}
