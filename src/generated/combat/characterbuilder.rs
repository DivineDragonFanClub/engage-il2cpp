
use crate::combat::characterassetform::CharacterAssetForm;
use crate::combat::characterassetform::ICharacterAssetForm;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterbuilder/CharacterBuilder.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterBuilder")]
#[parent(crate::combat::characterassetform::CharacterAssetForm)]
pub struct CharacterBuilder {
    #[rename(name = "dressUt")]
    pub dress_ut: crate::combat::dressutility::DressUtility,
    #[rename(name = "m_CachedRenderers")]
    pub m_cached_renderers: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::renderer::Renderer,
    >,
    #[rename(name = "m_CachedShadowRenderers")]
    pub m_cached_shadow_renderers: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::renderer::Renderer,
    >,
    #[rename(name = "m_bVisible")]
    pub m_b_visible: bool,
}

#[cfg(feature = "combat-characterbuilder")]
#[::unity2::methods]
impl CharacterBuilder {
    #[method(name = "get_Body", args = 0)]
    pub fn get_body(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Body", args = 1)]
    pub fn set_body(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Dress", args = 0)]
    pub fn get_dress(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Dress", args = 1)]
    pub fn set_dress(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Head", args = 0)]
    pub fn get_head(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Head", args = 1)]
    pub fn set_head(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Hair", args = 0)]
    pub fn get_hair(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Hair", args = 1)]
    pub fn set_hair(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Left", args = 0)]
    pub fn get_left(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Left", args = 1)]
    pub fn set_left(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Right", args = 0)]
    pub fn get_right(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Right", args = 1)]
    pub fn set_right(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Ride", args = 0)]
    pub fn get_ride(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Ride", args = 1)]
    pub fn set_ride(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_Ridress", args = 0)]
    pub fn get_ridress(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Ridress", args = 1)]
    pub fn set_ridress(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_BodyAOC", args = 0)]
    pub fn get_body_aoc(
        self,
    ) -> crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController;

    #[method(name = "set_BodyAOC", args = 1)]
    pub fn set_body_aoc(
        self,
        value: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
    ) -> ();

    #[method(name = "get_RideAOC", args = 0)]
    pub fn get_ride_aoc(
        self,
    ) -> crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController;

    #[method(name = "set_RideAOC", args = 1)]
    pub fn set_ride_aoc(
        self,
        value: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
    ) -> ();

    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> crate::combat::launchbehaviour::LaunchBehaviour;

    #[method(name = "set_Magic", args = 1)]
    pub fn set_magic(self, value: crate::combat::launchbehaviour::LaunchBehaviour) -> ();

    #[method(name = "get_PrivateEffectCatalog", args = 0)]
    pub fn get_private_effect_catalog(
        self,
    ) -> crate::combat::privateeffectcatalog::PrivateEffectCatalog;

    #[method(name = "set_PrivateEffectCatalog", args = 1)]
    pub fn set_private_effect_catalog(
        self,
        value: crate::combat::privateeffectcatalog::PrivateEffectCatalog,
    ) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "get_MainWeapon", args = 0)]
    pub fn get_main_weapon(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_MainHand", args = 0)]
    pub fn get_main_hand(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_WeaponKind", args = 0)]
    pub fn get_weapon_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "set_WeaponKind", args = 1)]
    pub fn set_weapon_kind(self, value: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "get_WeaponName", args = 0)]
    pub fn get_weapon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_WeaponName", args = 1)]
    pub fn set_weapon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsBrawl", args = 0)]
    pub fn get_is_brawl(self) -> bool;

    #[method(name = "get_IsFlying", args = 0)]
    pub fn get_is_flying(self) -> bool;

    #[method(name = "GetTall", args = 0)]
    pub fn get_tall(self) -> f32;

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = "set_IsVisible", args = 1)]
    pub fn set_is_visible(self, value: bool) -> ();

    #[method(name = "CoBuildHierarchy", args = 0)]
    pub fn co_build_hierarchy(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "BuildHierarchy", args = 0)]
    pub fn build_hierarchy(self) -> ();

    #[method(name = "BuildBaseHierarchy", args = 0)]
    pub fn build_base_hierarchy(self) -> ();

    #[method(name = "SetupAnimatorWithDAOC", args = 0)]
    pub fn setup_animator_with_daoc(self) -> ();

    #[method(name = "SetupAnimatorWithAOC", args = 0)]
    pub fn setup_animator_with_aoc(self) -> ();

    #[method(name = "AttachHeadHairAndWeapons", args = 0)]
    pub fn attach_head_hair_and_weapons(self) -> ();

    #[method(name = "UnloadD", args = 1)]
    pub fn unload_d(self, chr: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "Others", args = 0)]
    pub fn others(self) -> ();

    #[method(name = "GetGO", args = 1)]
    pub fn get_go(
        self,
        asset: crate::combat::characterasset::CharacterAsset,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "RepairAOC", args = 0)]
    pub fn repair_aoc(self) -> ();

    #[method(name = "AttachRidress", args = 0)]
    pub fn attach_ridress(self) -> ();

    #[method(name = "AttachDress", args = 0)]
    pub fn attach_dress(self) -> ();

    #[method(name = "BuildProportion", args = 0)]
    pub fn build_proportion(self) -> ();

    #[method(name = "AttachAccessory", args = 1)]
    pub fn attach_accessory(self, i: i32) -> ();

    #[method(name = "Validate", args = 1)]
    pub fn validate(
        aoc: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
    ) -> ();

    #[method(name = "ReplaceGOName", args = 3)]
    pub fn replace_go_name(
        go: crate::unity_engine::gameobject::GameObject,
        src: ::unity2::Il2CppString,
        dst: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetupBaseAnimator", args = 4)]
    pub fn setup_base_animator(
        src_go: crate::unity_engine::gameobject::GameObject,
        dst_go: crate::unity_engine::gameobject::GameObject,
        aoc: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
        need_listener: bool,
    ) -> ();

    #[method(name = "SetupRiderAnimator", args = 3)]
    pub fn setup_rider_animator(
        go: crate::unity_engine::gameobject::GameObject,
        aoc: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
        need_listener: bool,
    ) -> ();

    #[method(name = "SetVisibleForced", args = 1)]
    pub fn set_visible_forced(self, value: bool) -> ();

    #[method(name = "MakeCachedRenderersList", args = 0)]
    pub fn make_cached_renderers_list(self) -> ();

    #[method(name = "InvalidateCachedRenderersList", args = 0)]
    pub fn invalidate_cached_renderers_list(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterbuilder")]
impl CharacterBuilder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterBuilder),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterBuilderMethods>::ctor(this);
        this
    }
}
