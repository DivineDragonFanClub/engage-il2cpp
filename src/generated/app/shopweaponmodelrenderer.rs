
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopweaponmodelrenderer/ShopWeaponModelRenderer.md")))]
#[::unity2::class(namespace = "App", name = "ShopWeaponModelRenderer")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ShopWeaponModelRenderer {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_ModelRoot")]
    pub m_model_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ModelPath")]
    pub m_model_path: ::unity2::Il2CppString,
    #[rename(name = "m_ModelObject")]
    pub m_model_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ModelPathLoading")]
    pub m_model_path_loading: ::unity2::Il2CppString,
    #[rename(name = "m_ItemData")]
    pub m_item_data: crate::app::itemdata::ItemData,
}

#[cfg(feature = "app-shopweaponmodelrenderer")]
#[::unity2::methods]
impl ShopWeaponModelRenderer {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer;

    #[method(name = "CreateInternal", args = 0)]
    pub fn create_internal(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "GetRenderTexture", args = 0)]
    pub fn get_render_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "SetWeapon", args = 2)]
    pub fn set_weapon(self, item_data: crate::app::itemdata::ItemData, delay_loading: bool) -> ();

    #[method(name = "LoadWeaponModel", args = 1)]
    pub fn load_weapon_model(self, item_data: crate::app::itemdata::ItemData) -> ();

    #[method(name = "OnFinishLoadingWeapon", args = 1)]
    pub fn on_finish_loading_weapon(
        self,
        game_object_in: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "RemoveWeapon", args = 0)]
    pub fn remove_weapon(self) -> ();

    #[method(name = "RemoveWeaponCore", args = 0)]
    pub fn remove_weapon_core(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-shopweaponmodelrenderer")]
impl ShopWeaponModelRenderer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopWeaponModelRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as IShopWeaponModelRendererMethods>::ctor(this);
        this
    }
}
