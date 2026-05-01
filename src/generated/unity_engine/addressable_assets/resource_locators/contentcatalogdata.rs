
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/resource_locators/contentcatalogdata/ContentCatalogData_CompactLocation.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.ResourceLocators",
    name = "ContentCatalogData.CompactLocation"
)]
#[parent(crate::system::object::Object)]
pub struct ContentCatalogData_CompactLocation {
# [rename (name = "m_Locator")] pub m_locator : crate :: unity_engine :: addressable_assets :: resource_locators :: resourcelocationmap :: ResourceLocationMap ,
# [rename (name = "m_InternalId")] pub m_internal_id : :: unity2 :: Il2CppString ,
# [rename (name = "m_ProviderId")] pub m_provider_id : :: unity2 :: Il2CppString ,
# [rename (name = "m_Dependency")] pub m_dependency : :: unity2 :: IlInstance ,
# [rename (name = "m_Data")] pub m_data : :: unity2 :: IlInstance ,
# [rename (name = "m_HashCode")] pub m_hash_code : i32 ,
# [rename (name = "m_DependencyHashCode")] pub m_dependency_hash_code : i32 ,
# [rename (name = "m_PrimaryKey")] pub m_primary_key : :: unity2 :: Il2CppString ,
# [rename (name = "m_Type")] pub m_type : :: unity2 :: SystemType ,
}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-contentcatalogdata")]
#[::unity2::methods]
impl ContentCatalogData_CompactLocation {
    #[method(name = "get_InternalId", args = 0)]
    pub fn get_internal_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ProviderId", args = 0)]
    pub fn get_provider_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Dependencies", args = 0)]
    pub fn get_dependencies (self ,) -> crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > ;

    #[method(name = "get_HasDependencies", args = 0)]
    pub fn get_has_dependencies(self) -> bool;

    #[method(name = "get_DependencyHashCode", args = 0)]
    pub fn get_dependency_hash_code(self) -> i32;

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::system::object::Object;

    #[method(name = "get_PrimaryKey", args = 0)]
    pub fn get_primary_key(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PrimaryKey", args = 1)]
    pub fn set_primary_key(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ResourceType", args = 0)]
    pub fn get_resource_type(self) -> ::unity2::SystemType;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Hash", args = 1)]
    pub fn hash(self, t: ::unity2::SystemType) -> i32;

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        locator : crate :: unity_engine :: addressable_assets :: resource_locators :: resourcelocationmap :: ResourceLocationMap,
        internal_id: ::unity2::Il2CppString,
        provider_id: ::unity2::Il2CppString,
        dependency_key: crate::system::object::Object,
        data: crate::system::object::Object,
        dep_hash: i32,
        primary_key: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-contentcatalogdata")]
impl ContentCatalogData_CompactLocation {
    pub fn new(
        locator : crate :: unity_engine :: addressable_assets :: resource_locators :: resourcelocationmap :: ResourceLocationMap,
        internal_id: ::unity2::Il2CppString,
        provider_id: ::unity2::Il2CppString,
        dependency_key: crate::system::object::Object,
        data: crate::system::object::Object,
        dep_hash: i32,
        primary_key: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ContentCatalogData_CompactLocation),
                ::core::stringify!(new),
            )
        });
        <Self as IContentCatalogData_CompactLocationMethods>::ctor(
            this,
            locator,
            internal_id,
            provider_id,
            dependency_key,
            data,
            dep_hash,
            primary_key,
            r#type,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/resource_locators/contentcatalogdata/ContentCatalogData_Bucket.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ContentCatalogData_Bucket {
    pub data_offset: i32,
    pub entries: ::unity2::Array<i32>,
}

impl ::unity2::ClassIdentity for ContentCatalogData_Bucket {
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets.ResourceLocators";

