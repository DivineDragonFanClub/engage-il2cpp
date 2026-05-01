
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterappearance/CharacterAppearance.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterAppearance")]
#[parent(crate::system::object::Object)]
pub struct CharacterAppearance {
    #[static_field]
    #[rename(name = "s_StencilValue")]
    pub s_stencil_value: f32,
    #[rename(name = "assets")]
    pub assets: ::unity2::Array<crate::combat::characterasset::CharacterAsset>,
    #[rename(name = "AnimsetNames")]
    pub animset_names: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "accTargets")]
    pub acc_targets: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "MaskColor100")]
    pub mask_color100: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor075")]
    pub mask_color075: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor050")]
    pub mask_color050: crate::unity_engine::color::Color,
    #[rename(name = "MaskColor025")]
    pub mask_color025: crate::unity_engine::color::Color,
    #[rename(name = "SkinColor")]
    pub skin_color: crate::unity_engine::color::Color,
    #[rename(name = "GradColor")]
    pub grad_color: crate::unity_engine::color::Color,
    #[rename(name = "HairColor")]
    pub hair_color: crate::unity_engine::color::Color,
    #[rename(name = "ToonShadowColor")]
    pub toon_shadow_color: crate::unity_engine::color::Color,
    #[rename(name = "Sound")]
    pub sound: crate::app::assettable::AssetTable_Sound,
    #[rename(name = "Proportion")]
    pub proportion: crate::combat::proportionparameters::ProportionParameters,
    #[rename(name = "m_InstancedMaterials")]
    pub m_instanced_materials: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::material::Material,
    >,
    #[rename(name = "m_HolderLoadSimultaneous")]
    pub m_holder_load_simultaneous: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallWalk")]
    pub m_chara_tall_walk: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallHorse")]
    pub m_chara_tall_horse: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallWolf")]
    pub m_chara_tall_wolf: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallPegasus")]
    pub m_chara_tall_pegasus: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallDragon")]
    pub m_chara_tall_dragon: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallGriffon")]
    pub m_chara_tall_griffon: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallEngage")]
    pub m_chara_tall_engage: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallSombronHuman")]
    pub m_chara_tall_sombron_human: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallSombronDragon")]
    pub m_chara_tall_sombron_dragon: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallEngageTiki")]
    pub m_chara_tall_engage_tiki: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallPhantomWolf")]
    pub m_chara_tall_phantom_wolf: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallPhantomDragon")]
    pub m_chara_tall_phantom_dragon: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_CharaTallCorruptDragon")]
    pub m_chara_tall_corrupt_dragon: crate::app::gameparam::GameParam_Holder,
    #[static_field]
    #[rename(name = "hash_BaseColor")]
    pub hash_base_color: i32,
    #[static_field]
    #[rename(name = "hash_StencilGroup")]
    pub hash_stencil_group: i32,
    #[static_field]
    #[rename(name = "hash_GradationColor")]
    pub hash_gradation_color: i32,
    #[static_field]
    #[rename(name = "hash_ToonShadowColor")]
    pub hash_toon_shadow_color: i32,
    #[static_field]
    #[rename(name = "hash_MaskColor100")]
    pub hash_mask_color100: i32,
    #[static_field]
    #[rename(name = "hash_MaskColor075")]
    pub hash_mask_color075: i32,
    #[static_field]
    #[rename(name = "hash_MaskColor050")]
    pub hash_mask_color050: i32,
    #[static_field]
    #[rename(name = "hash_MaskColor025")]
    pub hash_mask_color025: i32,
    #[static_field]
    #[rename(name = "Conditions")]
    pub conditions: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "combat-characterappearance")]
#[::unity2::methods]
impl CharacterAppearance {
    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(a: crate::combat::characterappearance::CharacterAppearance) -> bool;

    #[method(name = "get_AnimSet", args = 0)]
    pub fn get_anim_set(self) -> crate::combat::characteranimset::CharacterAnimset;

    #[method(name = "set_AnimSet", args = 1)]
    pub fn set_anim_set(self, value: crate::combat::characteranimset::CharacterAnimset) -> ();

    #[method(name = "get_WeaponStyle", args = 0)]
    pub fn get_weapon_style(self) -> crate::combat::weaponstyle::WeaponStyle;

    #[method(name = "set_WeaponStyle", args = 1)]
    pub fn set_weapon_style(self, value: crate::combat::weaponstyle::WeaponStyle) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::combat::characterasset::CharacterAsset;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item_2(
        self,
        r#type: crate::combat::assettype::AssetType,
    ) -> crate::combat::characterasset::CharacterAsset;

