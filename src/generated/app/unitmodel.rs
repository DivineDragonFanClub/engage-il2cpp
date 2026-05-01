
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitmodel/UnitModel_DirtyFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitModel_DirtyFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitModel_DirtyFlags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitModel.DirtyFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitModel_DirtyFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitModel_DirtyFlags {
    pub fn speed() -> Self {
        Self { value: 1 }
    }

    pub fn animation() -> Self {
        Self { value: 2 }
    }

    pub fn renderer() -> Self {
        Self { value: 4 }
    }

    pub fn shine() -> Self {
        Self { value: 8 }
    }

    pub fn alpha() -> Self {
        Self { value: 16 }
    }

    pub fn bright() -> Self {
        Self { value: 32 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitmodel/UnitModel_ColorFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitModel_ColorFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitModel_ColorFlags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitModel.ColorFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitModel_ColorFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitModel_ColorFlags {
    pub fn fixed() -> Self {
        Self { value: 1 }
    }

    pub fn danager() -> Self {
        Self { value: 2 }
    }

    pub fn enemy() -> Self {
        Self { value: 4 }
    }

    pub fn ally() -> Self {
        Self { value: 8 }
    }

    pub fn dirty() -> Self {
        Self { value: 16 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitmodel/UnitModel_LoadMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitModel_LoadMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitModel_LoadMode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitModel.LoadMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitModel_LoadMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitModel_LoadMode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn loading() -> Self {
        Self { value: 1 }
    }

    pub fn done() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitmodel/UnitModel.md")))]
#[::unity2::class(namespace = "App", name = "UnitModel")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct UnitModel {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[rename(name = "m_HandleA")]
    pub m_handle_a: crate::app::unitmodel::UnitModel_ResourceHandle,
    #[rename(name = "m_HandleB")]
    pub m_handle_b: crate::app::unitmodel::UnitModel_ResourceHandle,
    #[rename(name = "m_Handle")]
    pub m_handle: crate::app::unitmodel::UnitModel_ResourceHandle,
    #[rename(name = "m_Materials")]
    pub m_materials: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::material::Material,
    >,
    #[rename(name = "m_Renderers")]
    pub m_renderers: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::renderer::Renderer,
    >,
    #[rename(name = "m_HairColor")]
    pub m_hair_color: crate::unity_engine::color::Color,
    #[rename(name = "m_SkinColor")]
    pub m_skin_color: crate::unity_engine::color::Color,
    #[rename(name = "m_MaskColor100")]
    pub m_mask_color100: crate::unity_engine::color::Color,
    #[rename(name = "m_MaskColor075")]
    pub m_mask_color075: crate::unity_engine::color::Color,
    #[rename(name = "m_MaskColor050")]
    pub m_mask_color050: crate::unity_engine::color::Color,
    #[rename(name = "m_MaskColor025")]
    pub m_mask_color025: crate::unity_engine::color::Color,
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RootScale")]
    pub m_root_scale: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_WingScale")]
    pub m_wing_scale: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_LoadMode")]
    pub m_load_mode: crate::app::unitmodel::UnitModel_LoadMode,
    #[rename(name = "m_Joint")]
    pub m_joint: crate::combat::characterjoint::CharacterJoint,
    #[rename(name = "m_ProportionParameters")]
    pub m_proportion_parameters: crate::combat::proportionparameters::ProportionParameters,
    #[rename(name = "m_LeftHandObject")]
    pub m_left_hand_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RightHandObject")]
    pub m_right_hand_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkinQuality")]
    pub m_skin_quality: crate::unity_engine::skinquality::SkinQuality,
    #[rename(name = "m_SkinnedMeshRenderers")]
    pub m_skinned_mesh_renderers:
        ::unity2::Array<crate::unity_engine::skinnedmeshrenderer::SkinnedMeshRenderer>,
    #[rename(name = "m_Animators")]
    pub m_animators: ::unity2::Array<crate::unity_engine::animator::Animator>,
    #[rename(name = "m_PlayAnim")]
    pub m_play_anim: crate::app::unitanim::UnitAnim_Types,
    #[rename(name = "m_IdleAnim")]
    pub m_idle_anim: crate::app::unitanim::UnitAnim_Types,
    #[rename(name = "m_PlayTime")]
    pub m_play_time: crate::app::unitanim::UnitAnim_Times,
    #[rename(name = "m_EquipItem")]
    pub m_equip_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_ForceItem")]
    pub m_force_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_ModelHash")]
    pub m_model_hash: i32,
    #[rename(name = "m_EquipHash")]
    pub m_equip_hash: i32,
    #[rename(name = "m_Speed")]
    pub m_speed: f32,
    #[rename(name = "m_Alpha")]
    pub m_alpha: f32,
    #[rename(name = "m_MapAlpha")]
    pub m_map_alpha: f32,
    #[rename(name = "m_IsVisible")]
    pub m_is_visible: bool,
    #[rename(name = "m_ColorFlags")]
    pub m_color_flags: crate::app::unitmodel::UnitModel_ColorFlags,
    #[rename(name = "m_DirtyFlags")]
    pub m_dirty_flags: crate::app::unitmodel::UnitModel_DirtyFlags,
    #[rename(name = "m_InterpolatorShine")]
    pub m_interpolator_shine: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_InterpolatorFader")]
    pub m_interpolator_fader: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_InterpolatorGoder")]
    pub m_interpolator_goder: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_Head")]
    pub m_head: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Body")]
    pub m_body: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Ride")]
    pub m_ride: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WingL")]
    pub m_wing_l: crate::unity_engine::transform::Transform,
    #[rename(name = "m_WingR")]
    pub m_wing_r: crate::unity_engine::transform::Transform,
    #[rename(name = "m_Trans")]
    pub m_trans: crate::unity_engine::transform::Transform,
    #[rename(name = "m_Sound")]
    pub m_sound: crate::app::assettable::AssetTable_Sound,
}

#[cfg(feature = "app-unitmodel")]
#[::unity2::methods]
impl UnitModel {
    #[method(name = "get_ModelHash", args = 0)]
    pub fn get_model_hash(self) -> i32;

    #[method(name = "get_EquipHash", args = 0)]
    pub fn get_equip_hash(self) -> i32;

    #[method(name = "get_ProportionParameters", args = 0)]
    pub fn get_proportion_parameters(
        self,
    ) -> crate::combat::proportionparameters::ProportionParameters;

    #[method(name = "Create", args = 4)]
    pub fn create(
        name: ::unity2::Il2CppString,
        unit: crate::app::unit::Unit,
        god_unit: crate::app::godunit::GodUnit,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::app::unitmodel::UnitModel;

    #[method(name = "get_Sound", args = 0)]
    pub fn get_sound(self) -> crate::app::assettable::AssetTable_Sound;

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = "get_Head", args = 0)]
    pub fn get_head(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_Body", args = 0)]
    pub fn get_body(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_Ride", args = 0)]
    pub fn get_ride(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FindObject", args = 1)]
    pub fn find_object(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetAlpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "Initialize", args = 2)]
    pub fn initialize(
        self,
        unit: crate::app::unit::Unit,
        god_unit: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "BrightOn", args = 0)]
    pub fn bright_on(self) -> ();

    #[method(name = "BrightOff", args = 0)]
    pub fn bright_off(self) -> ();

    #[method(name = "Shine", args = 1)]
    pub fn shine(self, time: f32) -> ();

    #[method(name = "FadeIn", args = 1)]
    pub fn fade_in(self, time: f32) -> ();

    #[method(name = "FadeOut", args = 1)]
    pub fn fade_out(self, time: f32) -> ();

    #[method(name = "GodChange", args = 3)]
    pub fn god_change(self, time: f32, alpha: f32, name: ::unity2::Il2CppString) -> ();

    #[method(name = "GodIn", args = 1)]
    pub fn god_in(self, time: f32) -> ();

    #[method(name = "GodOut", args = 1)]
    pub fn god_out(self, time: f32) -> ();

    #[method(name = "SetMaterialFloat", args = 2)]
    pub fn set_material_float(self, name: ::unity2::Il2CppString, value: f32) -> ();

    #[method(name = "SetMaterialColor", args = 2)]
    pub fn set_material_color(
        self,
        name: ::unity2::Il2CppString,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetMaterialColor", args = 3)]
    pub fn set_material_color_2(
        self,
        material: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetMaterialAlpha", args = 1)]
    pub fn set_material_alpha(self, alpha: f32) -> ();

    #[method(name = "CommitModel", args = 0)]
    pub fn commit_model(self) -> ();

    #[method(name = "CommitRenader", args = 0)]
    pub fn commit_renader(self) -> ();

    #[method(name = "GetRelativePath", args = 2)]
    pub fn get_relative_path(
        root: crate::unity_engine::transform::Transform,
        transform: crate::unity_engine::transform::Transform,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CommitAccSkin", args = 2)]
    pub fn commit_acc_skin(
        self,
        root: crate::unity_engine::transform::Transform,
        acc: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "SetAnimatorController", args = 2)]
    pub fn set_animator_controller(
        self,
        go: crate::unity_engine::gameobject::GameObject,
        controller: crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController,
    ) -> ();

    #[method(name = "GetAnimator", args = 1)]
    pub fn get_animator(
        self,
        go: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::animator::Animator;

    #[method(name = "GetAnimatorController", args = 1)]
    pub fn get_animator_controller(
        self,
        go: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController;

    #[method(name = "GetFolderName", args = 1)]
    pub fn get_folder_name(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetAssetPath", args = 3)]
    pub fn get_asset_path(
        root: ::unity2::Il2CppString,
        name: ::unity2::Il2CppString,
        subs: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetFooter", args = 1)]
    pub fn get_footer(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "HeadFolder", args = 1)]
    pub fn head_folder(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "BodyFolder", args = 1)]
    pub fn body_folder(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "AccFolder", args = 1)]
    pub fn acc_folder(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Swap", args = 0)]
    pub fn swap(self) -> ();

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, result: crate::app::assettable::AssetTable_Result) -> bool;

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "TryDestroy", args = 1)]
    pub fn try_destroy(self, go: crate::unity_engine::gameobject::GameObject) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "UpdateLoading", args = 0)]
    pub fn update_loading(self) -> ();

    #[method(name = "UpdateVisible", args = 0)]
    pub fn update_visible(self) -> ();

    #[method(name = "UpdateVisible", args = 1)]
    pub fn update_visible_2(self, visible: bool) -> ();

    #[method(name = "TickColor", args = 0)]
    pub fn tick_color(self) -> ();

    #[method(name = "GetMapAlpha", args = 0)]
    pub fn get_map_alpha() -> f32;

    #[method(name = "UpdateColor", args = 0)]
    pub fn update_color(self) -> ();

    #[method(name = "CommitColor", args = 1)]
    pub fn commit_color(self, flags: crate::app::unitmodel::UnitModel_ColorFlags) -> ();

    #[method(name = "UpdateDirty", args = 0)]
    pub fn update_dirty(self) -> ();

    #[method(name = "UpdateWeight", args = 0)]
    pub fn update_weight(self) -> ();

    #[method(name = "PreTick", args = 0)]
    pub fn pre_tick(self) -> ();

    #[method(name = "PostTick", args = 0)]
    pub fn post_tick(self) -> ();

    #[method(name = "GetAnim", args = 0)]
    pub fn get_anim(self) -> crate::app::unitanim::UnitAnim_Types;

    #[method(name = "GetIdleAnim", args = 0)]
    pub fn get_idle_anim(self) -> crate::app::unitanim::UnitAnim_Types;

    #[method(name = "SetIdleAnim", args = 2)]
    pub fn set_idle_anim(
        self,
        r#type: crate::app::unitanim::UnitAnim_Types,
        time: crate::app::unitanim::UnitAnim_Times,
    ) -> ();

    #[method(name = "SetIdleAnim", args = 2)]
    pub fn set_idle_anim_2(
        self,
        action: crate::app::unitsequence::UnitSequence_Action,
        time: crate::app::unitanim::UnitAnim_Times,
    ) -> ();

    #[method(name = "PlayIdle", args = 1)]
    pub fn play_idle(self, time: crate::app::unitanim::UnitAnim_Times) -> ();

    #[method(name = "PlayAnim", args = 2)]
    pub fn play_anim(
        self,
        r#type: crate::app::unitanim::UnitAnim_Types,
        time: crate::app::unitanim::UnitAnim_Times,
    ) -> ();

    #[method(name = "Instantiate", args = 2)]
    pub fn instantiate(
        self,
        go: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "InstantiateLeftWeapon", args = 1)]
    pub fn instantiate_left_weapon(self, go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "InstantiateRightWeapon", args = 1)]
    pub fn instantiate_right_weapon(self, go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "GetItemKind", args = 1)]
    pub fn get_item_kind(
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "UpdateWeapon", args = 1)]
    pub fn update_weapon(self, equip_item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "GetAnimator", args = 0)]
    pub fn get_animator_2(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "UpdateAnim", args = 0)]
    pub fn update_anim(self) -> ();

    #[method(name = "SetForceItem", args = 1)]
    pub fn set_force_item(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "UpdateSpeed", args = 0)]
    pub fn update_speed(self) -> ();

    #[method(name = "SetSpeed", args = 1)]
    pub fn set_speed(self, speed: f32) -> ();

    #[method(name = "GetSpeed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "CommitProportion", args = 0)]
    pub fn commit_proportion(self) -> ();

    #[method(name = "GetAnimators", args = 0)]
    pub fn get_animators(self) -> ::unity2::Array<crate::unity_engine::animator::Animator>;

    #[method(name = "OnGameColorValidate", args = 0)]
    pub fn on_game_color_validate(self) -> ();

    #[method(name = "GetEffectPosition", args = 0)]
    pub fn get_effect_position(self) -> crate::unity_engine::vector3::Vector3;
}

