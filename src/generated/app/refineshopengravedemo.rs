
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravedemo/RefineShopEngraveDemo_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RefineShopEngraveDemo_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RefineShopEngraveDemo_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RefineShopEngraveDemo.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RefineShopEngraveDemo_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RefineShopEngraveDemo_Label {
    pub fn end() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravedemo/RefineShopEngraveDemo.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopEngraveDemo")]
#[parent(crate::app::procinst::ProcInst)]
pub struct RefineShopEngraveDemo {
    #[rename(name = "m_RefineShopEngraveDemoRoot")]
    pub m_refine_shop_engrave_demo_root:
        crate::app::refineshopengravedemoroot::RefineShopEngraveDemoRoot,
    #[rename(name = "m_ShopWeaponModelRenderer")]
    pub m_shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
}

#[cfg(feature = "app-refineshopengravedemo")]
#[::unity2::methods]
impl RefineShopEngraveDemo {
    #[method(name = "get_m_BaseUnitItem", args = 0)]
    pub fn get_m_base_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_BaseUnitItem", args = 1)]
    pub fn set_m_base_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_EngravedUnitItem", args = 0)]
    pub fn get_m_engraved_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_EngravedUnitItem", args = 1)]
    pub fn set_m_engraved_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_m_GodData", args = 0)]
    pub fn get_m_god_data(self) -> crate::app::goddata::GodData;

    #[method(name = "set_m_GodData", args = 1)]
    pub fn set_m_god_data(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
        base_unit_item: crate::app::unititem::UnitItem,
        engraved_unit_item: crate::app::unititem::UnitItem,
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::refineshopengravedemo::RefineShopEngraveDemo;

    #[method(name = "CreateDefaultDesc", args = 0)]
    pub fn create_default_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        super_: crate::app::procinst::ProcInst,
        shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
        base_unit_item: crate::app::unititem::UnitItem,
        engraved_unit_item: crate::app::unititem::UnitItem,
        god_data: crate::app::goddata::GodData,
    ) -> ();

    #[method(name = "LoadPrefabs", args = 0)]
    pub fn load_prefabs(self) -> ();

    #[method(name = "IsLoadingPrefabs", args = 0)]
    pub fn is_loading_prefabs(self) -> bool;

    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "StartSound", args = 0)]
    pub fn start_sound(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "DestroyAndUnloadPrefabs", args = 0)]
    pub fn destroy_and_unload_prefabs(self) -> ();
}

#[cfg(feature = "app-refineshopengravedemo")]
impl RefineShopEngraveDemo {
    pub fn new(
        super_: crate::app::procinst::ProcInst,
        shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
        base_unit_item: crate::app::unititem::UnitItem,
        engraved_unit_item: crate::app::unititem::UnitItem,
        god_data: crate::app::goddata::GodData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveDemo),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveDemoMethods>::ctor(
            this,
            super_,
            shop_weapon_model_renderer,
            base_unit_item,
            engraved_unit_item,
            god_data,
        );
        this
    }
}
