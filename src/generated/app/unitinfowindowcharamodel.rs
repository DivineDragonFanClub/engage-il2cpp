
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitinfowindowcharamodel/UnitInfoWindowCharaModel_ReservedCharaVoice.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitInfoWindowCharaModel.ReservedCharaVoice"
)]
#[parent(crate::system::object::Object)]
pub struct UnitInfoWindowCharaModel_ReservedCharaVoice {
    #[rename(name = "m_PersonSwitchName")]
    pub m_person_switch_name: ::unity2::Il2CppString,
    #[rename(name = "m_EngageSwitchName")]
    pub m_engage_switch_name: ::unity2::Il2CppString,
    #[rename(name = "m_EventName")]
    pub m_event_name: ::unity2::Il2CppString,
    #[rename(name = "m_Chara")]
    pub m_chara: crate::combat::character::Character,
}

#[cfg(feature = "app-unitinfowindowcharamodel")]
#[::unity2::methods]
impl UnitInfoWindowCharaModel_ReservedCharaVoice {
    #[method(name = "ReserveVoice", args = 4)]
    pub fn reserve_voice(
        self,
        person_switch_name: ::unity2::Il2CppString,
        engage_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
        chara: crate::combat::character::Character,
    ) -> ();

    #[method(name = "PlayVoice", args = 0)]
    pub fn play_voice(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitinfowindowcharamodel")]
impl UnitInfoWindowCharaModel_ReservedCharaVoice {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitInfoWindowCharaModel_ReservedCharaVoice),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitInfoWindowCharaModel_ReservedCharaVoiceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitinfowindowcharamodel/UnitInfoWindowCharaModel.md")))]
#[::unity2::class(namespace = "App", name = "UnitInfoWindowCharaModel")]
#[parent(crate::system::object::Object)]
pub struct UnitInfoWindowCharaModel {
# [static_field] # [rename (name = "PrefabPath")] pub prefab_path : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "AnimHashIsCharaOnly")] pub anim_hash_is_chara_only : i32 ,
# [static_field] # [rename (name = "AnimHashIsStandBy")] pub anim_hash_is_stand_by : i32 ,
# [static_field] # [rename (name = "AnimHashIsTransition")] pub anim_hash_is_transition : i32 ,
# [static_field] # [rename (name = "AnimHashIsTransparent")] pub anim_hash_is_transparent : i32 ,
# [static_field] # [rename (name = "AnimHashBasicFace")] pub anim_hash_basic_face : i32 ,
# [static_field] # [rename (name = "AnimHashBasicBody")] pub anim_hash_basic_body : i32 ,
# [static_field] # [rename (name = "AnimHashRingSelectFace")] pub anim_hash_ring_select_face : i32 ,
# [static_field] # [rename (name = "AnimHashRingSelectBody")] pub anim_hash_ring_select_body : i32 ,
# [static_field] # [rename (name = "AnimHashFortuneTellingBodyIdle")] pub anim_hash_fortune_telling_body_idle : i32 ,
# [static_field] # [rename (name = "AnimHashFortuneTellingFaceGood")] pub anim_hash_fortune_telling_face_good : i32 ,
# [static_field] # [rename (name = "AnimHashFortuneTellingBodyGood")] pub anim_hash_fortune_telling_body_good : i32 ,
# [static_field] # [rename (name = "AnimHashFortuneTellingFaceBad")] pub anim_hash_fortune_telling_face_bad : i32 ,
# [static_field] # [rename (name = "AnimHashFortuneTellingBodyBad")] pub anim_hash_fortune_telling_body_bad : i32 ,
# [rename (name = "m_AnimHashFortuneTellingFace")] pub m_anim_hash_fortune_telling_face : i32 ,
# [static_field] # [rename (name = "AnimHashStandByFace")] pub anim_hash_stand_by_face : i32 ,
# [static_field] # [rename (name = "AnimHashStandByFacePain")] pub anim_hash_stand_by_face_pain : i32 ,
# [static_field] # [rename (name = "AnimHashStandByBodyNoWeapon")] pub anim_hash_stand_by_body_no_weapon : i32 ,
# [static_field] # [rename (name = "StandByAnimeHashTable")] pub stand_by_anime_hash_table : :: unity2 :: Array < i32 > ,
# [rename (name = "m_ReservedCharaVoice")] pub m_reserved_chara_voice : crate :: app :: unitinfowindowcharamodel :: UnitInfoWindowCharaModel_ReservedCharaVoice ,
# [rename (name = "m_PrefabHandle")] pub m_prefab_handle : crate :: app :: tresourcehandle_1 :: TResourceHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ,
# [rename (name = "m_GameObject")] pub m_game_object : crate :: unity_engine :: gameobject :: GameObject ,
# [rename (name = "m_CameraObject")] pub m_camera_object : crate :: unity_engine :: gameobject :: GameObject ,
# [rename (name = "m_RenderTexture")] pub m_render_texture : crate :: unity_engine :: rendertexture :: RenderTexture ,
# [rename (name = "m_OffscreenCamera")] pub m_offscreen_camera : crate :: unity_engine :: rendering :: universal :: custom :: customoffscreencamera :: CustomOffscreenCamera ,
# [rename (name = "m_CharaUpdater")] pub m_chara_updater : crate :: app :: unitinfowindowcharaupdater :: UnitInfoWindowCharaUpdater ,
# [rename (name = "m_Animator")] pub m_animator : crate :: unity_engine :: animator :: Animator ,
# [rename (name = "m_IsValid")] pub m_is_valid : bool ,
# [rename (name = "m_IsCharaStandBy")] pub m_is_chara_stand_by : bool ,
# [rename (name = "m_IsReverse")] pub m_is_reverse : bool ,
# [rename (name = "m_IsReverseRotation")] pub m_is_reverse_rotation : bool ,
# [rename (name = "m_Unit")] pub m_unit : crate :: app :: unit :: Unit ,
# [rename (name = "m_God")] pub m_god : crate :: app :: godunit :: GodUnit ,
# [rename (name = "m_NextLoadUnit")] pub m_next_load_unit : crate :: app :: unit :: Unit ,
# [rename (name = "m_NextLoadGod")] pub m_next_load_god : crate :: app :: godunit :: GodUnit ,
# [rename (name = "m_NextCallback")] pub m_next_callback : crate :: app :: talk3_d :: characterfactoryasync_2 :: CharacterFactoryAsync_onLoad ,
# [rename (name = "m_Chara")] pub m_chara : crate :: combat :: character :: Character ,
# [rename (name = "m_CreateReserveUnitItem")] pub m_create_reserve_unit_item : crate :: app :: unititem :: UnitItem ,
# [rename (name = "m_IsEfficacy")] pub m_is_efficacy : bool ,
# [rename (name = "m_LookAtTransform")] pub m_look_at_transform : crate :: unity_engine :: transform :: Transform ,
# [static_field] # [rename (name = "c_LookSpeed")] pub c_look_speed : f32 ,
# [rename (name = "m_OnSetupDoneCallback")] pub m_on_setup_done_callback : crate :: system :: action :: Action ,
# [rename (name = "m_DelayFrameCount")] pub m_delay_frame_count : crate :: app :: gameparam :: GameParam_Holder ,
}

#[cfg(feature = "app-unitinfowindowcharamodel")]
#[::unity2::methods]
impl UnitInfoWindowCharaModel {
    #[method(name = "SetOnSetupDoneCallback", args = 1)]
    pub fn set_on_setup_done_callback(self, callback: crate::system::action::Action) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, is_duplicate_render_texture: bool, is_reverse: bool) -> ();

    #[method(name = "CreateAsync", args = 2)]
    pub fn create_async(self, is_duplicate_render_texture: bool, is_reverse: bool) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "IsLoadingModel", args = 0)]
    pub fn is_loading_model(self) -> bool;

    #[method(name = "SetUnit", args = 6)]
    pub fn set_unit(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        b_relax: bool,
        b_reverse_rotation: bool,
        is_delay_load: bool,
    ) -> ();

    #[method(name = "ForcedSetUnit", args = 4)]
    pub fn forced_set_unit(
        self,
        unit: crate::app::unit::Unit,
        b_relax: bool,
        b_reverse_rotation: bool,
        is_delay_load: bool,
    ) -> ();

    #[method(name = "SetUnitHub", args = 4)]
    pub fn set_unit_hub(
        self,
        unit: crate::app::unit::Unit,
        b_relax: bool,
        b_reverse_rotation: bool,
        is_delay_load: bool,
    ) -> ();

    #[method(name = "SetUnitRelay", args = 3)]
    pub fn set_unit_relay(
        self,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        edit: crate::app::unitedit::UnitEdit,
    ) -> ();

    #[method(name = "SetUnitImpl", args = 6)]
    pub fn set_unit_impl(
        self,
        unit: crate::app::unit::Unit,
        b_relax: bool,
        b_reverse_rotation: bool,
        is_delay_load: bool,
        is_changed: bool,
        is_hub: bool,
    ) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "UpdateMotion", args = 0)]
    pub fn update_motion(self) -> ();

    #[method(name = "UpdateVisible", args = 0)]
    pub fn update_visible(self) -> ();

    #[method(name = "CreateImpl", args = 2)]
    pub fn create_impl(self, is_duplicate_render_texture: bool, is_reverse: bool) -> ();

    #[method(name = "DeleteCharaModel", args = 0)]
    pub fn delete_chara_model(self) -> ();

    #[method(name = "DeleteCharaModel", args = 1)]
    pub fn delete_chara_model_2(self, chara: crate::combat::character::Character) -> ();

    #[method(name = "CreateCharaModel", args = 2)]
    pub fn create_chara_model(self, b_relax: bool, is_hub: bool) -> ();

    #[method(name = "UpdateCharacterAnimation", args = 4)]
    pub fn update_character_animation(
        self,
        chara: crate::combat::character::Character,
        b_relax: bool,
        is_hub: bool,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "SetGod", args = 4)]
    pub fn set_god(
        self,
        god: crate::app::godunit::GodUnit,
        b_relax: bool,
        b_reverse_rotation: bool,
        is_delay_load: bool,
    ) -> ();

    #[method(name = "CharaOnlyOn", args = 0)]
    pub fn chara_only_on(self) -> ();

    #[method(name = "CharaOnlyOff", args = 0)]
    pub fn chara_only_off(self) -> ();

    #[method(name = "IsCharaOnlyTransition", args = 0)]
    pub fn is_chara_only_transition(self) -> bool;

    #[method(name = "SetCreateReserveUnitItem", args = 2)]
    pub fn set_create_reserve_unit_item(
        self,
        unit_item: crate::app::unititem::UnitItem,
        is_override: bool,
    ) -> ();

    #[method(name = "UpdateStandByAnime", args = 2)]
    pub fn update_stand_by_anime(
        self,
        unit_item: crate::app::unititem::UnitItem,
        is_weapon_shop: bool,
    ) -> ();

    #[method(name = "SetEfficacyAttack", args = 1)]
    pub fn set_efficacy_attack(self, is_efficacy: bool) -> ();

    #[method(name = "SetRelaxAnime", args = 1)]
    pub fn set_relax_anime(self, transition_duration: f32) -> ();

    #[method(name = "SetStatusAnime", args = 1)]
    pub fn set_status_anime(self, transition_duration: f32) -> ();

    #[method(name = "SetFortuneTellingGoodAnime", args = 1)]
    pub fn set_fortune_telling_good_anime(self, is_allow_unit_null: bool) -> ();

    #[method(name = "SetFortuneTellingBadAnime", args = 1)]
    pub fn set_fortune_telling_bad_anime(self, is_allow_unit_null: bool) -> ();

    #[method(name = "SetWeaponShopChara", args = 0)]
    pub fn set_weapon_shop_chara(self) -> ();

    #[method(name = "SetSummonChara", args = 0)]
    pub fn set_summon_chara(self) -> ();

    #[method(name = "UpdateGodSelectNormalFaceAnime", args = 0)]
    pub fn update_god_select_normal_face_anime(self) -> ();

    #[method(name = "ShowWeapon", args = 1)]
    pub fn show_weapon(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "HideWeapon", args = 0)]
    pub fn hide_weapon(self) -> ();

    #[method(name = "TransparentOn", args = 0)]
    pub fn transparent_on(self) -> ();

    #[method(name = "TransparentOff", args = 0)]
    pub fn transparent_off(self) -> ();

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Deactivate", args = 1)]
    pub fn deactivate(self, b_clear_stand_by: bool) -> ();

    #[method(name = "SetCharaImage", args = 1)]
    pub fn set_chara_image(self, image_simple: crate::unity_engine::ui::image::Image) -> ();

    #[method(name = "get_RenderTexture", args = 0)]
    pub fn get_render_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "get_FaceCameraComponent", args = 0)]
    pub fn get_face_camera_component(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "GetHeadLocator", args = 0)]
    pub fn get_head_locator(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "SetHeadLocator", args = 2)]
    pub fn set_head_locator(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        default_weight: f32,
    ) -> ();

    #[method(name = "SetHeadLocator", args = 3)]
    pub fn set_head_locator_2(
        self,
        loc: crate::unity_engine::transform::Transform,
        default_weight: f32,
        is_weight_interpolated: bool,
    ) -> ();

    #[method(name = "SetLookAt", args = 1)]
    pub fn set_look_at(self, transform: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "SetLookAtCamera", args = 0)]
    pub fn set_look_at_camera(self) -> ();

    #[method(name = "SetEyesWeight", args = 1)]
    pub fn set_eyes_weight(self, default_eyes_weight: f32) -> ();

    #[method(name = "IsCharaVisible", args = 0)]
    pub fn is_chara_visible(self) -> bool;

    #[method(name = "PlayCharaVoice", args = 3)]
    pub fn play_chara_voice(
        self,
        person_switch_name: ::unity2::Il2CppString,
        engage_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ReserveCharaVoice", args = 3)]
    pub fn reserve_chara_voice(
        self,
        person_switch_name: ::unity2::Il2CppString,
        engage_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PlayReservedCharaVoice", args = 0)]
    pub fn play_reserved_chara_voice(self) -> ();

    #[method(name = "AddCharaRot", args = 1)]
    pub fn add_chara_rot(self, quaternion: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "SetCameraAdjustY", args = 0)]
    pub fn set_camera_adjust_y(self) -> ();

    #[method(name = "Setup", args = 2)]
    pub fn setup(self, is_duplicate_render_texture: bool, is_reverse: bool) -> bool;

    #[method(name = "GetGameObject", args = 1)]
    pub fn get_game_object(
        self,
        obj_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateUnitModel", args = 3)]
    pub fn create_unit_model(
        self,
        unit: crate::app::unit::Unit,
        callback: crate::app::talk3_d::characterfactoryasync_2::CharacterFactoryAsync_onLoad,
        is_hub: bool,
    ) -> ();

    #[method(name = "CreateGodModel", args = 2)]
    pub fn create_god_model(
        self,
        god: crate::app::godunit::GodUnit,
        callback: crate::app::talk3_d::characterfactoryasync_2::CharacterFactoryAsync_onLoad,
    ) -> ();

    #[method(name = "CreateUnitModelRelay", args = 4)]
    pub fn create_unit_model_relay(
        self,
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
        edit: crate::app::unitedit::UnitEdit,
        callback: crate::app::talk3_d::characterfactoryasync_2::CharacterFactoryAsync_onLoad,
    ) -> ();

    #[method(name = "WaitLoading", args = 0)]
    pub fn wait_loading(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "HideCharaImage", args = 0)]
    pub fn hide_chara_image(self) -> ();

    #[method(name = "OnSetupDone", args = 0)]
    pub fn on_setup_done(self) -> ();

    #[method(name = "CreateCharaModel", args = 1)]
    pub fn create_chara_model_2(
        self,
        chara: crate::combat::character::Character,
    ) -> crate::combat::character::Character;

    #[method(name = "isPain", args = 0)]
    pub fn is_pain(self) -> bool;

    #[method(name = "get_GameObject", args = 0)]
    pub fn get_game_object_2(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-unitinfowindowcharamodel")]
impl UnitInfoWindowCharaModel {
    pub fn new(is_duplicate_render_texture: bool, is_reverse: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitInfoWindowCharaModel),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitInfoWindowCharaModelMethods>::ctor(
            this,
            is_duplicate_render_texture,
            is_reverse,
        );
        this
    }
}
