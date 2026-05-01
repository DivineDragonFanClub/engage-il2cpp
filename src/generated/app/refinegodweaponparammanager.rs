
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponparammanager/RefineGodWeaponParamManager_ReturnEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineGodWeaponParamManager.ReturnEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineGodWeaponParamManager_ReturnEventHandler {}

#[cfg(feature = "app-refinegodweaponparammanager")]
#[::unity2::methods]
impl RefineGodWeaponParamManager_ReturnEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        result: crate::app::basicmenu::BasicMenu_Result,
        god_weapon: crate::app::itemdata::ItemData,
    ) -> ();
}

#[cfg(feature = "app-refinegodweaponparammanager")]
impl RefineGodWeaponParamManager_ReturnEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponParamManager_ReturnEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponParamManager_ReturnEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponparammanager/RefineGodWeaponParamManager.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponParamManager")]
#[parent(crate::system::object::Object)]
pub struct RefineGodWeaponParamManager {
    #[rename(name = "m_ReturnEventHandler")]
    pub m_return_event_handler:
        crate::app::refinegodweaponparammanager::RefineGodWeaponParamManager_ReturnEventHandler,
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::refinegodweaponparammenu::RefineGodWeaponParamMenu,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[rename(name = "m_GodWeapon")]
    pub m_god_weapon: crate::app::itemdata::ItemData,
    #[rename(name = "m_RefineKind")]
    pub m_refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
    #[rename(name = "m_CurrentLevel")]
    pub m_current_level: i32,
}

#[cfg(feature = "app-refinegodweaponparammanager")]
#[::unity2::methods]
impl RefineGodWeaponParamManager {
    #[method(name = "Create", args = 5)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::itemdata::ItemData,
        return_event_handler : crate :: app :: refinegodweaponparammanager :: RefineGodWeaponParamManager_ReturnEventHandler,
    ) -> crate::app::refinegodweaponparammanager::RefineGodWeaponParamManager;

    #[method(name = "CreateForReset", args = 5)]
    pub fn create_for_reset(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::itemdata::ItemData,
        return_event_handler : crate :: app :: refinegodweaponparammanager :: RefineGodWeaponParamManager_ReturnEventHandler,
    ) -> crate::app::refinegodweaponparammanager::RefineGodWeaponParamManager;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::itemdata::ItemData,
        refine_or_reset: bool,
        return_event_handler : crate :: app :: refinegodweaponparammanager :: RefineGodWeaponParamManager_ReturnEventHandler,
    ) -> ();

    #[method(name = "OnSelect", args = 4)]
    pub fn on_select(
        self,
        god_unit: crate::app::godunit::GodUnit,
        item_data: crate::app::itemdata::ItemData,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        refine_or_reset: bool,
    ) -> ();

    #[method(name = "OnDecide", args = 4)]
    pub fn on_decide(
        self,
        refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        current_level: i32,
        sid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "OnYesNoDialogYes", args = 0)]
    pub fn on_yes_no_dialog_yes(self) -> ();

    #[method(name = "OnOkResult", args = 0)]
    pub fn on_ok_result(self) -> ();

    #[method(name = "OnDecideToReset", args = 4)]
    pub fn on_decide_to_reset(
        self,
        refine_data: crate::app::godweaponrefinedata::GodWeaponRefineData,
        refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        current_level: i32,
        sid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "OnResetYesNoDialogYes", args = 0)]
    pub fn on_reset_yes_no_dialog_yes(self) -> ();

    #[method(name = "OnOkResultToReset", args = 0)]
    pub fn on_ok_result_to_reset(self) -> ();

    #[method(name = "RefineGodWeapon_Do", args = 5)]
    pub fn refine_god_weapon_do(
        self,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::itemdata::ItemData,
        refine_kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        next_level: i32,
        sid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "RefineGodWeapon_Reset", args = 3)]
    pub fn refine_god_weapon_reset(
        self,
        god_unit: crate::app::godunit::GodUnit,
        kind: crate::app::godweaponrefinedata::GodWeaponRefineData_Kind,
        god_weapon: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "OnRequestClose", args = 0)]
    pub fn on_request_close(self) -> ();

    #[method(name = "TestToHasRejectiveSkill", args = 1)]
    pub fn test_to_has_rejective_skill(
        self,
        god_weapon_data: crate::app::itemdata::ItemData,
    ) -> bool;
}

#[cfg(feature = "app-refinegodweaponparammanager")]
impl RefineGodWeaponParamManager {
    pub fn new(
        super_: crate::app::procinst::ProcInst,
        root: crate::app::refinegodweaponroot::RefineGodWeaponRoot,
        god_unit: crate::app::godunit::GodUnit,
        god_weapon: crate::app::itemdata::ItemData,
        refine_or_reset: bool,
        return_event_handler : crate :: app :: refinegodweaponparammanager :: RefineGodWeaponParamManager_ReturnEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponParamManager),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponParamManagerMethods>::ctor(
            this,
            super_,
            root,
            god_unit,
            god_weapon,
            refine_or_reset,
            return_event_handler,
        );
        this
    }
}
