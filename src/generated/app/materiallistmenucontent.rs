
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/materiallistmenucontent/MaterialListMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "MaterialListMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MaterialListMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_ItemHelpText")]
    pub m_item_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CategoryIconList")]
    pub m_category_icon_list:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::ui::image::Image>,
}

#[cfg(feature = "app-materiallistmenucontent")]
#[::unity2::methods]
impl MaterialListMenuContent {
    #[method(name = "get_Category", args = 0)]
    pub fn get_category(
        self,
    ) -> crate::app::materiallistmenucontent::MaterialListMenuContent_CategoryType;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(
        self,
        value: crate::app::materiallistmenucontent::MaterialListMenuContent_CategoryType,
    ) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::materiallistmenucontent::MaterialListMenuContent;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "InitCategory", args = 0)]
    pub fn init_category(self) -> ();

    #[method(name = "LeftCategory", args = 1)]
    pub fn left_category(self, is_trigger: bool) -> ();

    #[method(name = "RightCategory", args = 1)]
    pub fn right_category(self, is_trigger: bool) -> ();

    #[method(name = "SetItemHelpText", args = 1)]
    pub fn set_item_help_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-materiallistmenucontent")]
impl MaterialListMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialListMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialListMenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/materiallistmenucontent/MaterialListMenuContent_CategoryType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MaterialListMenuContent_CategoryType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MaterialListMenuContent_CategoryType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MaterialListMenuContent.CategoryType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MaterialListMenuContent_CategoryType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MaterialListMenuContent_CategoryType {
    pub fn all() -> Self {
        Self { value: 0 }
    }

    pub fn gift() -> Self {
        Self { value: 1 }
    }

    pub fn food() -> Self {
        Self { value: 2 }
    }

    pub fn material() -> Self {
        Self { value: 3 }
    }

    pub fn other() -> Self {
        Self { value: 4 }
    }

    pub fn count() -> Self {
        Self { value: 5 }
    }

    pub fn begin() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }
}
