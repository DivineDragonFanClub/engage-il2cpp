
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/root/configmenu/ConfigMenu.md")))]
#[::unity2::class(namespace = "", name = "ConfigMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ConfigMenu {}

#[cfg(feature = "root-configmenu")]
#[::unity2::methods]
impl ConfigMenu {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnInitialize", args = 0)]
    pub fn on_initialize(self) -> ();

    #[method(name = "CanToggleEnableNetwork", args = 0)]
    pub fn can_toggle_enable_network() -> bool;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
    ) -> crate::unity_engine::gameobject::GameObject;
}
