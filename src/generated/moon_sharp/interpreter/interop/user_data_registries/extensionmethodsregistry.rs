
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/user_data_registries/extensionmethodsregistry/ExtensionMethodsRegistry.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.UserDataRegistries",
    name = "ExtensionMethodsRegistry"
)]
#[parent(crate::system::object::Object)]
pub struct ExtensionMethodsRegistry {
# [static_field] # [rename (name = "s_Lock")] pub s_lock : :: unity2 :: IlInstance ,
# [static_field] # [rename (name = "s_Registry")] pub s_registry : crate :: moon_sharp :: interpreter :: data_structs :: multidictionary_2 :: MultiDictionary_2 < :: unity2 :: Il2CppString , crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor > ,
# [static_field] # [rename (name = "s_UnresolvedGenericsRegistry")] pub s_unresolved_generics_registry : crate :: moon_sharp :: interpreter :: data_structs :: multidictionary_2 :: MultiDictionary_2 < :: unity2 :: Il2CppString , crate :: moon_sharp :: interpreter :: interop :: user_data_registries :: extensionmethodsregistry :: ExtensionMethodsRegistry_UnresolvedGenericMethod > ,
# [static_field] # [rename (name = "s_ExtensionMethodChangeVersion")] pub s_extension_method_change_version : i32 ,
}

#[cfg(feature = "moon_sharp-interpreter-interop-user_data_registries-extensionmethodsregistry")]
#[::unity2::methods]
impl ExtensionMethodsRegistry {
    #[method(name = "RegisterExtensionType", args = 2)]
    pub fn register_extension_type(
        r#type: ::unity2::SystemType,
        mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "FrameworkGetMethods", args = 0)]
    pub fn framework_get_methods() -> crate::system::object::Object;

    #[method(name = "GetExtensionMethodsByName", args = 1)]
    pub fn get_extension_methods_by_name (name : :: unity2 :: Il2CppString) -> crate :: system :: collections :: generic :: ienumerable_1 :: IEnumerable_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor > ;

    #[method(name = "GetExtensionMethodsChangeVersion", args = 0)]
    pub fn get_extension_methods_change_version() -> i32;

    #[method(name = "GetExtensionMethodsByNameAndType", args = 2)]
    pub fn get_extension_methods_by_name_and_type (name : :: unity2 :: Il2CppString , extended_type : :: unity2 :: SystemType) -> crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor > ;

    #[method(name = "InstantiateMethodInfo", args = 4)]
    pub fn instantiate_method_info(
        mi: crate::system::reflection::methodinfo::MethodInfo,
        extension_type: ::unity2::SystemType,
        generic_type: ::unity2::SystemType,
        extended_type: ::unity2::SystemType,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetGenericMatch", args = 2)]
    pub fn get_generic_match(
        extension_type: ::unity2::SystemType,
        extended_type: ::unity2::SystemType,
    ) -> ::unity2::SystemType;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-user_data_registries-extensionmethodsregistry")]
impl ExtensionMethodsRegistry {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExtensionMethodsRegistry),
                ::core::stringify!(new),
            )
        });
        <Self as IExtensionMethodsRegistryMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/user_data_registries/extensionmethodsregistry/ExtensionMethodsRegistry_UnresolvedGenericMethod.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.UserDataRegistries",
    name = "ExtensionMethodsRegistry.UnresolvedGenericMethod"
)]
#[parent(crate::system::object::Object)]
pub struct ExtensionMethodsRegistry_UnresolvedGenericMethod {
    #[rename(name = "Method")]
    pub method: crate::system::reflection::methodinfo::MethodInfo,
    #[rename(name = "AccessMode")]
    pub access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    #[rename(name = "AlreadyAddedTypes")]
    pub already_added_types:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::SystemType>,
}

#[cfg(feature = "moon_sharp-interpreter-interop-user_data_registries-extensionmethodsregistry")]
#[::unity2::methods]
impl ExtensionMethodsRegistry_UnresolvedGenericMethod {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        mi: crate::system::reflection::methodinfo::MethodInfo,
        mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-user_data_registries-extensionmethodsregistry")]
impl ExtensionMethodsRegistry_UnresolvedGenericMethod {
    pub fn new(
        mi: crate::system::reflection::methodinfo::MethodInfo,
        mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExtensionMethodsRegistry_UnresolvedGenericMethod),
                ::core::stringify!(new),
            )
        });
        <Self as IExtensionMethodsRegistry_UnresolvedGenericMethodMethods>::ctor(this, mi, mode);
        this
    }
}
