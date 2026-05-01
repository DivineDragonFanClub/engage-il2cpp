
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/manifestresourceinfo/ManifestResourceInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "ManifestResourceInfo")]
#[parent(crate::system::object::Object)]
pub struct ManifestResourceInfo {
    #[rename(name = "_containingAssembly")]
    pub containing_assembly: crate::system::reflection::assembly::Assembly,
    #[rename(name = "_containingFileName")]
    pub containing_file_name: ::unity2::Il2CppString,
    #[rename(name = "_resourceLocation")]
    pub resource_location: crate::system::reflection::resourcelocation::ResourceLocation,
}

#[cfg(feature = "system-reflection-manifestresourceinfo")]
#[::unity2::methods]
impl ManifestResourceInfo {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        containing_assembly: crate::system::reflection::assembly::Assembly,
        containing_file_name: ::unity2::Il2CppString,
        resource_location: crate::system::reflection::resourcelocation::ResourceLocation,
    ) -> ();

    #[method(name = "get_ReferencedAssembly", args = 0)]
    pub fn get_referenced_assembly(self) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "get_FileName", args = 0)]
    pub fn get_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ResourceLocation", args = 0)]
    pub fn get_resource_location(
        self,
    ) -> crate::system::reflection::resourcelocation::ResourceLocation;
}

#[cfg(feature = "system-reflection-manifestresourceinfo")]
impl ManifestResourceInfo {
    pub fn new(
        containing_assembly: crate::system::reflection::assembly::Assembly,
        containing_file_name: ::unity2::Il2CppString,
        resource_location: crate::system::reflection::resourcelocation::ResourceLocation,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ManifestResourceInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IManifestResourceInfoMethods>::ctor(
            this,
            containing_assembly,
            containing_file_name,
            resource_location,
        );
        this
    }
}
