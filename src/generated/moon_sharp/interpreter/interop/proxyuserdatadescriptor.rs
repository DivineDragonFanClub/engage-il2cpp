
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/proxyuserdatadescriptor/ProxyUserDataDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "ProxyUserDataDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct ProxyUserDataDescriptor {
    #[rename(name = "m_ProxyDescriptor")]
    pub m_proxy_descriptor:
        crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    #[rename(name = "m_ProxyFactory")]
    pub m_proxy_factory: crate::moon_sharp::interpreter::interop::iproxyfactory::IProxyFactory,
}

#[cfg(feature = "moon_sharp-interpreter-interop-proxyuserdatadescriptor")]
#[::unity2::methods]
impl ProxyUserDataDescriptor {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        proxy_factory: crate::moon_sharp::interpreter::interop::iproxyfactory::IProxyFactory,
        proxy_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
        friendly_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "get_InnerDescriptor", args = 0)]
    pub fn get_inner_descriptor(
        self,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> ::unity2::SystemType;

    #[method(name = "Proxy", args = 1)]
    pub fn proxy(self, obj: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "Index", args = 4)]
    pub fn index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetIndex", args = 5)]
    pub fn set_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
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

#[cfg(feature = "moon_sharp-interpreter-interop-proxyuserdatadescriptor")]
impl ProxyUserDataDescriptor {
    pub fn new(
        proxy_factory: crate::moon_sharp::interpreter::interop::iproxyfactory::IProxyFactory,
        proxy_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
        friendly_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProxyUserDataDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IProxyUserDataDescriptorMethods>::ctor(
            this,
            proxy_factory,
            proxy_descriptor,
            friendly_name,
        );
        this
    }
}