    const NAME: &'static str = "ContentCatalogData.Bucket";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ContentCatalogData_Bucket {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/resource_locators/contentcatalogdata/ContentCatalogData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.ResourceLocators",
    name = "ContentCatalogData"
)]
#[parent(crate::system::object::Object)]
pub struct ContentCatalogData {
# [rename (name = "localHash")] pub local_hash : :: unity2 :: Il2CppString ,
# [rename (name = "location")] pub location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ,
# [rename (name = "m_LocatorId")] pub m_locator_id : :: unity2 :: Il2CppString ,
# [rename (name = "m_InstanceProviderData")] pub m_instance_provider_data : crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData ,
# [rename (name = "m_SceneProviderData")] pub m_scene_provider_data : crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData ,
# [rename (name = "m_ResourceProviderData")] pub m_resource_provider_data : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData > ,
# [rename (name = "m_ProviderIds")] pub m_provider_ids : :: unity2 :: Array < :: unity2 :: Il2CppString > ,
# [rename (name = "m_InternalIds")] pub m_internal_ids : :: unity2 :: Array < :: unity2 :: Il2CppString > ,
# [rename (name = "m_KeyDataString")] pub m_key_data_string : :: unity2 :: Il2CppString ,
# [rename (name = "m_BucketDataString")] pub m_bucket_data_string : :: unity2 :: Il2CppString ,
# [rename (name = "m_EntryDataString")] pub m_entry_data_string : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "kBytesPerInt32")] pub k_bytes_per_int32 : i32 ,
# [static_field] # [rename (name = "k_EntryDataItemPerEntry")] pub k_entry_data_item_per_entry : i32 ,
# [rename (name = "m_ExtraDataString")] pub m_extra_data_string : :: unity2 :: Il2CppString ,
# [rename (name = "m_resourceTypes")] pub m_resource_types : :: unity2 :: Array < crate :: unity_engine :: resource_management :: util :: serializedtype :: SerializedType > ,
# [rename (name = "m_InternalIdPrefixes")] pub m_internal_id_prefixes : :: unity2 :: Array < :: unity2 :: Il2CppString > ,
}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-contentcatalogdata")]
#[::unity2::methods]
impl ContentCatalogData {
    #[method(name = "get_ProviderId", args = 0)]
    pub fn get_provider_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ProviderId", args = 1)]
    pub fn set_provider_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_InstanceProviderData", args = 0)]
    pub fn get_instance_provider_data (self ,) -> crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData ;

    #[method(name = "set_InstanceProviderData", args = 1)]
    pub fn set_instance_provider_data(
        self,
        value : crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData,
    ) -> ();

    #[method(name = "get_SceneProviderData", args = 0)]
    pub fn get_scene_provider_data (self ,) -> crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData ;

    #[method(name = "set_SceneProviderData", args = 1)]
    pub fn set_scene_provider_data(
        self,
        value : crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData,
    ) -> ();

    #[method(name = "get_ResourceProviderData", args = 0)]
    pub fn get_resource_provider_data (self ,) -> crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData > ;

    #[method(name = "set_ResourceProviderData", args = 1)]
    pub fn set_resource_provider_data(
        self,
        value : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData >,
    ) -> ();

    #[method(name = "get_ProviderIds", args = 0)]
    pub fn get_provider_ids(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "get_InternalIds", args = 0)]
    pub fn get_internal_ids(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "CleanData", args = 0)]
    pub fn clean_data(self) -> ();

    #[method(name = "CreateCustomLocator", args = 2)]
    pub fn create_custom_locator (self , override_id : :: unity2 :: Il2CppString , provider_suffix : :: unity2 :: Il2CppString) -> crate :: unity_engine :: addressable_assets :: resource_locators :: resourcelocationmap :: ResourceLocationMap ;

    #[method(name = "CreateLocator", args = 1)]
    pub fn create_locator (self , provider_suffix : :: unity2 :: Il2CppString) -> crate :: unity_engine :: addressable_assets :: resource_locators :: resourcelocationmap :: ResourceLocationMap ;

    #[method(name = "ExpandInternalId", args = 2)]
    pub fn expand_internal_id(
        internal_id_prefixes: ::unity2::Array<::unity2::Il2CppString>,
        v: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-contentcatalogdata")]
impl ContentCatalogData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ContentCatalogData),
                ::core::stringify!(new),
            )
        });
        <Self as IContentCatalogDataMethods>::ctor(this);
        this
    }
}
