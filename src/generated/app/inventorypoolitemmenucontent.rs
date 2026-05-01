
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::system::object::IObject;
use crate::system::object::Object;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorypoolitemmenucontent/InventoryPoolItemMenuContent_KindBgNameClass.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct InventoryPoolItemMenuContent_KindBgNameClass {
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
    pub m_bg: crate::unity_engine::ui::image::Image,
}

impl ::unity2::ClassIdentity for InventoryPoolItemMenuContent_KindBgNameClass {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "InventoryPoolItemMenuContent.KindBgNameClass";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for InventoryPoolItemMenuContent_KindBgNameClass {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventorypoolitemmenucontent/InventoryPoolItemMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "InventoryPoolItemMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct InventoryPoolItemMenuContent {
    #[rename(name = "m_MenuTitleText")]
    pub m_menu_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TransporterText")]
    pub m_transporter_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_StockValueText")]
    pub m_stock_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_StockValueMaxText")]
    pub m_stock_value_max_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_KindBgName")]
    pub m_kind_bg_name: crate::system::collections::generic::list_1::List_1<
        crate::app::inventorypoolitemmenucontent::InventoryPoolItemMenuContent_KindBgNameClass,
    >,
}

#[cfg(feature = "app-inventorypoolitemmenucontent")]
#[::unity2::methods]
impl InventoryPoolItemMenuContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "GetKindNum", args = 0)]
    pub fn get_kind_num(self) -> i32;

    #[method(name = "GetFirstKind", args = 0)]
    pub fn get_first_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "GetLastKind", args = 0)]
    pub fn get_last_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "GetNextKind", args = 1)]
    pub fn get_next_kind(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
    ) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "GetPrevKind", args = 1)]
    pub fn get_prev_kind(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
    ) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "GetKindIndex", args = 1)]
    pub fn get_kind_index(self, kind: crate::app::itemdata::ItemData_Kinds) -> i32;

    #[method(name = "SetKind", args = 1)]
    pub fn set_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "SetStockValue", args = 1)]
    pub fn set_stock_value(self, value: i32) -> ();

    #[method(name = "SetStockValueMax", args = 1)]
    pub fn set_stock_value_max(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventorypoolitemmenucontent")]
impl InventoryPoolItemMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventoryPoolItemMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IInventoryPoolItemMenuContentMethods>::ctor(this);
        this
    }
}
