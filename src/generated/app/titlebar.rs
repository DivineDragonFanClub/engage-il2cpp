
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlebar/TitleBar_Num.md")))]
#[::unity2::class(namespace = "App", name = "TitleBar.Num")]
#[parent(crate::system::object::Object)]
pub struct TitleBar_Num {
    #[rename(name = "Gold")]
    pub gold: i32,
    #[rename(name = "PieceOfBond")]
    pub piece_of_bond: i32,
    #[rename(name = "RefineSilver")]
    pub refine_silver: i32,
    #[rename(name = "RefineSteel")]
    pub refine_steel: i32,
    #[rename(name = "RefineIron")]
    pub refine_iron: i32,
    #[rename(name = "RefineGodList")]
    pub refine_god_list: ::unity2::Array<i32>,
    #[rename(name = "ProofMaster")]
    pub proof_master: i32,
    #[rename(name = "ProofChange")]
    pub proof_change: i32,
    #[rename(name = "ProofEnchant")]
    pub proof_enchant: i32,
    #[rename(name = "ProofGunner")]
    pub proof_gunner: i32,
    #[rename(name = "RelayTicket")]
    pub relay_ticket: i32,
}

#[cfg(feature = "app-titlebar")]
#[::unity2::methods]
impl TitleBar_Num {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetValue", args = 1)]
    pub fn set_value(self, value: crate::app::titlebar::TitleBar_Num) -> ();

    #[method(name = "SetValueCurrent", args = 0)]
    pub fn set_value_current(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-titlebar")]
impl TitleBar_Num {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleBar_Num),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleBar_NumMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlebar/TitleBar_FooterType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TitleBar_FooterType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TitleBar_FooterType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TitleBar.FooterType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TitleBar_FooterType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TitleBar_FooterType {
    pub fn gold() -> Self {
        Self { value: 1 }
    }

    pub fn piece_of_bond() -> Self {
        Self { value: 2 }
    }

    pub fn refine() -> Self {
        Self { value: 4 }
    }

    pub fn refine_god() -> Self {
        Self { value: 8 }
    }

    pub fn proof() -> Self {
        Self { value: 16 }
    }

    pub fn relay_ticket() -> Self {
        Self { value: 32 }
    }

    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn gold_and_bond() -> Self {
        Self { value: 3 }
    }

    pub fn gold_and_refine() -> Self {
        Self { value: 5 }
    }

    pub fn gold_and_bond_and_refine() -> Self {
        Self { value: 7 }
    }

    pub fn gold_and_bond_and_relay_ticket() -> Self {
        Self { value: 35 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlebar/TitleBar_Values.md")))]
#[::unity2::class(namespace = "App", name = "TitleBar.Values")]
#[parent(crate::system::object::Object)]
pub struct TitleBar_Values {
    #[rename(name = "Root")]
    pub root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "Animator")]
    pub animator: crate::unity_engine::animator::Animator,
    #[rename(name = "MaterialObjList")]
    pub material_obj_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "PieceOfBondObject")]
    pub piece_of_bond_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "PieceOfBondValue")]
    pub piece_of_bond_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "MoneyObject")]
    pub money_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "MoneyValue")]
    pub money_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-titlebar")]
#[::unity2::methods]
impl TitleBar_Values {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-titlebar")]
impl TitleBar_Values {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleBar_Values),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleBar_ValuesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlebar/TitleBar_AnimType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TitleBar_AnimType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TitleBar_AnimType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TitleBar.AnimType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TitleBar_AnimType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TitleBar_AnimType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn open() -> Self {
        Self { value: 1 }
    }

    pub fn close() -> Self {
        Self { value: 2 }
    }

