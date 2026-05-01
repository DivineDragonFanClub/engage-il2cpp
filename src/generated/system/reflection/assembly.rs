
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assembly/Assembly.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "Assembly")]
#[parent(crate::system::object::Object)]
pub struct Assembly {
    #[rename(name = "_mono_assembly")]
    pub mono_assembly: ::unity2::IntPtr,
    #[rename(name = "resolve_event_holder")]
    pub resolve_event_holder: crate::system::reflection::assembly::Assembly_ResolveEventHolder,
    #[rename(name = "_evidence")]
    pub evidence: ::unity2::IlInstance,
    #[rename(name = "_minimum")]
    pub minimum: ::unity2::IlInstance,
    #[rename(name = "_optional")]
    pub optional: ::unity2::IlInstance,
    #[rename(name = "_refuse")]
    pub refuse: ::unity2::IlInstance,
    #[rename(name = "_granted")]
    pub granted: ::unity2::IlInstance,
    #[rename(name = "_denied")]
    pub denied: ::unity2::IlInstance,
    #[rename(name = "fromByteArray")]
    pub from_byte_array: bool,
    #[rename(name = "assemblyName")]
    pub assembly_name: ::unity2::Il2CppString,
}

#[cfg(feature = "system-reflection-assembly")]
#[::unity2::methods]
impl Assembly {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_code_base", args = 1)]
    pub fn get_code_base(self, escaped: bool) -> ::unity2::Il2CppString;

    #[method(name = "get_fullname", args = 0)]
    pub fn get_fullname(self) -> ::unity2::Il2CppString;

    #[method(name = "get_location", args = 0)]
    pub fn get_location(self) -> ::unity2::Il2CppString;

    #[method(name = "GetAotId", args = 0)]
    pub fn get_aot_id() -> ::unity2::Il2CppString;

    #[method(name = "get_CodeBase", args = 0)]
    pub fn get_code_base_2(self) -> ::unity2::Il2CppString;

    #[method(name = "get_FullName", args = 0)]
    pub fn get_full_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "GetCustomAttributes", args = 2)]
    pub fn get_custom_attributes(
        self,
        attribute_type: ::unity2::SystemType,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "GetManifestResourceInternal", args = 3)]
    pub fn get_manifest_resource_internal(
        self,
        name: ::unity2::Il2CppString,
        size: i32,
        module: crate::system::reflection::module::Module,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetManifestResourceStream", args = 1)]
    pub fn get_manifest_resource_stream(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::system::io::stream::Stream;

    #[method(name = "GetTypes", args = 1)]
    pub fn get_types(self, exported_only: bool) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetTypes", args = 0)]
    pub fn get_types_2(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetType", args = 2)]
    pub fn get_type(
        self,
        name: ::unity2::Il2CppString,
        throw_on_error: bool,
    ) -> ::unity2::SystemType;

    #[method(name = "GetType", args = 1)]
    pub fn get_type_2(self, name: ::unity2::Il2CppString) -> ::unity2::SystemType;

    #[method(name = "InternalGetType", args = 4)]
    pub fn internal_get_type(
        self,
        module: crate::system::reflection::module::Module,
        name: ::unity2::Il2CppString,
        throw_on_error: bool,
        ignore_case: bool,
    ) -> ::unity2::SystemType;

    #[method(name = "GetName", args = 1)]
    pub fn get_name(
        self,
        copied_name: bool,
    ) -> crate::system::reflection::assemblyname::AssemblyName;

    #[method(name = "GetName", args = 0)]
    pub fn get_name_2(self) -> crate::system::reflection::assemblyname::AssemblyName;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetAssembly", args = 1)]
    pub fn get_assembly(
        r#type: ::unity2::SystemType,
    ) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "Load", args = 1)]
    pub fn load(
        assembly_string: ::unity2::Il2CppString,
    ) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "GetModulesInternal", args = 0)]
    pub fn get_modules_internal(self)
        -> ::unity2::Array<crate::system::reflection::module::Module>;

    #[method(name = "GetManifestResourceNames", args = 0)]
    pub fn get_manifest_resource_names(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetExecutingAssembly", args = 0)]
    pub fn get_executing_assembly() -> crate::system::reflection::assembly::Assembly;

    #[method(name = "GetCallingAssembly", args = 0)]
    pub fn get_calling_assembly() -> crate::system::reflection::assembly::Assembly;

    #[method(name = "GetManifestResourceInfoInternal", args = 2)]
    pub fn get_manifest_resource_info_internal(
        self,
        name: ::unity2::Il2CppString,
        info: crate::system::reflection::manifestresourceinfo::ManifestResourceInfo,
    ) -> bool;

    #[method(name = "GetManifestResourceInfo", args = 1)]
    pub fn get_manifest_resource_info(
        self,
        resource_name: ::unity2::Il2CppString,
    ) -> crate::system::reflection::manifestresourceinfo::ManifestResourceInfo;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, o: crate::system::object::Object) -> bool;

    #[method(name = "get_IsFullyTrusted", args = 0)]
    pub fn get_is_fully_trusted(self) -> bool;

    #[method(name = "GetType", args = 3)]
    pub fn get_type_3(
        self,
        name: ::unity2::Il2CppString,
        throw_on_error: bool,
        ignore_case: bool,
    ) -> ::unity2::SystemType;

    #[method(name = "GetModule", args = 1)]
    pub fn get_module(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::system::reflection::module::Module;

    #[method(name = "GetModules", args = 1)]
    pub fn get_modules(
        self,
        get_resource_modules: bool,
    ) -> ::unity2::Array<crate::system::reflection::module::Module>;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::assembly::Assembly,
        right: crate::system::reflection::assembly::Assembly,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::reflection::assembly::Assembly,
        right: crate::system::reflection::assembly::Assembly,
    ) -> bool;
}

#[cfg(feature = "system-reflection-assembly")]
impl Assembly {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Assembly),
                ::core::stringify!(new),
            )
        });
        <Self as IAssemblyMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assembly/Assembly_UnmanagedMemoryStreamForModule.md")))]
#[::unity2::class(
    namespace = "System.Reflection",
    name = "Assembly.UnmanagedMemoryStreamForModule"
)]
pub struct Assembly_UnmanagedMemoryStreamForModule {
    #[rename(name = "module")]
    pub module: crate::system::reflection::module::Module,
}

#[cfg(feature = "system-reflection-assembly")]
#[::unity2::methods]
impl Assembly_UnmanagedMemoryStreamForModule {
    #[method(name = "Dispose", args = 1)]
    pub fn dispose(self, disposing: bool) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assembly/Assembly_ResolveEventHolder.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "Assembly.ResolveEventHolder")]
#[parent(crate::system::object::Object)]
pub struct Assembly_ResolveEventHolder {}

#[cfg(feature = "system-reflection-assembly")]
#[::unity2::methods]
impl Assembly_ResolveEventHolder {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-assembly")]
impl Assembly_ResolveEventHolder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Assembly_ResolveEventHolder),
                ::core::stringify!(new),
            )
        });
        <Self as IAssembly_ResolveEventHolderMethods>::ctor(this);
        this
    }
}
