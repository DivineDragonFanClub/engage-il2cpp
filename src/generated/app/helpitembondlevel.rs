
use crate::app::helpitembase::HelpItemBase;
use crate::app::helpitembase::IHelpItemBase;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/helpitembondlevel/HelpItemBondLevel.md")))]
#[::unity2::class(namespace = "App", name = "HelpItemBondLevel")]
#[parent(crate::app::helpitembase::HelpItemBase)]
pub struct HelpItemBondLevel {
    #[rename(name = "m_MID")]
    pub m_mid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-helpitembondlevel")]
#[::unity2::methods]
impl HelpItemBondLevel {
    #[method(name = "get_HelpItemType", args = 0)]
    pub fn get_help_item_type(self) -> crate::app::helpmanager::HelpManager_HelpItemType;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "SetContents", args = 1)]
    pub fn set_contents(self, setter: crate::app::helpparamsetter::HelpParamSetter) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-helpitembondlevel")]
impl HelpItemBondLevel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HelpItemBondLevel),
                ::core::stringify!(new),
            )
        });
        <Self as IHelpItemBondLevelMethods>::ctor(this);
        this
    }
}
