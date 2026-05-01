
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_Sound.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AssetTable_Sound {
    pub voice_id: ::unity2::Il2CppString,
    pub footstep_id: ::unity2::Il2CppString,
    pub material_id: ::unity2::Il2CppString,
}

impl ::unity2::ClassIdentity for AssetTable_Sound {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AssetTable.Sound";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AssetTable_Sound {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-assettable")]
#[::unity2::methods(value)]
impl AssetTable_Sound {
    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_ConditionFlags.md")))]
#[::unity2::class(namespace = "App", name = "AssetTable.ConditionFlags")]
#[parent(crate::system::object::Object)]
pub struct AssetTable_ConditionFlags {
    #[rename(name = "m_Bits")]
    pub m_bits: crate::app::bitstruct::BitStruct,
    #[rename(name = "m_Keys")]
    pub m_keys: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_Hits")]
    pub m_hits: crate::system::collections::generic::list_1::List_1<i32>,
    #[rename(name = "m_Dics")]
    pub m_dics: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        i32,
    >,
}

#[cfg(feature = "app-assettable")]
#[::unity2::methods]
impl AssetTable_ConditionFlags {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Names", args = 0)]
    pub fn get_names(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "get_Hits", args = 0)]
    pub fn get_hits(self) -> crate::system::collections::generic::list_1::List_1<i32>;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Test", args = 1)]
    pub fn test(self, index: i32) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_2(self, key: ::unity2::Il2CppString) -> bool;

