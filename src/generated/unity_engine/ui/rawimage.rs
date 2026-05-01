
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::graphic::Graphic;
use crate::unity_engine::ui::graphic::IGraphic;
use crate::unity_engine::ui::maskablegraphic::IMaskableGraphic;
use crate::unity_engine::ui::maskablegraphic::MaskableGraphic;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/rawimage/RawImage.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "RawImage")]
#[parent(crate::unity_engine::ui::maskablegraphic::MaskableGraphic)]
pub struct RawImage {
    #[rename(name = "m_Texture")]
    pub m_texture: crate::unity_engine::texture::Texture,
    #[rename(name = "m_UVRect")]
    pub m_uv_rect: crate::unity_engine::rect::Rect,
}

#[cfg(feature = "unity_engine-ui-rawimage")]
#[::unity2::methods]
impl RawImage {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_mainTexture", args = 0)]
    pub fn get_main_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "get_texture", args = 0)]
    pub fn get_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "set_texture", args = 1)]
    pub fn set_texture(self, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "get_uvRect", args = 0)]
    pub fn get_uv_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "set_uvRect", args = 1)]
    pub fn set_uv_rect(self, value: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "SetNativeSize", args = 0)]
    pub fn set_native_size(self) -> ();

    #[method(name = "OnPopulateMesh", args = 1)]
    pub fn on_populate_mesh(self, vh: crate::unity_engine::ui::vertexhelper::VertexHelper) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();
}

#[cfg(feature = "unity_engine-ui-rawimage")]
impl RawImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RawImage),
                ::core::stringify!(new),
            )
        });
        <Self as IRawImageMethods>::ctor(this);
        this
    }
}