    #[method(name = "get_AccTargets", args = 0)]
    pub fn get_acc_targets(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "get_Race", args = 0)]
    pub fn get_race(self) -> crate::combat::animname::AnimName_Race;

    #[method(name = "get_IsRide", args = 0)]
    pub fn get_is_ride(self) -> bool;

    #[method(name = "get_IsFlying", args = 0)]
    pub fn get_is_flying(self) -> bool;

    #[method(name = "get_IsCorruptAnimal", args = 0)]
    pub fn get_is_corrupt_animal(self) -> bool;

    #[method(name = "get_IsBigDragon", args = 0)]
    pub fn get_is_big_dragon(self) -> bool;

    #[method(name = "get_IsWyrm", args = 0)]
    pub fn get_is_wyrm(self) -> bool;

    #[method(name = "get_IsBrawl", args = 0)]
    pub fn get_is_brawl(self) -> bool;

    #[method(name = "get_HasRod", args = 0)]
    pub fn get_has_rod(self) -> bool;

    #[method(name = "get_MainHandModel", args = 0)]
    pub fn get_main_hand_model(self) -> crate::combat::characterasset::CharacterAsset;

    #[method(name = "get_IsHighClass", args = 0)]
    pub fn get_is_high_class(self) -> bool;

    #[method(name = "get_Is異形", args = 0)]
    pub fn get_is__(self) -> bool;

    #[method(name = "get_IsLastBoss", args = 0)]
    pub fn get_is_last_boss(self) -> bool;

    #[method(name = "get_BackwardCancelPosition", args = 0)]
    pub fn get_backward_cancel_position(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        rhs: crate::combat::characterappearance::CharacterAppearance,
        weapon_style: crate::combat::weaponstyle::WeaponStyle,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "SetDefaultName", args = 0)]
    pub fn set_default_name(self) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ::unity2::Il2CppString;

    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async(self) -> ();

    #[method(name = "LoadAnimSetAsync", args = 0)]
    pub fn load_anim_set_async(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "ReplaceBodyAnimForEditor", args = 1)]
    pub fn replace_body_anim_for_editor(
        self,
        a: crate::combat::characterasset::CharacterAsset,
    ) -> ();

    #[method(name = "CreateAnimSetForViewerPreview", args = 1)]
    pub fn create_anim_set_for_viewer_preview(self, map_distance: i32) -> ();

    #[method(name = "GetTall", args = 0)]
    pub fn get_tall(self) -> f32;

    #[method(name = "GetBodySize", args = 0)]
    pub fn get_body_size(self) -> f32;

    #[method(name = "ModifyColors", args = 1)]
    pub fn modify_colors(self, go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "HasMaterialToModify", args = 1)]
    pub fn has_material_to_modify(
        self,
        mats: ::unity2::Array<crate::unity_engine::material::Material>,
    ) -> bool;

    #[method(name = "DestroyInstancedMaterials", args = 0)]
    pub fn destroy_instanced_materials(self) -> ();

    #[method(name = "Verification", args = 1)]
    pub fn verification(self, title: ::unity2::Il2CppString) -> ();

    #[method(name = "SerializeToString", args = 0)]
    pub fn serialize_to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "DeserializeFromString", args = 1)]
    pub fn deserialize_from_string(self, contents: ::unity2::Il2CppString) -> ();

    #[method(name = "CreateFromGameStatus", args = 3)]
    pub fn create_from_game_status(
        game_status: crate::combat::charactergamestatus::CharacterGameStatus,
        map_distance: i32,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "CreateForSound", args = 1)]
    pub fn create_for_sound(
        unit: crate::app::unit::Unit,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "CreateFromGodUnit", args = 2)]
    pub fn create_from_god_unit(
        gunit: crate::app::godunit::GodUnit,
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "CreateFromPreset", args = 1)]
    pub fn create_from_preset(
        name: ::unity2::Il2CppString,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "GetConstions", args = 1)]
    pub fn get_constions(
        conditions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "CreateFromResult", args = 2)]
    pub fn create_from_result(
        result: crate::app::assettable::AssetTable_Result,
        map_distance_1or2: i32,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "CalcWeaponStyle", args = 1)]
    pub fn calc_weapon_style(self, map_distance: i32) -> crate::combat::weaponstyle::WeaponStyle;

    #[method(name = "IsSameCharacter", args = 2)]
    pub fn is_same_character(
        l: crate::combat::characterappearance::CharacterAppearance,
        r: crate::combat::characterappearance::CharacterAppearance,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "combat-characterappearance")]
impl CharacterAppearance {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterAppearance),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterAppearanceMethods>::ctor(this);
        this
    }

    pub fn new_2(
        rhs: crate::combat::characterappearance::CharacterAppearance,
        weapon_style: crate::combat::weaponstyle::WeaponStyle,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterAppearance),
                ::core::stringify!(new_2),
            )
        });
        <Self as ICharacterAppearanceMethods>::ctor_2(this, rhs, weapon_style);
        this
    }
}
