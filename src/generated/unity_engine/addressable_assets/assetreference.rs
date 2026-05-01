
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/assetreference/AssetReference.md")))]
#[::unity2::class(namespace = "UnityEngine.AddressableAssets", name = "AssetReference")]
#[parent(crate::system::object::Object)]
pub struct AssetReference {
# [rename (name = "m_AssetGUID")] pub m_asset_guid : :: unity2 :: Il2CppString ,
# [rename (name = "m_SubObjectName")] pub m_sub_object_name : :: unity2 :: Il2CppString ,
# [rename (name = "m_SubObjectType")] pub m_sub_object_type : :: unity2 :: Il2CppString ,
# [rename (name = "m_Operation")] pub m_operation : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ,
}

#[cfg(feature = "unity_engine-addressable_assets-assetreference")]
#[::unity2::methods]
impl AssetReference {
    #[method(name = "get_OperationHandle", args = 0)]
    pub fn get_operation_handle (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "get_RuntimeKey", args = 0)]
    pub fn get_runtime_key(self) -> crate::system::object::Object;

    #[method(name = "get_AssetGUID", args = 0)]
    pub fn get_asset_guid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_SubObjectName", args = 0)]
    pub fn get_sub_object_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SubObjectName", args = 1)]
    pub fn set_sub_object_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SubOjbectType", args = 0)]
    pub fn get_sub_ojbect_type(self) -> ::unity2::SystemType;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "get_IsDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, guid: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Asset", args = 0)]
    pub fn get_asset(self) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "LoadScene", args = 0)]
    pub fn load_scene (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "Instantiate", args = 3)]
    pub fn instantiate (self , position : crate :: unity_engine :: vector3 :: Vector3 , rotation : crate :: unity_engine :: quaternion :: Quaternion , parent : crate :: unity_engine :: transform :: Transform) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "Instantiate", args = 2)]
    pub fn instantiate_2 (self , parent : crate :: unity_engine :: transform :: Transform , instantiate_in_world_space : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "LoadSceneAsync", args = 3)]
    pub fn load_scene_async (self , load_mode : crate :: unity_engine :: scene_management :: loadscenemode :: LoadSceneMode , activate_on_load : bool , priority : i32) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "UnLoadScene", args = 0)]
    pub fn un_load_scene (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "InstantiateAsync", args = 3)]
    pub fn instantiate_async (self , position : crate :: unity_engine :: vector3 :: Vector3 , rotation : crate :: unity_engine :: quaternion :: Quaternion , parent : crate :: unity_engine :: transform :: Transform) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateAsync", args = 2)]
    pub fn instantiate_async_2 (self , parent : crate :: unity_engine :: transform :: Transform , instantiate_in_world_space : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "RuntimeKeyIsValid", args = 0)]
    pub fn runtime_key_is_valid(self) -> bool;

    #[method(name = "ReleaseAsset", args = 0)]
    pub fn release_asset(self) -> ();

    #[method(name = "ReleaseInstance", args = 1)]
    pub fn release_instance(self, obj: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "ValidateAsset", args = 1)]
    pub fn validate_asset(self, obj: crate::unity_engine::object_2::Object_2) -> bool;

    #[method(name = "ValidateAsset", args = 1)]
    pub fn validate_asset_2(self, path: ::unity2::Il2CppString) -> bool;
}

#[cfg(feature = "unity_engine-addressable_assets-assetreference")]
impl AssetReference {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetReference),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetReferenceMethods>::ctor(this);
        this
    }

    pub fn new_2(guid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetReference),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAssetReferenceMethods>::ctor_2(this, guid);
        this
    }
}
