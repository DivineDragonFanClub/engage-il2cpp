
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ring_cleaning/effectcontroller/EffectController_AutoDelete.md")))]
#[::unity2::class(namespace = "App.RingCleaning", name = "EffectController.AutoDelete")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct EffectController_AutoDelete {}

#[cfg(feature = "app-ring_cleaning-effectcontroller")]
#[::unity2::methods]
impl EffectController_AutoDelete {
    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ring_cleaning-effectcontroller")]
impl EffectController_AutoDelete {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EffectController_AutoDelete),
                ::core::stringify!(new),
            )
        });
        <Self as IEffectController_AutoDeleteMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ring_cleaning/effectcontroller/EffectController.md")))]
#[::unity2::class(namespace = "App.RingCleaning", name = "EffectController")]
#[parent(crate::system::object::Object)]
pub struct EffectController {
    #[static_field]
    #[rename(name = "GlitterEffectPrefab")]
    pub glitter_effect_prefab: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "GlitterStrongEffectPrefab")]
    pub glitter_strong_effect_prefab: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "GlitterWeakNotHit")]
    pub glitter_weak_not_hit: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "GlitterWeakNear")]
    pub glitter_weak_near: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "GlitterStrongA")]
    pub glitter_strong_a: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "GlitterStrongB")]
    pub glitter_strong_b: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "NoteEffectA")]
    pub note_effect_a: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "NoteEffectB")]
    pub note_effect_b: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "HitEffect2D")]
    pub hit_effect2_d: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "RubEffect")]
    pub rub_effect: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_EffectRoot")]
    pub s_effect_root: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "s_Rub")]
    pub s_rub: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-ring_cleaning-effectcontroller")]
#[::unity2::methods]
impl EffectController {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "CreateGlitterEffect", args = 2)]
    pub fn create_glitter_effect(
        strength: crate::app::ringcleaningsequence::RingCleaningSequence_Strength,
        hit_result: crate::app::ringcleaningsequence::RingCleaningSequence_HitResult,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "PlayHitScreen", args = 0)]
    pub fn play_hit_screen() -> ();

    #[method(name = "PlayRubEffect", args = 0)]
    pub fn play_rub_effect() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_NoteEffect", args = 0)]
    pub fn get_note_effect() -> ::unity2::Il2CppString;

    #[method(name = "get_NoteEffectBig", args = 0)]
    pub fn get_note_effect_big() -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ring_cleaning-effectcontroller")]
impl EffectController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EffectController),
                ::core::stringify!(new),
            )
        });
        <Self as IEffectControllerMethods>::ctor(this);
        this
    }
}
