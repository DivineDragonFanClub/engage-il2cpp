
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubrefineshopsequence/HubRefineShopSequence.md")))]
#[::unity2::class(namespace = "App", name = "HubRefineShopSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubRefineShopSequence {
    #[rename(name = "m_WeaponModelRenderer")]
    pub m_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
    #[rename(name = "m_RefineShopRefineBaseRoot")]
    pub m_refine_shop_refine_base_root:
        crate::app::refineshoprefinebaseroot::RefineShopRefineBaseRoot,
    #[rename(name = "m_RefineShopRefineTargetRoot")]
    pub m_refine_shop_refine_target_root:
        crate::app::refineshoprefinetargetroot::RefineShopRefineTargetRoot,
    #[rename(name = "m_RefineShopEngraveItemSelectRoot")]
    pub m_refine_shop_engrave_item_select_root:
        crate::app::refineshopengraveitemselectroot::RefineShopEngraveItemSelectRoot,
    #[rename(name = "m_RefineShopEngraveGodRoot")]
    pub m_refine_shop_engrave_god_root:
        crate::app::refineshopengravegodroot::RefineShopEngraveGodRoot,
    #[rename(name = "m_RefineShopEngraveDemoRoot")]
    pub m_refine_shop_engrave_demo_root:
        crate::app::refineshopengravedemoroot::RefineShopEngraveDemoRoot,
    #[rename(name = "m_RefineShopExchangeMenuTop")]
    pub m_refine_shop_exchange_menu_top:
        crate::app::refineshopexchangemenutop::RefineShopExchangeMenuTop,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::basicmenu::BasicMenu_Result,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_OwnerItemIndex")]
    pub m_owner_item_index: i32,
    #[rename(name = "m_BaseUnitItem")]
    pub m_base_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_AfterUnitItem")]
    pub m_after_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_GodData")]
    pub m_god_data: crate::app::goddata::GodData,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
}

#[cfg(feature = "app-hubrefineshopsequence")]
#[::unity2::methods]
impl HubRefineShopSequence {
    #[method(name = "get_m_TopMenuResult", args = 0)]
    pub fn get_m_top_menu_result(self) -> crate::app::refineshoptopmenu::RefineShopTopMenu_Result2;

    #[method(name = "set_m_TopMenuResult", args = 1)]
    pub fn set_m_top_menu_result(
        self,
        value: crate::app::refineshoptopmenu::RefineShopTopMenu_Result2,
    ) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "CreateRefineShopTopMenu", args = 0)]
    pub fn create_refine_shop_top_menu(self) -> ();

    #[method(name = "CreateRefineShopRefineMenu", args = 0)]
    pub fn create_refine_shop_refine_menu(self) -> ();

    #[method(name = "DestroyRefineShopRefineMenu", args = 0)]
    pub fn destroy_refine_shop_refine_menu(self) -> ();

    #[method(name = "CreateRefineShopRefineTargetMenu", args = 0)]
    pub fn create_refine_shop_refine_target_menu(self) -> ();

    #[method(name = "DestroyRefineShopRefineTargetMenu", args = 0)]
    pub fn destroy_refine_shop_refine_target_menu(self) -> ();

    #[method(name = "CreateRefineShopEngraveItemSelectMenu", args = 0)]
    pub fn create_refine_shop_engrave_item_select_menu(self) -> ();

    #[method(name = "DestroyRefineShopEngraveItemSelectMenu", args = 0)]
    pub fn destroy_refine_shop_engrave_item_select_menu(self) -> ();

    #[method(name = "CreateRefineShopEngraveGodMenu", args = 0)]
    pub fn create_refine_shop_engrave_god_menu(self) -> ();

    #[method(name = "DestroyRefineShopEngraveGodMenu", args = 0)]
    pub fn destroy_refine_shop_engrave_god_menu(self) -> ();

    #[method(name = "PrepareRefineShopEngraveDemo", args = 0)]
    pub fn prepare_refine_shop_engrave_demo(self) -> ();

    #[method(name = "WaitPreparingRefineShopEngraveDemo", args = 0)]
    pub fn wait_preparing_refine_shop_engrave_demo(self) -> bool;

    #[method(name = "CreateRefineShopEngraveDemo", args = 0)]
    pub fn create_refine_shop_engrave_demo(self) -> ();

    #[method(name = "DestroyRefineShopEngraveDemo", args = 0)]
    pub fn destroy_refine_shop_engrave_demo(self) -> ();

    #[method(name = "CreateRefineShopExchangeMenu", args = 0)]
    pub fn create_refine_shop_exchange_menu(self) -> ();

    #[method(name = "DestroyRefineShopExchangeMenu", args = 0)]
    pub fn destroy_refine_shop_exchange_menu(self) -> ();

    #[method(name = "HideTitle", args = 0)]
    pub fn hide_title(self) -> ();

    #[method(name = "ShowTitle", args = 0)]
    pub fn show_title(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();
}

#[cfg(feature = "app-hubrefineshopsequence")]
impl HubRefineShopSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubRefineShopSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IHubRefineShopSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubrefineshopsequence/HubRefineShopSequence_Label2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubRefineShopSequence_Label2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubRefineShopSequence_Label2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubRefineShopSequence.Label2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubRefineShopSequence_Label2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubRefineShopSequence_Label2 {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn top() -> Self {
        Self { value: 1 }
    }

    pub fn refine() -> Self {
        Self { value: 2 }
    }

    pub fn refine_list() -> Self {
        Self { value: 3 }
    }

    pub fn engrave() -> Self {
        Self { value: 4 }
    }

    pub fn engrave_god_list() -> Self {
        Self { value: 5 }
    }

    pub fn execute_engrave() -> Self {
        Self { value: 6 }
    }

    pub fn exchange() -> Self {
        Self { value: 7 }
    }

    pub fn exchange_yes_no() -> Self {
        Self { value: 8 }
    }

    pub fn execute_exchange() -> Self {
        Self { value: 9 }
    }

    pub fn end() -> Self {
        Self { value: 10 }
    }
}