#[cfg(feature = "app-unitmodel")]
impl UnitModel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitModel),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitModelMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitmodel/UnitModel_ResourceHandle.md")))]
#[::unity2::class(namespace = "App", name = "UnitModel.ResourceHandle")]
#[parent(crate::system::object::Object)]
pub struct UnitModel_ResourceHandle {
    #[rename(name = "BodyPrefab")]
    pub body_prefab: crate::app::resourcegameobject::ResourceGameObject,
    #[rename(name = "HeadPrefab")]
    pub head_prefab: crate::app::resourcegameobject::ResourceGameObject,
    #[rename(name = "RidePrefab")]
    pub ride_prefab: crate::app::resourcegameobject::ResourceGameObject,
    #[rename(name = "LeftHandPrefab")]
    pub left_hand_prefab: crate::app::resourcegameobject::ResourceGameObject,
    #[rename(name = "RightHandPrefab")]
    pub right_hand_prefab: crate::app::resourcegameobject::ResourceGameObject,
    #[rename(name = "BodyAnim")]
    pub body_anim: crate::app::resourceanimatorcontroller::ResourceAnimatorController,
    #[rename(name = "RideAnim")]
    pub ride_anim: crate::app::resourceanimatorcontroller::ResourceAnimatorController,
    #[rename(name = "AccPrefabs")]
    pub acc_prefabs: crate::system::collections::generic::list_1::List_1<
        crate::app::resourcegameobject::ResourceGameObject,
    >,
    #[rename(name = "AccLocators")]
    pub acc_locators: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
}

#[cfg(feature = "app-unitmodel")]
#[::unity2::methods]
impl UnitModel_ResourceHandle {
    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, result: crate::app::assettable::AssetTable_Result) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitmodel")]
impl UnitModel_ResourceHandle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitModel_ResourceHandle),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitModel_ResourceHandleMethods>::ctor(this);
        this
    }
}
