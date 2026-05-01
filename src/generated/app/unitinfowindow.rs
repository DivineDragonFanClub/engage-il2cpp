
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitinfowindow/UnitInfoWindow.md")))]
#[::unity2::class(namespace = "App", name = "UnitInfoWindow")]
#[parent(crate::system::object::Object)]
pub struct UnitInfoWindow {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "AnimHashIsCharaOnly")]
    pub anim_hash_is_chara_only: i32,
    #[static_field]
    #[rename(name = "AnimHashIsTransition")]
    pub anim_hash_is_transition: i32,
    #[static_field]
    #[rename(name = "AnimHashIsTransparent")]
    pub anim_hash_is_transparent: i32,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_AnimatorStatus")]
    pub m_animator_status: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CharaImageMask")]
    pub m_chara_image_mask: crate::app::unitinfocharaimagemaskoffset::UnitInfoCharaImageMaskOffset,
    #[rename(name = "m_IsValid")]
    pub m_is_valid: bool,
}

#[cfg(feature = "app-unitinfowindow")]
#[::unity2::methods]
impl UnitInfoWindow {
    #[method(name = "get_gameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_CharaImageMask", args = 0)]
    pub fn get_chara_image_mask(
        self,
    ) -> crate::app::unitinfocharaimagemaskoffset::UnitInfoCharaImageMaskOffset;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        render_texture: crate::unity_engine::rendertexture::RenderTexture,
        is_reverse: bool,
        is_hide_status: bool,
    ) -> ();

    #[method(name = "CreateAsync", args = 3)]
    pub fn create_async(
        self,
        render_texture: crate::unity_engine::rendertexture::RenderTexture,
        is_reverse: bool,
        is_hide_status: bool,
    ) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, is_active: bool) -> ();

    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "SetUnit", args = 3)]
    pub fn set_unit(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "SetUnit", args = 4)]
    pub fn set_unit_2(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "CharaMaskOn", args = 0)]
    pub fn chara_mask_on(self) -> ();

    #[method(name = "CharaMaskOff", args = 0)]
    pub fn chara_mask_off(self) -> ();

    #[method(name = "TryPlayAnimation", args = 2)]
    pub fn try_play_animation(
        self,
        animator: crate::unity_engine::animator::Animator,
        name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "CharaOnlyOn", args = 0)]
    pub fn chara_only_on(self) -> ();

    #[method(name = "CharaOnlyOff", args = 0)]
    pub fn chara_only_off(self) -> ();

    #[method(name = "IsCharaOnlyTransition", args = 0)]
    pub fn is_chara_only_transition(self) -> bool;

    #[method(name = "TransparentOn", args = 0)]
    pub fn transparent_on(self) -> ();

    #[method(name = "TransparentOff", args = 0)]
    pub fn transparent_off(self) -> ();

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Deactivate", args = 0)]
    pub fn deactivate(self) -> ();

    #[method(name = "GetCharaImage", args = 0)]
    pub fn get_chara_image(self) -> crate::unity_engine::ui::image::Image;

    #[method(name = "ActiveStatus", args = 0)]
    pub fn active_status(self) -> ();

    #[method(name = "DeactiveStatus", args = 0)]
    pub fn deactive_status(self) -> ();

    #[method(name = "IsActiveStatus", args = 0)]
    pub fn is_active_status(self) -> bool;

    #[method(name = "CreateImpl", args = 3)]
    pub fn create_impl(
        self,
        render_texture: crate::unity_engine::rendertexture::RenderTexture,
        is_reverse: bool,
        is_hide_status: bool,
    ) -> ();

    #[method(name = "Setup", args = 3)]
    pub fn setup(
        self,
        render_texture: crate::unity_engine::rendertexture::RenderTexture,
        is_reverse: bool,
        is_hide_status: bool,
    ) -> bool;

    #[method(name = "GetCharaImage", args = 2)]
    pub fn get_chara_image_2(
        self,
        chara_image_obj: crate::unity_engine::gameobject::GameObject,
        chara_image_image_obj: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::ui::image::Image;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-unitinfowindow")]
impl UnitInfoWindow {
    pub fn new(
        render_texture: crate::unity_engine::rendertexture::RenderTexture,
        is_reverse: bool,
        is_hide_status: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitInfoWindow),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitInfoWindowMethods>::ctor(this, render_texture, is_reverse, is_hide_status);
        this
    }
}