    #[method(name = "Add", args = 1)]
    pub fn add(self, keys: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_2(self, key: ::unity2::Il2CppString) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_3(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_4(self, force: crate::app::force::Force_Type) -> ();

    #[method(name = "GetState", args = 1)]
    pub fn get_state(unit: crate::app::unit::Unit) -> crate::app::assettable::AssetTable_States;

    #[method(name = "Add", args = 1)]
    pub fn add_5(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsSimpleMode", args = 0)]
    pub fn is_simple_mode(self) -> bool;

    #[method(name = "AddGender", args = 2)]
    pub fn add_gender(
        self,
        gender: crate::app::gender::Gender,
        dress_gender: crate::app::gender::Gender,
    ) -> ();

    #[method(name = "AddGender", args = 1)]
    pub fn add_gender_2(self, person: crate::app::persondata::PersonData) -> ();

    #[method(name = "AddGender", args = 1)]
    pub fn add_gender_3(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AddGender", args = 1)]
    pub fn add_gender_4(self, goid_data: crate::app::goddata::GodData) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_6(
        self,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
    ) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_7(
        self,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        force: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_8(self, skills: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_9(
        self,
        state: crate::app::assettable::AssetTable_States,
        god_data: crate::app::goddata::GodData,
        is_darkness: bool,
    ) -> ();

    #[method(name = "ReplaceGid2Eid", args = 1)]
    pub fn replace_gid2_eid(self, gid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-assettable")]
impl AssetTable_ConditionFlags {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetTable_ConditionFlags),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetTable_ConditionFlagsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_ConditionIndexes.md")))]
#[::unity2::class(namespace = "App", name = "AssetTable.ConditionIndexes")]
#[parent(crate::system::object::Object)]
pub struct AssetTable_ConditionIndexes {
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<
        crate::system::collections::generic::list_1::List_1<i32>,
    >,
}

#[cfg(feature = "app-assettable")]
#[::unity2::methods]
impl AssetTable_ConditionIndexes {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, indexes: crate::system::collections::generic::list_1::List_1<i32>) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "Test", args = 1)]
    pub fn test(self, flags: crate::app::assettable::AssetTable_ConditionFlags) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-assettable")]
impl AssetTable_ConditionIndexes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetTable_ConditionIndexes),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetTable_ConditionIndexesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable.md")))]
#[::unity2::class(namespace = "App", name = "AssetTable")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: assettable :: AssetTable >)]
pub struct AssetTable {
    #[static_field]
    #[rename(name = "s_PresetNames")]
    pub s_preset_names: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "HairColor")]
    pub hair_color: crate::unity_engine::color::Color,
    #[rename(name = "GradColor")]
    pub grad_color: crate::unity_engine::color::Color,
    #[rename(name = "SkinColor")]
    pub skin_color: crate::unity_engine::color::Color,
    #[rename(name = "ToonShadowColor")]
    pub toon_shadow_color: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor100")]
    pub mask_color100: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor075")]
    pub mask_color075: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor050")]
    pub mask_color050: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor025")]
    pub mask_color025: crate::unity_engine::color::Color,
    #[rename(name = "m_ConditionIndexes")]
    pub m_condition_indexes: crate::app::assettable::AssetTable_ConditionIndexes,
    #[static_field]
    #[rename(name = "BitCount")]
    pub bit_count: i32,
    #[static_field]
    #[rename(name = "s_SearchLists")]
    pub s_search_lists: ::unity2::Array<
        crate::system::collections::generic::list_1::List_1<crate::app::assettable::AssetTable>,
    >,
    #[static_field]
    #[rename(name = "s_ConditionIndexes")]
    pub s_condition_indexes: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        i32,
    >,
    #[static_field]
    #[rename(name = "s_ConditionFlags")]
    pub s_condition_flags: crate::app::assettable::AssetTable_ConditionFlags,
    #[static_field]
    #[rename(name = "NullSound")]
    pub null_sound: crate::app::assettable::AssetTable_Sound,
    #[static_field]
    #[rename(name = "NullColor")]
    pub null_color: crate::unity_engine::color::Color,
    #[static_field]
    #[rename(name = "Shared")]
    pub shared: crate::app::assettable::AssetTable_Result,
}

#[cfg(feature = "app-assettable")]
#[::unity2::methods]
impl AssetTable {
    #[method(name = "DebuggerDisplay", args = 0)]
    pub fn debugger_display(self) -> ::unity2::Il2CppString;

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_PresetNames", args = 0)]
    pub fn get_preset_names() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "get_PresetName", args = 0)]
    pub fn get_preset_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PresetName", args = 1)]
    pub fn set_preset_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Mode", args = 0)]
    pub fn get_mode(self) -> crate::app::assettable::AssetTable_Modes;

    #[method(name = "set_Mode", args = 1)]
    pub fn set_mode(self, value: crate::app::assettable::AssetTable_Modes) -> ();

    #[method(name = "get_Conditions", args = 0)]
    pub fn get_conditions(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Conditions", args = 1)]
    pub fn set_conditions(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_BodyModel", args = 0)]
    pub fn get_body_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BodyModel", args = 1)]
    pub fn set_body_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DressModel", args = 0)]
    pub fn get_dress_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DressModel", args = 1)]
    pub fn set_dress_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HeadModel", args = 0)]
    pub fn get_head_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HeadModel", args = 1)]
    pub fn set_head_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HairModel", args = 0)]
    pub fn get_hair_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HairModel", args = 1)]
    pub fn set_hair_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RideModel", args = 0)]
    pub fn get_ride_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RideModel", args = 1)]
    pub fn set_ride_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RideDressModel", args = 0)]
    pub fn get_ride_dress_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RideDressModel", args = 1)]
    pub fn set_ride_dress_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LeftHand", args = 0)]
    pub fn get_left_hand(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LeftHand", args = 1)]
    pub fn set_left_hand(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RightHand", args = 0)]
    pub fn get_right_hand(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RightHand", args = 1)]
    pub fn set_right_hand(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Trail", args = 0)]
    pub fn get_trail(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Trail", args = 1)]
    pub fn set_trail(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Magic", args = 1)]
    pub fn set_magic(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BodyAnim", args = 0)]
    pub fn get_body_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BodyAnim", args = 1)]
    pub fn set_body_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RideAnim", args = 0)]
    pub fn get_ride_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RideAnim", args = 1)]
    pub fn set_ride_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_InfoAnim", args = 0)]
    pub fn get_info_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_InfoAnim", args = 1)]
    pub fn set_info_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_TalkAnim", args = 0)]
    pub fn get_talk_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_TalkAnim", args = 1)]
    pub fn set_talk_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DemoAnim", args = 0)]
    pub fn get_demo_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DemoAnim", args = 1)]
    pub fn set_demo_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HubAnim", args = 0)]
    pub fn get_hub_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HubAnim", args = 1)]
    pub fn set_hub_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HairR", args = 0)]
    pub fn get_hair_r(self) -> u8;

    #[method(name = "set_HairR", args = 1)]
    pub fn set_hair_r(self, value: u8) -> ();

    #[method(name = "get_HairG", args = 0)]
    pub fn get_hair_g(self) -> u8;

    #[method(name = "set_HairG", args = 1)]
    pub fn set_hair_g(self, value: u8) -> ();

    #[method(name = "get_HairB", args = 0)]
    pub fn get_hair_b(self) -> u8;

    #[method(name = "set_HairB", args = 1)]
    pub fn set_hair_b(self, value: u8) -> ();

    #[method(name = "get_GradR", args = 0)]
    pub fn get_grad_r(self) -> u8;

    #[method(name = "set_GradR", args = 1)]
    pub fn set_grad_r(self, value: u8) -> ();

    #[method(name = "get_GradG", args = 0)]
    pub fn get_grad_g(self) -> u8;

    #[method(name = "set_GradG", args = 1)]
    pub fn set_grad_g(self, value: u8) -> ();

    #[method(name = "get_GradB", args = 0)]
    pub fn get_grad_b(self) -> u8;

    #[method(name = "set_GradB", args = 1)]
    pub fn set_grad_b(self, value: u8) -> ();

    #[method(name = "get_SkinR", args = 0)]
    pub fn get_skin_r(self) -> u8;

    #[method(name = "set_SkinR", args = 1)]
    pub fn set_skin_r(self, value: u8) -> ();

    #[method(name = "get_SkinG", args = 0)]
    pub fn get_skin_g(self) -> u8;

    #[method(name = "set_SkinG", args = 1)]
    pub fn set_skin_g(self, value: u8) -> ();

    #[method(name = "get_SkinB", args = 0)]
    pub fn get_skin_b(self) -> u8;

    #[method(name = "set_SkinB", args = 1)]
    pub fn set_skin_b(self, value: u8) -> ();

    #[method(name = "get_ToonR", args = 0)]
    pub fn get_toon_r(self) -> u8;

    #[method(name = "set_ToonR", args = 1)]
    pub fn set_toon_r(self, value: u8) -> ();

    #[method(name = "get_ToonG", args = 0)]
    pub fn get_toon_g(self) -> u8;

    #[method(name = "set_ToonG", args = 1)]
    pub fn set_toon_g(self, value: u8) -> ();

    #[method(name = "get_ToonB", args = 0)]
    pub fn get_toon_b(self) -> u8;

    #[method(name = "set_ToonB", args = 1)]
    pub fn set_toon_b(self, value: u8) -> ();

    #[method(name = "get_MaskColor100R", args = 0)]
    pub fn get_mask_color100_r(self) -> u8;

    #[method(name = "set_MaskColor100R", args = 1)]
    pub fn set_mask_color100_r(self, value: u8) -> ();

    #[method(name = "get_MaskColor100G", args = 0)]
    pub fn get_mask_color100_g(self) -> u8;

    #[method(name = "set_MaskColor100G", args = 1)]
    pub fn set_mask_color100_g(self, value: u8) -> ();

    #[method(name = "get_MaskColor100B", args = 0)]
    pub fn get_mask_color100_b(self) -> u8;

    #[method(name = "set_MaskColor100B", args = 1)]
    pub fn set_mask_color100_b(self, value: u8) -> ();

    #[method(name = "get_MaskColor075R", args = 0)]
    pub fn get_mask_color075_r(self) -> u8;

    #[method(name = "set_MaskColor075R", args = 1)]
    pub fn set_mask_color075_r(self, value: u8) -> ();

    #[method(name = "get_MaskColor075G", args = 0)]
    pub fn get_mask_color075_g(self) -> u8;

    #[method(name = "set_MaskColor075G", args = 1)]
    pub fn set_mask_color075_g(self, value: u8) -> ();

    #[method(name = "get_MaskColor075B", args = 0)]
    pub fn get_mask_color075_b(self) -> u8;

    #[method(name = "set_MaskColor075B", args = 1)]
    pub fn set_mask_color075_b(self, value: u8) -> ();

    #[method(name = "get_MaskColor050R", args = 0)]
    pub fn get_mask_color050_r(self) -> u8;

    #[method(name = "set_MaskColor050R", args = 1)]
    pub fn set_mask_color050_r(self, value: u8) -> ();

    #[method(name = "get_MaskColor050G", args = 0)]
    pub fn get_mask_color050_g(self) -> u8;

    #[method(name = "set_MaskColor050G", args = 1)]
    pub fn set_mask_color050_g(self, value: u8) -> ();

    #[method(name = "get_MaskColor050B", args = 0)]
    pub fn get_mask_color050_b(self) -> u8;

    #[method(name = "set_MaskColor050B", args = 1)]
    pub fn set_mask_color050_b(self, value: u8) -> ();

    #[method(name = "get_MaskColor025R", args = 0)]
    pub fn get_mask_color025_r(self) -> u8;

    #[method(name = "set_MaskColor025R", args = 1)]
    pub fn set_mask_color025_r(self, value: u8) -> ();

    #[method(name = "get_MaskColor025G", args = 0)]
    pub fn get_mask_color025_g(self) -> u8;

    #[method(name = "set_MaskColor025G", args = 1)]
    pub fn set_mask_color025_g(self, value: u8) -> ();

    #[method(name = "get_MaskColor025B", args = 0)]
    pub fn get_mask_color025_b(self) -> u8;

    #[method(name = "set_MaskColor025B", args = 1)]
    pub fn set_mask_color025_b(self, value: u8) -> ();

    #[method(name = "get_Acc1", args = 0)]
    pub fn get_acc1(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc1", args = 1)]
    pub fn set_acc1(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Acc2", args = 0)]
    pub fn get_acc2(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc2", args = 1)]
    pub fn set_acc2(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Acc3", args = 0)]
    pub fn get_acc3(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc3", args = 1)]
    pub fn set_acc3(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Acc4", args = 0)]
    pub fn get_acc4(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc4", args = 1)]
    pub fn set_acc4(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Acc5", args = 0)]
    pub fn get_acc5(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc5", args = 1)]
    pub fn set_acc5(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Acc6", args = 0)]
    pub fn get_acc6(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc6", args = 1)]
    pub fn set_acc6(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Acc7", args = 0)]
    pub fn get_acc7(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc7", args = 1)]
    pub fn set_acc7(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Acc8", args = 0)]
    pub fn get_acc8(self) -> crate::app::assettable::AssetTable_Accessory;

    #[method(name = "set_Acc8", args = 1)]
    pub fn set_acc8(self, value: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "get_Accessories", args = 0)]
    pub fn get_accessories(self) -> crate::app::assettable::AssetTable_AccessoryList;

    #[method(name = "set_Accessories", args = 1)]
    pub fn set_accessories(self, value: crate::app::assettable::AssetTable_AccessoryList) -> ();

    #[method(name = "get_ScaleAll", args = 0)]
    pub fn get_scale_all(self) -> f32;

    #[method(name = "set_ScaleAll", args = 1)]
    pub fn set_scale_all(self, value: f32) -> ();

    #[method(name = "get_ScaleHead", args = 0)]
    pub fn get_scale_head(self) -> f32;

    #[method(name = "set_ScaleHead", args = 1)]
    pub fn set_scale_head(self, value: f32) -> ();

    #[method(name = "get_ScaleNeck", args = 0)]
    pub fn get_scale_neck(self) -> f32;

    #[method(name = "set_ScaleNeck", args = 1)]
    pub fn set_scale_neck(self, value: f32) -> ();

    #[method(name = "get_ScaleTorso", args = 0)]
    pub fn get_scale_torso(self) -> f32;

    #[method(name = "set_ScaleTorso", args = 1)]
    pub fn set_scale_torso(self, value: f32) -> ();

    #[method(name = "get_ScaleShoulders", args = 0)]
    pub fn get_scale_shoulders(self) -> f32;

    #[method(name = "set_ScaleShoulders", args = 1)]
    pub fn set_scale_shoulders(self, value: f32) -> ();

    #[method(name = "get_ScaleArms", args = 0)]
    pub fn get_scale_arms(self) -> f32;

    #[method(name = "set_ScaleArms", args = 1)]
    pub fn set_scale_arms(self, value: f32) -> ();

    #[method(name = "get_ScaleHands", args = 0)]
    pub fn get_scale_hands(self) -> f32;

    #[method(name = "set_ScaleHands", args = 1)]
    pub fn set_scale_hands(self, value: f32) -> ();

    #[method(name = "get_ScaleLegs", args = 0)]
    pub fn get_scale_legs(self) -> f32;

    #[method(name = "set_ScaleLegs", args = 1)]
    pub fn set_scale_legs(self, value: f32) -> ();

    #[method(name = "get_ScaleFeet", args = 0)]
    pub fn get_scale_feet(self) -> f32;

    #[method(name = "set_ScaleFeet", args = 1)]
    pub fn set_scale_feet(self, value: f32) -> ();

    #[method(name = "get_VolumeArms", args = 0)]
    pub fn get_volume_arms(self) -> f32;

    #[method(name = "set_VolumeArms", args = 1)]
    pub fn set_volume_arms(self, value: f32) -> ();

    #[method(name = "get_VolumeLegs", args = 0)]
    pub fn get_volume_legs(self) -> f32;

    #[method(name = "set_VolumeLegs", args = 1)]
    pub fn set_volume_legs(self, value: f32) -> ();

    #[method(name = "get_VolumeBust", args = 0)]
    pub fn get_volume_bust(self) -> f32;

    #[method(name = "set_VolumeBust", args = 1)]
    pub fn set_volume_bust(self, value: f32) -> ();

    #[method(name = "get_VolumeAbdomen", args = 0)]
    pub fn get_volume_abdomen(self) -> f32;

    #[method(name = "set_VolumeAbdomen", args = 1)]
    pub fn set_volume_abdomen(self, value: f32) -> ();

    #[method(name = "get_VolumeTorso", args = 0)]
    pub fn get_volume_torso(self) -> f32;

    #[method(name = "set_VolumeTorso", args = 1)]
    pub fn set_volume_torso(self, value: f32) -> ();

    #[method(name = "get_VolumeScaleArms", args = 0)]
    pub fn get_volume_scale_arms(self) -> f32;

    #[method(name = "set_VolumeScaleArms", args = 1)]
    pub fn set_volume_scale_arms(self, value: f32) -> ();

    #[method(name = "get_VolumeScaleLegs", args = 0)]
    pub fn get_volume_scale_legs(self) -> f32;

    #[method(name = "set_VolumeScaleLegs", args = 1)]
    pub fn set_volume_scale_legs(self, value: f32) -> ();

    #[method(name = "get_MapScaleAll", args = 0)]
    pub fn get_map_scale_all(self) -> f32;

    #[method(name = "set_MapScaleAll", args = 1)]
    pub fn set_map_scale_all(self, value: f32) -> ();

    #[method(name = "get_MapScaleHead", args = 0)]
    pub fn get_map_scale_head(self) -> f32;

    #[method(name = "set_MapScaleHead", args = 1)]
    pub fn set_map_scale_head(self, value: f32) -> ();

    #[method(name = "get_MapScaleWing", args = 0)]
    pub fn get_map_scale_wing(self) -> f32;

    #[method(name = "set_MapScaleWing", args = 1)]
    pub fn set_map_scale_wing(self, value: f32) -> ();

    #[method(name = "get_Voice", args = 0)]
    pub fn get_voice(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Voice", args = 1)]
    pub fn set_voice(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FootStep", args = 0)]
    pub fn get_foot_step(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FootStep", args = 1)]
    pub fn set_foot_step(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Material", args = 0)]
    pub fn get_material(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Material", args = 1)]
    pub fn set_material(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Comment", args = 0)]
    pub fn get_comment(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Comment", args = 1)]
    pub fn set_comment(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "OnCompletedEnd", args = 0)]
    pub fn on_completed_end(self) -> ();

    #[method(name = "get_ConditionNames", args = 0)]
    pub fn get_condition_names(
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "get_ConditionHits", args = 0)]
    pub fn get_condition_hits() -> crate::system::collections::generic::list_1::List_1<i32>;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "HasColor", args = 1)]
    pub fn has_color(color: crate::unity_engine::color::Color) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-assettable")]
impl AssetTable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetTable),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetTableMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_States.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AssetTable_States {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AssetTable_States {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AssetTable.States";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AssetTable_States {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AssetTable_States {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn engaging() -> Self {
        Self { value: 1 }
    }

    pub fn engage_attack() -> Self {
        Self { value: 2 }
    }

    pub fn engage_link_attack_main() -> Self {
        Self { value: 3 }
    }

    pub fn engage_link_attack_sub() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_AccessoryList.md")))]
#[::unity2::class(namespace = "App", name = "AssetTable.AccessoryList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: assettable :: AssetTable_Accessory >)]
pub struct AssetTable_AccessoryList {}

#[cfg(feature = "app-assettable")]
#[::unity2::methods]
impl AssetTable_AccessoryList {
    #[method(name = "TryAdd", args = 1)]
    pub fn try_add(self, accessory: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-assettable")]
impl AssetTable_AccessoryList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetTable_AccessoryList),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetTable_AccessoryListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_Accessory.md")))]
#[::unity2::class(namespace = "App", name = "AssetTable.Accessory")]
#[parent(crate::system::object::Object)]
pub struct AssetTable_Accessory {}

#[cfg(feature = "app-assettable")]
#[::unity2::methods]
impl AssetTable_Accessory {
    #[method(name = "get_Locator", args = 0)]
    pub fn get_locator(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Locator", args = 1)]
    pub fn set_locator(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Model", args = 0)]
    pub fn get_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Model", args = 1)]
    pub fn set_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Exist", args = 0)]
    pub fn get_exist(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-assettable")]
impl AssetTable_Accessory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetTable_Accessory),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetTable_AccessoryMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_Result.md")))]
#[::unity2::class(namespace = "App", name = "AssetTable.Result")]
#[parent(crate::system::object::Object)]
pub struct AssetTable_Result {
    #[rename(name = "m_InfoAnim")]
    pub m_info_anim: ::unity2::Il2CppString,
    #[rename(name = "m_TalkAnim")]
    pub m_talk_anim: ::unity2::Il2CppString,
    #[rename(name = "m_DemoAnim")]
    pub m_demo_anim: ::unity2::Il2CppString,
    #[rename(name = "m_HubAnim")]
    pub m_hub_anim: ::unity2::Il2CppString,
    #[rename(name = "m_ForceID")]
    pub m_force_id: ::unity2::Il2CppString,
    #[rename(name = "m_WeaponID")]
    pub m_weapon_id: ::unity2::Il2CppString,
    #[rename(name = "m_BodyAnims")]
    pub m_body_anims: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_Accessories")]
    pub m_accessories: crate::app::assettable::AssetTable_AccessoryList,
    #[rename(name = "m_AccessoryDictionary")]
    pub m_accessory_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::assettable::AssetTable_Accessory,
    >,
}

#[cfg(feature = "app-assettable")]
#[::unity2::methods]
impl AssetTable_Result {
    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Jid", args = 0)]
    pub fn get_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Jid", args = 1)]
    pub fn set_jid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BodyModel", args = 0)]
    pub fn get_body_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BodyModel", args = 1)]
    pub fn set_body_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DressModel", args = 0)]
    pub fn get_dress_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DressModel", args = 1)]
    pub fn set_dress_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HeadModel", args = 0)]
    pub fn get_head_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HeadModel", args = 1)]
    pub fn set_head_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HairModel", args = 0)]
    pub fn get_hair_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HairModel", args = 1)]
    pub fn set_hair_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RideModel", args = 0)]
    pub fn get_ride_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RideModel", args = 1)]
    pub fn set_ride_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RideDressModel", args = 0)]
    pub fn get_ride_dress_model(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RideDressModel", args = 1)]
    pub fn set_ride_dress_model(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LeftHand", args = 0)]
    pub fn get_left_hand(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LeftHand", args = 1)]
    pub fn set_left_hand(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RightHand", args = 0)]
    pub fn get_right_hand(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RightHand", args = 1)]
    pub fn set_right_hand(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Trail", args = 0)]
    pub fn get_trail(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Trail", args = 1)]
    pub fn set_trail(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Magic", args = 1)]
    pub fn set_magic(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BodyAnim", args = 0)]
    pub fn get_body_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BodyAnim", args = 1)]
    pub fn set_body_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RideAnim", args = 0)]
    pub fn get_ride_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RideAnim", args = 1)]
    pub fn set_ride_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HairColor", args = 0)]
    pub fn get_hair_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_HairColor", args = 1)]
    pub fn set_hair_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_GradColor", args = 0)]
    pub fn get_grad_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_GradColor", args = 1)]
    pub fn set_grad_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_SkinColor", args = 0)]
    pub fn get_skin_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_SkinColor", args = 1)]
    pub fn set_skin_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ToonShadowColor", args = 0)]
    pub fn get_toon_shadow_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_ToonShadowColor", args = 1)]
    pub fn set_toon_shadow_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_MaskColor100", args = 0)]
    pub fn get_mask_color100(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_MaskColor100", args = 1)]
    pub fn set_mask_color100(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_MaskColor075", args = 0)]
    pub fn get_mask_color075(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_MaskColor075", args = 1)]
    pub fn set_mask_color075(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_MaskColor050", args = 0)]
    pub fn get_mask_color050(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_MaskColor050", args = 1)]
    pub fn set_mask_color050(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_MaskColor025", args = 0)]
    pub fn get_mask_color025(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_MaskColor025", args = 1)]
    pub fn set_mask_color025(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ScaleAll", args = 0)]
    pub fn get_scale_all(self) -> f32;

    #[method(name = "set_ScaleAll", args = 1)]
    pub fn set_scale_all(self, value: f32) -> ();

    #[method(name = "get_ScaleHead", args = 0)]
    pub fn get_scale_head(self) -> f32;

    #[method(name = "set_ScaleHead", args = 1)]
    pub fn set_scale_head(self, value: f32) -> ();

    #[method(name = "get_ScaleNeck", args = 0)]
    pub fn get_scale_neck(self) -> f32;

    #[method(name = "set_ScaleNeck", args = 1)]
    pub fn set_scale_neck(self, value: f32) -> ();

    #[method(name = "get_ScaleTorso", args = 0)]
    pub fn get_scale_torso(self) -> f32;

    #[method(name = "set_ScaleTorso", args = 1)]
    pub fn set_scale_torso(self, value: f32) -> ();

    #[method(name = "get_ScaleShoulders", args = 0)]
    pub fn get_scale_shoulders(self) -> f32;

    #[method(name = "set_ScaleShoulders", args = 1)]
    pub fn set_scale_shoulders(self, value: f32) -> ();

    #[method(name = "get_ScaleArms", args = 0)]
    pub fn get_scale_arms(self) -> f32;

    #[method(name = "set_ScaleArms", args = 1)]
    pub fn set_scale_arms(self, value: f32) -> ();

    #[method(name = "get_ScaleHands", args = 0)]
    pub fn get_scale_hands(self) -> f32;

    #[method(name = "set_ScaleHands", args = 1)]
    pub fn set_scale_hands(self, value: f32) -> ();

    #[method(name = "get_ScaleLegs", args = 0)]
    pub fn get_scale_legs(self) -> f32;

    #[method(name = "set_ScaleLegs", args = 1)]
    pub fn set_scale_legs(self, value: f32) -> ();

    #[method(name = "get_ScaleFeet", args = 0)]
    pub fn get_scale_feet(self) -> f32;

    #[method(name = "set_ScaleFeet", args = 1)]
    pub fn set_scale_feet(self, value: f32) -> ();

    #[method(name = "get_VolumeArms", args = 0)]
    pub fn get_volume_arms(self) -> f32;

    #[method(name = "get_VolumeLegs", args = 0)]
    pub fn get_volume_legs(self) -> f32;

    #[method(name = "get_VolumeBust", args = 0)]
    pub fn get_volume_bust(self) -> f32;

    #[method(name = "set_VolumeBust", args = 1)]
    pub fn set_volume_bust(self, value: f32) -> ();

    #[method(name = "get_VolumeAbdomen", args = 0)]
    pub fn get_volume_abdomen(self) -> f32;

    #[method(name = "set_VolumeAbdomen", args = 1)]
    pub fn set_volume_abdomen(self, value: f32) -> ();

    #[method(name = "get_VolumeTorso", args = 0)]
    pub fn get_volume_torso(self) -> f32;

    #[method(name = "set_VolumeTorso", args = 1)]
    pub fn set_volume_torso(self, value: f32) -> ();

    #[method(name = "get_VolumeBaseArms", args = 0)]
    pub fn get_volume_base_arms(self) -> f32;

    #[method(name = "set_VolumeBaseArms", args = 1)]
    pub fn set_volume_base_arms(self, value: f32) -> ();

    #[method(name = "get_VolumeBaseLegs", args = 0)]
    pub fn get_volume_base_legs(self) -> f32;

    #[method(name = "set_VolumeBaseLegs", args = 1)]
    pub fn set_volume_base_legs(self, value: f32) -> ();

    #[method(name = "get_VolumeScaleArms", args = 0)]
    pub fn get_volume_scale_arms(self) -> f32;

    #[method(name = "set_VolumeScaleArms", args = 1)]
    pub fn set_volume_scale_arms(self, value: f32) -> ();

    #[method(name = "get_VolumeScaleLegs", args = 0)]
    pub fn get_volume_scale_legs(self) -> f32;

    #[method(name = "set_VolumeScaleLegs", args = 1)]
    pub fn set_volume_scale_legs(self, value: f32) -> ();

    #[method(name = "get_MapScaleAll", args = 0)]
    pub fn get_map_scale_all(self) -> f32;

    #[method(name = "set_MapScaleAll", args = 1)]
    pub fn set_map_scale_all(self, value: f32) -> ();

    #[method(name = "get_MapScaleHead", args = 0)]
    pub fn get_map_scale_head(self) -> f32;

    #[method(name = "set_MapScaleHead", args = 1)]
    pub fn set_map_scale_head(self, value: f32) -> ();

    #[method(name = "get_MapScaleWing", args = 0)]
    pub fn get_map_scale_wing(self) -> f32;

    #[method(name = "set_MapScaleWing", args = 1)]
    pub fn set_map_scale_wing(self, value: f32) -> ();

    #[method(name = "get_BodyAnims", args = 0)]
    pub fn get_body_anims(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "get_Accessories", args = 0)]
    pub fn get_accessories(self) -> crate::app::assettable::AssetTable_AccessoryList;

    #[method(name = "get_Sound", args = 0)]
    pub fn get_sound(self) -> crate::app::assettable::AssetTable_Sound;

    #[method(name = "set_Sound", args = 1)]
    pub fn set_sound(self, value: crate::app::assettable::AssetTable_Sound) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Setup", args = 4)]
    pub fn setup(
        self,
        mode: crate::app::assettable::AssetTable_Modes,
        unit: crate::app::unit::Unit,
        equipped: crate::app::itemdata::ItemData,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "Setup", args = 4)]
    pub fn setup_2(
        self,
        mode: crate::app::assettable::AssetTable_Modes,
        god_data: crate::app::goddata::GodData,
        is_darkness: bool,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "Setup", args = 1)]
    pub fn setup_3(
        self,
        data: crate::app::assettable::AssetTable,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "Setup", args = 3)]
    pub fn setup_4(
        self,
        mode: crate::app::assettable::AssetTable_Modes,
        person: crate::app::persondata::PersonData,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "Setup", args = 5)]
    pub fn setup_5(
        self,
        mode: crate::app::assettable::AssetTable_Modes,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        equipped: crate::app::itemdata::ItemData,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "Setup", args = 9)]
    pub fn setup_6(
        self,
        mode: crate::app::assettable::AssetTable_Modes,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        god: crate::app::goddata::GodData,
        equipped: crate::app::itemdata::ItemData,
        force: crate::app::force::Force_Type,
        state: crate::app::assettable::AssetTable_States,
        is_darkness: bool,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "Commit", args = 1)]
    pub fn commit(self, mode: crate::app::assettable::AssetTable_Modes) -> ();

    #[method(name = "Commit", args = 4)]
    pub fn commit_2(
        self,
        mode: crate::app::assettable::AssetTable_Modes,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        equipped: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "Commit", args = 2)]
    pub fn commit_3(
        self,
        mode: crate::app::assettable::AssetTable_Modes,
        god_data: crate::app::goddata::GodData,
    ) -> ();

    #[method(name = "Commit", args = 1)]
    pub fn commit_4(self, data: crate::app::assettable::AssetTable) -> ();

    #[method(name = "Commit", args = 2)]
    pub fn commit_5(
        self,
        value: ::unity2::Il2CppString,
        replace: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Commit", args = 2)]
    pub fn commit_6(self, value: f32, replace: f32) -> f32;

    #[method(name = "Commit", args = 2)]
    pub fn commit_7(
        self,
        color: crate::unity_engine::color::Color,
        replace: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "Commit", args = 1)]
    pub fn commit_8(self, accessory: crate::app::assettable::AssetTable_Accessory) -> ();

    #[method(name = "Replace", args = 1)]
    pub fn replace(self, mode: crate::app::assettable::AssetTable_Modes) -> ();

    #[method(name = "Replace", args = 3)]
    pub fn replace_2(
        self,
        name: ::unity2::Il2CppString,
        mode: crate::app::assettable::AssetTable_Modes,
        rename: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ReplaceScale", args = 2)]
    pub fn replace_scale(self, value: f32, replace: f32) -> f32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "HideStirrup", args = 0)]
    pub fn hide_stirrup(self) -> ();

    #[method(name = "ReplaceForTalk", args = 0)]
    pub fn replace_for_talk(self) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "ReplaceForInfo", args = 0)]
    pub fn replace_for_info(self) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "ReplaceForDemo", args = 0)]
    pub fn replace_for_demo(self) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "ReplaceForHub", args = 0)]
    pub fn replace_for_hub(self) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "TryDump", args = 0)]
    pub fn try_dump(self) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForTalk", args = 1)]
    pub fn get_for_talk(pid: ::unity2::Il2CppString) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForTalk", args = 1)]
    pub fn get_for_talk_2(
        unit: crate::app::unit::Unit,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForDemo", args = 3)]
    pub fn get_for_demo(
        pid: ::unity2::Il2CppString,
        is_default: bool,
        is_plain: bool,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetKizunaCondition", args = 1)]
    pub fn get_kizuna_condition(
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetForKizuna", args = 2)]
    pub fn get_for_kizuna(
        pid: ::unity2::Il2CppString,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetHubCondition", args = 1)]
    pub fn get_hub_condition(
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetForHub", args = 2)]
    pub fn get_for_hub(
        pid: ::unity2::Il2CppString,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForHubDirect", args = 2)]
    pub fn get_for_hub_direct(
        pid: ::unity2::Il2CppString,
        condisions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForAccessory", args = 1)]
    pub fn get_for_accessory(
        unit: crate::app::unit::Unit,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromUnit", args = 3)]
    pub fn get_from_unit(
        mode: crate::app::assettable::AssetTable_Modes,
        unit: crate::app::unit::Unit,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromUnit", args = 4)]
    pub fn get_from_unit_2(
        mode: crate::app::assettable::AssetTable_Modes,
        unit: crate::app::unit::Unit,
        equipped: crate::app::itemdata::ItemData,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForUnitInfo", args = 1)]
    pub fn get_for_unit_info(
        unit: crate::app::unit::Unit,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForUnitInfo", args = 2)]
    pub fn get_for_unit_info_2(
        unit: crate::app::unit::Unit,
        equipped: crate::app::itemdata::ItemData,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForUnitInfo", args = 2)]
    pub fn get_for_unit_info_3(
        god_data: crate::app::goddata::GodData,
        is_darkness: bool,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForUnitInfo", args = 1)]
    pub fn get_for_unit_info_4(
        god_unit: crate::app::godunit::GodUnit,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForUnitHub", args = 1)]
    pub fn get_for_unit_hub(
        unit: crate::app::unit::Unit,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForTalk", args = 1)]
    pub fn get_for_talk_3(
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForTalk", args = 1)]
    pub fn get_for_talk_4(
        god_unit: crate::app::godunit::GodUnit,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForDemo", args = 1)]
    pub fn get_for_demo_2(
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForHub", args = 1)]
    pub fn get_for_hub_2(
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromGod", args = 3)]
    pub fn get_from_god(
        mode: crate::app::assettable::AssetTable_Modes,
        god_unit: crate::app::godunit::GodUnit,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromGod", args = 4)]
    pub fn get_from_god_2(
        mode: crate::app::assettable::AssetTable_Modes,
        god_data: crate::app::goddata::GodData,
        is_darkness: bool,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromItem", args = 2)]
    pub fn get_from_item(
        mode: crate::app::assettable::AssetTable_Modes,
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromShop", args = 2)]
    pub fn get_from_shop(
        mode: crate::app::assettable::AssetTable_Modes,
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromPreset", args = 1)]
    pub fn get_from_preset(
        name: ::unity2::Il2CppString,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetEditConditions", args = 1)]
    pub fn get_edit_conditions(
        edit: crate::app::unitedit::UnitEdit,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetForRelay", args = 3)]
    pub fn get_for_relay(
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        edit: crate::app::unitedit::UnitEdit,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetForEditor", args = 7)]
    pub fn get_for_editor(
        mode: crate::app::assettable::AssetTable_Modes,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        iid: ::unity2::Il2CppString,
        gid: ::unity2::Il2CppString,
        force: crate::app::force::Force_Type,
        state: crate::app::assettable::AssetTable_States,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "GetFromPID", args = 3)]
    pub fn get_from_pid(
        mode: crate::app::assettable::AssetTable_Modes,
        pid: ::unity2::Il2CppString,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::app::assettable::AssetTable_Result;
}

#[cfg(feature = "app-assettable")]
impl AssetTable_Result {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetTable_Result),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetTable_ResultMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/assettable/AssetTable_Modes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AssetTable_Modes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AssetTable_Modes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AssetTable.Modes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AssetTable_Modes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AssetTable_Modes {
    pub fn common() -> Self {
        Self { value: 0 }
    }

    pub fn onmap() -> Self {
        Self { value: 1 }
    }

    pub fn combat() -> Self {
        Self { value: 2 }
    }

    pub fn gmap() -> Self {
        Self { value: 3 }
    }

    pub fn num() -> Self {
        Self { value: 4 }
    }
}
