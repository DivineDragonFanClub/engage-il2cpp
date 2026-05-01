
use crate::moon_sharp::interpreter::refidobject::IRefIdObject;
use crate::moon_sharp::interpreter::refidobject::RefIdObject;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/userdata/UserData.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "UserData")]
#[parent(crate::moon_sharp::interpreter::refidobject::RefIdObject)]
pub struct UserData {}

#[cfg(feature = "moon_sharp-interpreter-userdata")]
#[::unity2::methods]
impl UserData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_UserValue", args = 0)]
    pub fn get_user_value(self) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "set_UserValue", args = 1)]
    pub fn set_user_value(self, value: crate::moon_sharp::interpreter::dynvalue::DynValue) -> ();

    #[method(name = "get_Object", args = 0)]
    pub fn get_object(self) -> crate::system::object::Object;

    #[method(name = "set_Object", args = 1)]
    pub fn set_object(self, value: crate::system::object::Object) -> ();

    #[method(name = "get_Descriptor", args = 0)]
    pub fn get_descriptor(
        self,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "set_Descriptor", args = 1)]
    pub fn set_descriptor(
        self,
        value: crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "RegisterType", args = 3)]
    pub fn register_type(
        r#type: ::unity2::SystemType,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "RegisterProxyType", args = 3)]
    pub fn register_proxy_type(
        proxy_factory: crate::moon_sharp::interpreter::interop::iproxyfactory::IProxyFactory,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        friendly_name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "RegisterType", args = 2)]
    pub fn register_type_2(
        r#type: ::unity2::SystemType,
        custom_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "RegisterType", args = 1)]
    pub fn register_type_3(
        custom_descriptor : crate :: moon_sharp :: interpreter :: interop :: iuserdatadescriptor :: IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "RegisterAssembly", args = 2)]
    pub fn register_assembly(
        asm: crate::system::reflection::assembly::Assembly,
        include_extension_types: bool,
    ) -> ();

    #[method(name = "IsTypeRegistered", args = 1)]
    pub fn is_type_registered(t: ::unity2::SystemType) -> bool;

    #[method(name = "UnregisterType", args = 1)]
    pub fn unregister_type(t: ::unity2::SystemType) -> ();

    #[method(name = "Create", args = 2)]
    pub fn create(
        o: crate::system::object::Object,
        descr: crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Create", args = 1)]
    pub fn create_2(
        o: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CreateStatic", args = 1)]
    pub fn create_static(
        descr: crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CreateStatic", args = 1)]
    pub fn create_static_2(
        t: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "get_RegistrationPolicy", args = 0)]
    pub fn get_registration_policy () -> crate :: moon_sharp :: interpreter :: interop :: registration_policies :: iregistrationpolicy :: IRegistrationPolicy ;

    #[method(name = "set_RegistrationPolicy", args = 1)]
    pub fn set_registration_policy(
        value : crate :: moon_sharp :: interpreter :: interop :: registration_policies :: iregistrationpolicy :: IRegistrationPolicy,
    ) -> ();

    #[method(name = "get_DefaultAccessMode", args = 0)]
    pub fn get_default_access_mode(
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "set_DefaultAccessMode", args = 1)]
    pub fn set_default_access_mode(
        value: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "RegisterExtensionType", args = 2)]
    pub fn register_extension_type(
        r#type: ::unity2::SystemType,
        mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "GetExtensionMethodsByNameAndType", args = 2)]
    pub fn get_extension_methods_by_name_and_type (name : :: unity2 :: Il2CppString , extended_type : :: unity2 :: SystemType) -> crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: ioverloadablememberdescriptor :: IOverloadableMemberDescriptor > ;

    #[method(name = "GetExtensionMethodsChangeVersion", args = 0)]
    pub fn get_extension_methods_change_version() -> i32;

    #[method(name = "GetDescriptorForType", args = 2)]
    pub fn get_descriptor_for_type(
        r#type: ::unity2::SystemType,
        search_interfaces: bool,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "GetDescriptorForObject", args = 1)]
    pub fn get_descriptor_for_object(
        o: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;

    #[method(name = "GetDescriptionOfRegisteredTypes", args = 1)]
    pub fn get_description_of_registered_types(
        use_historical_data: bool,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "GetRegisteredTypes", args = 1)]
    pub fn get_registered_types(
        use_historical_data: bool,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::SystemType>;
}

#[cfg(feature = "moon_sharp-interpreter-userdata")]
impl UserData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UserData),
                ::core::stringify!(new),
            )
        });
        <Self as IUserDataMethods>::ctor(this);
        this
    }
}
