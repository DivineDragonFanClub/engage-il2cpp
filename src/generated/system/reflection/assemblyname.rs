
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assemblyname/AssemblyName.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "AssemblyName")]
#[parent(crate::system::object::Object)]
pub struct AssemblyName {
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "codebase")]
    pub codebase: ::unity2::Il2CppString,
    #[rename(name = "major")]
    pub major: i32,
    #[rename(name = "minor")]
    pub minor: i32,
    #[rename(name = "build")]
    pub build: i32,
    #[rename(name = "revision")]
    pub revision: i32,
    #[rename(name = "flags")]
    pub flags: crate::system::reflection::assemblynameflags::AssemblyNameFlags,
    #[rename(name = "keypair")]
    pub keypair: crate::system::reflection::strongnamekeypair::StrongNameKeyPair,
    #[rename(name = "publicKey")]
    pub public_key: ::unity2::Array<u8>,
    #[rename(name = "keyToken")]
    pub key_token: ::unity2::Array<u8>,
    #[rename(name = "processor_architecture")]
    pub processor_architecture:
        crate::system::reflection::processorarchitecture::ProcessorArchitecture,
    #[rename(name = "contentType")]
    pub content_type: crate::system::reflection::assemblycontenttype::AssemblyContentType,
}

#[cfg(feature = "system-reflection-assemblyname")]
#[::unity2::methods]
impl AssemblyName {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, assembly_name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Flags", args = 0)]
    pub fn get_flags(self) -> crate::system::reflection::assemblynameflags::AssemblyNameFlags;

    #[method(name = "get_FullName", args = 0)]
    pub fn get_full_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPublicKeyToken", args = 0)]
    pub fn get_public_key_token(self) -> ::unity2::Array<u8>;

    #[method(name = "get_IsPublicKeyValid", args = 0)]
    pub fn get_is_public_key_valid(self) -> bool;

    #[method(name = "InternalGetPublicKeyToken", args = 0)]
    pub fn internal_get_public_key_token(self) -> ::unity2::Array<u8>;

    #[method(name = "ComputePublicKeyToken", args = 0)]
    pub fn compute_public_key_token(self) -> ::unity2::Array<u8>;

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();

    #[method(name = "Create", args = 2)]
    pub fn create(
        assembly: crate::system::reflection::assembly::Assembly,
        fill_codebase: bool,
    ) -> crate::system::reflection::assemblyname::AssemblyName;
}

#[cfg(feature = "system-reflection-assemblyname")]
impl AssemblyName {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssemblyName),
                ::core::stringify!(new),
            )
        });
        <Self as IAssemblyNameMethods>::ctor(this);
        this
    }

    pub fn new_2(assembly_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssemblyName),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAssemblyNameMethods>::ctor_2(this, assembly_name);
        this
    }
}