    pub fn invalid() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlebar/TitleBar.md")))]
#[::unity2::class(namespace = "App", name = "TitleBar")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TitleBar {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_AnimatorHeader")]
    pub m_animator_header: crate::unity_engine::animator::Animator,
    #[rename(name = "m_AnimatorFooter")]
    pub m_animator_footer: crate::unity_engine::animator::Animator,
    #[rename(name = "m_Title0")]
    pub m_title0: crate::app::titlebar::TitleBar_Title,
    #[rename(name = "m_Title1")]
    pub m_title1: crate::app::titlebar::TitleBar_Title,
    #[rename(name = "m_Values0")]
    pub m_values0: crate::app::titlebar::TitleBar_Values,
    #[rename(name = "m_Values1")]
    pub m_values1: crate::app::titlebar::TitleBar_Values,
    #[rename(name = "m_CountTime")]
    pub m_count_time: f32,
    #[rename(name = "m_CurrentTitle")]
    pub m_current_title: crate::app::titlebar::TitleBar_Title,
    #[rename(name = "m_CurrentValues")]
    pub m_current_values: crate::app::titlebar::TitleBar_Values,
    #[rename(name = "m_HeaderTitle")]
    pub m_header_title: ::unity2::Il2CppString,
    #[rename(name = "m_HeaderTitleHelp")]
    pub m_header_title_help: ::unity2::Il2CppString,
    #[rename(name = "m_HeaderKeyHelp")]
    pub m_header_key_help: ::unity2::Il2CppString,
    #[rename(name = "m_FooterType")]
    pub m_footer_type: crate::app::titlebar::TitleBar_FooterType,
    #[rename(name = "m_IsShowHeader")]
    pub m_is_show_header: bool,
    #[rename(name = "m_IsShowFooter")]
    pub m_is_show_footer: bool,
    #[rename(name = "m_IsOpenHeader")]
    pub m_is_open_header: bool,
    #[rename(name = "m_IsOpenFooter")]
    pub m_is_open_footer: bool,
    #[rename(name = "m_IsSameFooter")]
    pub m_is_same_footer: bool,
    #[rename(name = "m_NumDisplay")]
    pub m_num_display: crate::app::titlebar::TitleBar_Num,
    #[rename(name = "m_NumCurrent")]
    pub m_num_current: crate::app::titlebar::TitleBar_Num,
    #[rename(name = "m_NumCountValue")]
    pub m_num_count_value: crate::app::titlebar::TitleBar_Num,
    #[rename(name = "m_NumCount")]
    pub m_num_count: f32,
    #[rename(name = "m_ValueCountList")]
    pub m_value_count_list: crate::system::collections::generic::list_1::List_1<
        crate::app::valuecountcontroller::ValueCountController,
    >,
}

#[cfg(feature = "app-titlebar")]
#[::unity2::methods]
impl TitleBar {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> crate::app::titlebar::TitleBar;

    #[method(name = "set_Instance", args = 1)]
    pub fn set_instance(value: crate::app::titlebar::TitleBar) -> ();

