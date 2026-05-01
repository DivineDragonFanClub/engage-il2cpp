
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusmapeditcontent/VersusMapEditContent.md")))]
#[::unity2::class(namespace = "App", name = "VersusMapEditContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct VersusMapEditContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SpriteAtlasThumbPath")]
    pub sprite_atlas_thumb_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SpriteAtlasCategoryPath")]
    pub sprite_atlas_category_path: ::unity2::Il2CppString,
    #[rename(name = "m_ThumbSpriteAtlas")]
    pub m_thumb_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_ThumbSprite")]
    pub m_thumb_sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_CategorySpriteAtlas")]
    pub m_category_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_CategorySprite")]
    pub m_category_sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_ThumbImage")]
    pub m_thumb_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CategoryImage")]
    pub m_category_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_HelpNum")]
    pub m_help_num: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Anim")]
    pub m_anim: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CategoryItems")]
    pub m_category_items: crate::system::collections::generic::list_1::List_1<
        crate::app::versusmapeditcategorycontent::VersusMapEditCategoryContent,
    >,
}

#[cfg(feature = "app-versusmapeditcontent")]
#[::unity2::methods]
impl VersusMapEditContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::versusmapeditcontent::VersusMapEditContent;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "SetImage", args = 1)]
    pub fn set_image(self, data: crate::app::mapeditorobjectdata::MapEditorObjectData) -> ();

    #[method(name = "SetTextNum", args = 3)]
    pub fn set_text_num(
        self,
        category: crate::app::mapeditorcategorydata::MapEditorCategoryData,
        num: i32,
        num_max: i32,
    ) -> ();

    #[method(name = "SetHighlight", args = 1)]
    pub fn set_highlight(
        self,
        category: crate::app::mapeditorcategorydata::MapEditorCategoryData,
    ) -> ();

    #[method(name = "GetThumbSprite", args = 1)]
    pub fn get_thumb_sprite(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetCategorySprite", args = 1)]
    pub fn get_category_sprite(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "OpenToOne", args = 0)]
    pub fn open_to_one(self) -> ();

    #[method(name = "OpenToEveryone", args = 0)]
    pub fn open_to_everyone(self) -> ();

    #[method(name = "IsOpenToEveryone", args = 0)]
    pub fn is_open_to_everyone(self) -> bool;

    #[method(name = "IsOpenToEveryoneEnd", args = 0)]
    pub fn is_open_to_everyone_end(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versusmapeditcontent")]
impl VersusMapEditContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusMapEditContent),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusMapEditContentMethods>::ctor(this);
        this
    }
}
