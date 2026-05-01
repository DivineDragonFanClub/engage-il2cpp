
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/user_data_registries/typedescriptorregistry/TypeDescriptorRegistry.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.UserDataRegistries",
    name = "TypeDescriptorRegistry"
)]
#[parent(crate::system::object::Object)]
pub struct TypeDescriptorRegistry {
    #[static_field]
    #[rename(name = "s_Lock")]
    pub s_lock: ::unity2::IlInstance,
    #[static_field]
    #[rename(name = "s_TypeRegistry")]
    pub s_type_registry: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::SystemType,
        crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    >,
    #[static_field]
    #[rename(name = "s_TypeRegistryHistory")]
    pub s_type_registry_history: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::SystemType,
        crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    >,
    #[static_field]
    #[rename(name = "s_DefaultAccessMode")]
    pub s_default_access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
}

#[cfg(feature = "moon_sharp-interpreter-interop-user_data_registries-typedescriptorregistry")]
#[::unity2::methods]
impl TypeDescriptorRegistry {
    #[method(name = "RegisterAssembly", args = 2)]
    pub fn register_assembly(
        asm: crate::system::reflection::assembly::Assembly,
        include_extension_types: bool,
    ) -> ();

    #[method(name = "IsTypeRegistered", args = 1)]
    pub fn is_type_registered(r#type: ::unity2::SystemType) -> bool;

    #[method(name = "UnregisterType", args = 1)]
    pub fn unregister_type(t: ::unity2::SystemType) -> ();

    #[method(name = "get_DefaultAccessMode", args = 0)]
    pub fn get_default_access_mode(
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "set_DefaultAccessMode", args = 1)]
    pub fn set_default_access_mode(
        value: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "RegisterProxyType_Impl", args = 3)]
    pub fn register_proxy_type_impl(
        proxy_factory: crate::moon_sharp::interpreter::interop::iproxyfactory::IProxyFactory,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "RegisterType_Impl", args = 4)]
    pub fn register_type_impl(
        r#type: ::unity2::SystemType,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        friendly_name: ::unity2::Il2CppString,
        descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "PerformRegistration", args = 3)]
    pub fn perform_registration(
        r#type: ::unity2::SystemType,
        new_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
        old_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "ResolveDefaultAccessModeForType", args = 2)]
    pub fn resolve_default_access_mode_for_type(
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        r#type: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "GetDescriptorForType", args = 2)]
    pub fn get_descriptor_for_type(
        r#type: ::unity2::SystemType,
        search_interfaces: bool,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "FrameworkIsAssignableFrom", args = 1)]
    pub fn framework_is_assignable_from(r#type: ::unity2::SystemType) -> bool;

    #[method(name = "IsTypeBlacklisted", args = 1)]
    pub fn is_type_blacklisted(t: ::unity2::SystemType) -> bool;

    #[method(name = "get_RegisteredTypes", args = 0)]
    pub fn get_registered_types(
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<
            ::unity2::SystemType,
            crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
        >,
    >;

    #[method(name = "get_RegisteredTypesHistory", args = 0)]
    pub fn get_registered_types_history(
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<
            ::unity2::SystemType,
            crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
        >,
    >;

    #[method(name = "get_RegistrationPolicy", args = 0)]
    pub fn get_registration_policy () -> crate :: moon_sharp :: interpreter :: interop :: registration_policies :: iregistrationpolicy :: IRegistrationPolicy ;

    #[method(name = "set_RegistrationPolicy", args = 1)]
    pub fn set_registration_policy(
        value : crate :: moon_sharp :: interpreter :: interop :: registration_policies :: iregistrationpolicy :: IRegistrationPolicy,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