    #[method(name = "CreateAsync", args = 1)]
    pub fn create_async(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "CreateInstance", args = 0)]
    pub fn create_instance() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::titlebar::TitleBar;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "HideHeader", args = 0)]
    pub fn hide_header(self) -> ();

    #[method(name = "HideHeaderKeyHelp", args = 1)]
    pub fn hide_header_key_help(self, key_help: ::unity2::Il2CppString) -> ();

    #[method(name = "HideFooter", args = 0)]
    pub fn hide_footer(self) -> ();

    #[method(name = "ShowHeader", args = 0)]
    pub fn show_header(self) -> ();

    #[method(name = "ShowHeaderKeyHelp", args = 0)]
    pub fn show_header_key_help(self) -> ();

    #[method(name = "ShowFooter", args = 0)]
    pub fn show_footer(self) -> ();

    #[method(name = "IsOpenHeader", args = 0)]
    pub fn is_open_header(self) -> bool;

    #[method(name = "IsOpenFooter", args = 0)]
    pub fn is_open_footer(self) -> bool;

    #[method(name = "OpenHeader", args = 3)]
    pub fn open_header(
        self,
        title: ::unity2::Il2CppString,
        title_help: ::unity2::Il2CppString,
        key_help_id: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "OpenHeaderSortie", args = 5)]
    pub fn open_header_sortie(
        self,
        title: ::unity2::Il2CppString,
        title_help: ::unity2::Il2CppString,
        key_help_id: ::unity2::Il2CppString,
        unit_num: i32,
        unit_max_num: i32,
    ) -> ();

    #[method(name = "OpenFooter", args = 1)]
    pub fn open_footer(self, r#type: crate::app::titlebar::TitleBar_FooterType) -> ();

    #[method(name = "CloseHeader", args = 0)]
    pub fn close_header(self) -> ();

    #[method(name = "CloseFooter", args = 0)]
    pub fn close_footer(self) -> ();

    #[method(name = "UpdateFooterValues", args = 0)]
    pub fn update_footer_values(self) -> ();

    #[method(name = "SetUnitNum", args = 2)]
    pub fn set_unit_num(self, num: i32, max_num: i32) -> ();

    #[method(name = "TransitHeader", args = 3)]
    pub fn transit_header(
        self,
        title: ::unity2::Il2CppString,
        title_help: ::unity2::Il2CppString,
        key_help_id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetAnimType", args = 1)]
    pub fn get_anim_type(
        self,
        animator: crate::unity_engine::animator::Animator,
    ) -> crate::app::titlebar::TitleBar_AnimType;

    #[method(name = "ClearHeaderAnim", args = 0)]
    pub fn clear_header_anim(self) -> ();

    #[method(name = "ClearFooterAnim", args = 0)]
    pub fn clear_footer_anim(self) -> ();

    #[method(name = "IsHeaderSortie", args = 0)]
    pub fn is_header_sortie(self) -> bool;

    #[method(name = "SetTitle", args = 3)]
    pub fn set_title(
        self,
        title: ::unity2::Il2CppString,
        title_help: ::unity2::Il2CppString,
        key_help_id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "IsFooterTypeGold", args = 0)]
    pub fn is_footer_type_gold(self) -> bool;

    #[method(name = "IsFooterTypePieceOfBond", args = 0)]
    pub fn is_footer_type_piece_of_bond(self) -> bool;

    #[method(name = "IsFooterTypeRefine", args = 0)]
    pub fn is_footer_type_refine(self) -> bool;

    #[method(name = "IsFooterTypeRefineGod", args = 0)]
    pub fn is_footer_type_refine_god(self) -> bool;

    #[method(name = "IsFooterTypeProof", args = 0)]
    pub fn is_footer_type_proof(self) -> bool;

    #[method(name = "IsFooterTypeRelayTicket", args = 0)]
    pub fn is_footer_type_relay_ticket(self) -> bool;

    #[method(name = "InitFooterValue", args = 1)]
    pub fn init_footer_value(self, values: crate::app::titlebar::TitleBar_Values) -> ();

    #[method(name = "SetFooterValue", args = 0)]
    pub fn set_footer_value(self) -> ();

    #[method(name = "SetFooterCountValue", args = 0)]
    pub fn set_footer_count_value(self) -> ();

    #[method(name = "SetMaterialActive", args = 4)]
    pub fn set_material_active(
        self,
        values: crate::app::titlebar::TitleBar_Values,
        index: i32,
        icon_sprite: crate::unity_engine::sprite::Sprite,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetMaterialValue", args = 3)]
    pub fn set_material_value(
        self,
        values: crate::app::titlebar::TitleBar_Values,
        index: i32,
        value: i32,
    ) -> ();

    #[method(name = "UpdateValueMoney", args = 1)]
    pub fn update_value_money(self, values: crate::app::titlebar::TitleBar_Values) -> ();

    #[method(name = "UpdateValuePieceOfBond", args = 1)]
    pub fn update_value_piece_of_bond(self, values: crate::app::titlebar::TitleBar_Values) -> ();

    #[method(name = "UpdateValueRefine", args = 1)]
    pub fn update_value_refine(self, values: crate::app::titlebar::TitleBar_Values) -> ();

    #[method(name = "UpdateValueRefineGod", args = 1)]
    pub fn update_value_refine_god(self, values: crate::app::titlebar::TitleBar_Values) -> ();

    #[method(name = "UpdateValueProof", args = 1)]
    pub fn update_value_proof(self, values: crate::app::titlebar::TitleBar_Values) -> ();

    #[method(name = "UpdateValueRelayTicket", args = 1)]
    pub fn update_value_relay_ticket(self, values: crate::app::titlebar::TitleBar_Values) -> ();

    #[method(name = "TransitTitleAnim", args = 0)]
    pub fn transit_title_anim(self) -> ();

    #[method(name = "TransitValuesAnim", args = 0)]
    pub fn transit_values_anim(self) -> ();

    #[method(name = "GetHiddenTitle", args = 0)]
    pub fn get_hidden_title(self) -> crate::app::titlebar::TitleBar_Title;

    #[method(name = "GetHiddenValues", args = 0)]
    pub fn get_hidden_values(self) -> crate::app::titlebar::TitleBar_Values;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-titlebar")]
impl TitleBar {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleBar),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleBarMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlebar/TitleBar_Title.md")))]
#[::unity2::class(namespace = "App", name = "TitleBar.Title")]
#[parent(crate::system::object::Object)]
pub struct TitleBar_Title {
    #[rename(name = "Root")]
    pub root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "Animator")]
    pub animator: crate::unity_engine::animator::Animator,
    #[rename(name = "TitleText")]
    pub title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "HelpText")]
    pub help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "UnitObj")]
    pub unit_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "UnitValue")]
    pub unit_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "UnitMaxValue")]
    pub unit_max_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "KeyHelp")]
    pub key_help: crate::app::keyhelptitlebarcontroller::KeyHelpTitleBarController,
}

#[cfg(feature = "app-titlebar")]
#[::unity2::methods]
impl TitleBar_Title {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-titlebar")]
impl TitleBar_Title {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleBar_Title),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleBar_TitleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/titlebar/TitleBar_TitleBarProc.md")))]
#[::unity2::class(namespace = "App", name = "TitleBar.TitleBarProc")]
#[parent(crate::app::procinst::ProcInst)]
pub struct TitleBar_TitleBarProc {}

#[cfg(feature = "app-titlebar")]
#[::unity2::methods]
impl TitleBar_TitleBarProc {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-titlebar")]
impl TitleBar_TitleBarProc {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TitleBar_TitleBarProc),
                ::core::stringify!(new),
            )
        });
        <Self as ITitleBar_TitleBarProcMethods>::ctor(this);
        this
    }
}
