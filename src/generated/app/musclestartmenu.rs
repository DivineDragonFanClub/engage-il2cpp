
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclestartmenu/MuscleStartMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "MuscleStartMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MuscleStartMenu_DecideEventHandler {}

#[cfg(feature = "app-musclestartmenu")]
#[::unity2::methods]
impl MuscleStartMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::muscle_exercise::r#type::Type) -> ();
}

#[cfg(feature = "app-musclestartmenu")]
impl MuscleStartMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleStartMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleStartMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclestartmenu/MuscleStartMenu_MuscleStartMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MuscleStartMenu.MuscleStartMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MuscleStartMenu_MuscleStartMenuItem {
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CommentText")]
    pub m_comment_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RuleText")]
    pub m_rule_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EffectText")]
    pub m_effect_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ID")]
    pub m_id: ::unity2::Il2CppString,
    #[rename(name = "m_ItemName")]
    pub m_item_name: ::unity2::Il2CppString,
    #[rename(name = "m_Title")]
    pub m_title: ::unity2::Il2CppString,
    #[rename(name = "m_Comment")]
    pub m_comment: ::unity2::Il2CppString,
    #[rename(name = "m_Rule")]
    pub m_rule: ::unity2::Il2CppString,
    #[rename(name = "m_Effect")]
    pub m_effect: ::unity2::Il2CppString,
}

#[cfg(feature = "app-musclestartmenu")]
#[::unity2::methods]
impl MuscleStartMenu_MuscleStartMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        set_menu: crate::unity_engine::gameobject::GameObject,
        id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-musclestartmenu")]
impl MuscleStartMenu_MuscleStartMenuItem {
    pub fn new(
        set_menu: crate::unity_engine::gameobject::GameObject,
        id: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleStartMenu_MuscleStartMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleStartMenu_MuscleStartMenuItemMethods>::ctor(this, set_menu, id);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclestartmenu/MuscleStartMenu.md")))]
#[::unity2::class(namespace = "App", name = "MuscleStartMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MuscleStartMenu {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::musclestartmenu::MuscleStartMenu_DecideEventHandler,
}

#[cfg(feature = "app-musclestartmenu")]
#[::unity2::methods]
impl MuscleStartMenu {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        decide_event_handler: crate::app::musclestartmenu::MuscleStartMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_select: i32,
        decide_event_handler: crate::app::musclestartmenu::MuscleStartMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-musclestartmenu")]
impl MuscleStartMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        decide_event_handler: crate::app::musclestartmenu::MuscleStartMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleStartMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleStartMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            decide_event_handler,
        );
        this
    }
}
