
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/valuecountcontroller/ValueCountController.md")))]
#[::unity2::class(namespace = "App", name = "ValueCountController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ValueCountController {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_ObjRoot")]
    pub m_obj_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TextValue")]
    pub m_text_value: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Anim")]
    pub m_anim: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-valuecountcontroller")]
#[::unity2::methods]
impl ValueCountController {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "Create", args = 2)]
    pub fn create(
        pos: crate::unity_engine::vector3::Vector3,
        value: i32,
    ) -> crate::app::valuecountcontroller::ValueCountController;

    #[method(name = "Initialize", args = 2)]
    pub fn initialize(self, pos: crate::unity_engine::vector3::Vector3, value: i32) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-valuecountcontroller")]
impl ValueCountController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ValueCountController),
                ::core::stringify!(new),
            )
        });
        <Self as IValueCountControllerMethods>::ctor(this);
        this
    }
}
