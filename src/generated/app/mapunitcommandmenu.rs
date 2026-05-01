
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::mapbasicmenu::IMapBasicMenu;
use crate::app::mapbasicmenu::MapBasicMenu;
use crate::app::mapbasicmenuitem::IMapBasicMenuItem;
use crate::app::mapbasicmenuitem::MapBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_ItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.ItemMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_ItemMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_ItemMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_ItemMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_ItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_ItemMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_AttackMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.AttackMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_AttackMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_AttackMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActionMask", args = 0)]
    pub fn get_action_mask(self) -> crate::app::maptarget::MapTarget_ActionMask;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "CheckBuildableWithUnit", args = 1)]
    pub fn check_buildable_with_unit(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanEnumerateBuild", args = 0)]
    pub fn can_enumerate_build(self) -> bool;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_AttackMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_AttackMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_AttackMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageRewarpMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageRewarpMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageActionBaseMenuItem)]
pub struct MapUnitCommandMenu_EngageRewarpMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageRewarpMenuItem {
    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageRewarpMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageRewarpMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageRewarpMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_VisitMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.VisitMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_VisitMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_VisitMenuItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_VisitMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_VisitMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_VisitMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_CannonMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.CannonMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_CannonMenuItem {
    #[rename(name = "m_State")]
    pub m_state: crate::app::mapunitcommandmenu::MapUnitCommandMenu_CannonMenuItem_States,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_CannonMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = "CalcState", args = 0)]
    pub fn calc_state(
        self,
    ) -> crate::app::mapunitcommandmenu::MapUnitCommandMenu_CannonMenuItem_States;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_CannonMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_CannonMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_CannonMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_BreakdownMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.BreakdownMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BreakdownBaseMenuItem)]
pub struct MapUnitCommandMenu_BreakdownMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_BreakdownMenuItem {
    #[method(name = "GetForce", args = 0)]
    pub fn get_force(self) -> crate::app::force::Force_Type;

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::mapinspector::MapInspector_Kind;

    #[method(name = "GetMind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_BreakdownMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_BreakdownMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_BreakdownMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_TreasureBoxMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.TreasureBoxMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_TreasureBoxMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_TreasureBoxMenuItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_TreasureBoxMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_TreasureBoxMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_TreasureBoxMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageWaitMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageWaitMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageActionBaseMenuItem)]
pub struct MapUnitCommandMenu_EngageWaitMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageWaitMenuItem {
    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActionMask", args = 0)]
    pub fn get_action_mask(self) -> crate::app::maptarget::MapTarget_ActionMask;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageWaitMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageWaitMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageWaitMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_JumpToMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.JumpToMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_JumpToMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_JumpToMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "JumpSequence", args = 0)]
    pub fn jump_sequence(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "Enumerate", args = 2)]
    pub fn enumerate(
        self,
        mind: crate::app::mapmind::MapMind_Type,
        action_mask: crate::app::maptarget::MapTarget_ActionMask,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_JumpToMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_JumpToMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_JumpToMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_SkillAttackMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.SkillAttackMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_AttackMenuItem)]
pub struct MapUnitCommandMenu_SkillAttackMenuItem {
    #[rename(name = "m_CommandSkill")]
    pub m_command_skill: crate::app::skilldata::SkillData,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_SkillAttackMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, command_skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_CommandSkill", args = 0)]
    pub fn get_command_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_SkillAttackMenuItem {
    pub fn new(command_skill: crate::app::skilldata::SkillData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_SkillAttackMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_SkillAttackMenuItemMethods>::ctor(this, command_skill);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_SubMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.SubMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_SubMenuItem {
    #[rename(name = "m_CommandSkill")]
    pub m_command_skill: crate::app::skilldata::SkillData,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_SubMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_SubMenuItem {
    pub fn new(skill: crate::app::skilldata::SkillData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_SubMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_SubMenuItemMethods>::ctor(this, skill);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_NoCancelMagicMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.NoCancelMagicMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_SkillAttackMenuItem)]
pub struct MapUnitCommandMenu_NoCancelMagicMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_NoCancelMagicMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_NoCancelMagicMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_NoCancelMagicMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_NoCancelMagicMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_ContractMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.ContractMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_ContractMenuItem {
    #[rename(name = "m_ContractSkill")]
    pub m_contract_skill: crate::app::skilldata::SkillData,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_ContractMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_ContractMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_ContractMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_ContractMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_TradeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.TradeMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_TradeMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_TradeMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_TradeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_TradeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_TradeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_OverlapSkillMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.OverlapSkillMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct MapUnitCommandMenu_OverlapSkillMenuItem {
    #[rename(name = "m_Skill")]
    pub m_skill: crate::app::skilldata::SkillData,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_OverlapSkillMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "SetupHelpText", args = 0)]
    pub fn setup_help_text(self) -> ();

    #[method(name = "Enumerate", args = 1)]
    pub fn enumerate(self, skill: crate::app::skilldata::SkillData) -> i32;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_OverlapSkillMenuItem {
    pub fn new(skill: crate::app::skilldata::SkillData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_OverlapSkillMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_OverlapSkillMenuItemMethods>::ctor(this, skill);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageSummonMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageSummonMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageActionBaseMenuItem)]
pub struct MapUnitCommandMenu_EngageSummonMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageSummonMenuItem {
    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActionMask", args = 0)]
    pub fn get_action_mask(self) -> crate::app::maptarget::MapTarget_ActionMask;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageSummonMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageSummonMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageSummonMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_DestroyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.DestroyMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_DestroyMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_DestroyMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_DestroyMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_DestroyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_DestroyMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_GodChangeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.GodChangeMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_GodChangeMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_GodChangeMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_GodChangeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_GodChangeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_GodChangeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_GuardMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.GuardMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_GuardMenuItem {
    #[rename(name = "m_GuardType")]
    pub m_guard_type: crate::app::unit::Unit_GuardType,
    #[rename(name = "m_GuardSkill")]
    pub m_guard_skill: crate::app::skilldata::SkillData,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_GuardMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_GuardMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_GuardMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_GuardMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_BreakdownEnemyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.BreakdownEnemyMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BreakdownBaseMenuItem)]
pub struct MapUnitCommandMenu_BreakdownEnemyMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_BreakdownEnemyMenuItem {
    #[method(name = "GetForce", args = 0)]
    pub fn get_force(self) -> crate::app::force::Force_Type;

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::mapinspector::MapInspector_Kind;

    #[method(name = "GetMind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_BreakdownEnemyMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_BreakdownEnemyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_BreakdownEnemyMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_TorchOnMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.TorchOnMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_TorchOnMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_TorchOnMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTerrainCommand", args = 0)]
    pub fn get_terrain_command(self) -> crate::app::terraindata_2::TerrainData_Commands;

    #[method(name = "Decide", args = 1)]
    pub fn decide(self, mind: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_TorchOnMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_TorchOnMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_TorchOnMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_TargetSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.TargetSelectMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_TargetSelectMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_TargetSelectMenuItem {
    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "Enumerate", args = 1)]
    pub fn enumerate(self, skill: crate::app::skilldata::SkillData) -> i32;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_TargetSelectMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_TargetSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_TargetSelectMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageActionBaseMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapUnitCommandMenu.EngageActionBaseMenuItem"
)]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_SkillAttackMenuItem)]
pub struct MapUnitCommandMenu_EngageActionBaseMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageActionBaseMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetEngageMind", args = 1)]
    pub fn get_engage_mind(self, unit: crate::app::unit::Unit)
        -> crate::app::mapmind::MapMind_Type;

    #[method(name = "CheckBuildableWithUnit", args = 1)]
    pub fn check_buildable_with_unit(self, unit: crate::app::unit::Unit) -> bool;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageActionBaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageActionBaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageActionBaseMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_FixedMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.FixedMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_FixedMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_FixedMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_FixedMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_FixedMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_FixedMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_BreakdownBaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.BreakdownBaseMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_BreakdownBaseMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_BreakdownBaseMenuItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetForce", args = 0)]
    pub fn get_force(self) -> crate::app::force::Force_Type;

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::mapinspector::MapInspector_Kind;

    #[method(name = "GetMind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_BreakdownBaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_BreakdownBaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_BreakdownBaseMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_CannonMenuItem_States.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapUnitCommandMenu_CannonMenuItem_States {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapUnitCommandMenu_CannonMenuItem_States {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapUnitCommandMenu.CannonMenuItem.States";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapUnitCommandMenu_CannonMenuItem_States {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapUnitCommandMenu_CannonMenuItem_States {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn not_shell() -> Self {
        Self { value: 1 }
    }

    pub fn not_target() -> Self {
        Self { value: 2 }
    }

    pub fn not_bow() -> Self {
        Self { value: 3 }
    }

    pub fn not_magic() -> Self {
        Self { value: 4 }
    }

    pub fn cannon() -> Self {
        Self { value: 5 }
    }

    pub fn fire_cannon() -> Self {
        Self { value: 6 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EnchantWeaponMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EnchantWeaponMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EnchantMenuItem)]
pub struct MapUnitCommandMenu_EnchantWeaponMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EnchantWeaponMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_IsWeapon", args = 0)]
    pub fn get_is_weapon(self) -> bool;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EnchantWeaponMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EnchantWeaponMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EnchantWeaponMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_TalkMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.TalkMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_TalkMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_TalkMenuItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_TalkMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_TalkMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_TalkMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_RodMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.RodMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_RodMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_RodMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_RodMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_RodMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_RodMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageCommandMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageCommandMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_EngageCommandMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageCommandMenuItem {
    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageCommandMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageCommandMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageCommandMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_NoCancelFixedMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.NoCancelFixedMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_FixedMenuItem)]
pub struct MapUnitCommandMenu_NoCancelFixedMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_NoCancelFixedMenuItem {
    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_NoCancelFixedMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_NoCancelFixedMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_NoCancelFixedMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageStartMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageStartMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageCommandMenuItem)]
pub struct MapUnitCommandMenu_EngageStartMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageStartMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageStartMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageStartMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageStartMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_VisionDeleteMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.VisionDeleteMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_VisionDeleteMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_VisionDeleteMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_VisionDeleteMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_VisionDeleteMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_VisionDeleteMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_DanceMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.DanceMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_DanceMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_DanceMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_DanceMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_DanceMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_DanceMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_DoorMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.DoorMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_DoorMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_DoorMenuItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_DoorMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_DoorMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_DoorMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_VisionCreateMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.VisionCreateMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_VisionCreateMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_VisionCreateMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_VisionCreateMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_VisionCreateMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_VisionCreateMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_BreakthroughMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.BreakthroughMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_BreakthroughMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_BreakthroughMenuItem {
    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_BreakthroughMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_BreakthroughMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_BreakthroughMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EnchantItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EnchantItemMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EnchantMenuItem)]
pub struct MapUnitCommandMenu_EnchantItemMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EnchantItemMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_IsWeapon", args = 0)]
    pub fn get_is_weapon(self) -> bool;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EnchantItemMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EnchantItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EnchantItemMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageLinkMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageLinkMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageCommandMenuItem)]
pub struct MapUnitCommandMenu_EngageLinkMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageLinkMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageLinkMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageLinkMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageLinkMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageAttackMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageAttackMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageActionBaseMenuItem)]
pub struct MapUnitCommandMenu_EngageAttackMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageAttackMenuItem {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActionMask", args = 0)]
    pub fn get_action_mask(self) -> crate::app::maptarget::MapTarget_ActionMask;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageAttackMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageAttackMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageAttackMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu")]
#[parent(crate::app::mapbasicmenu::MapBasicMenu)]
pub struct MapUnitCommandMenu {
    #[static_field]
    #[rename(name = "s_SelectIndex")]
    pub s_select_index: i32,
    #[rename(name = "m_MapUnitCommandMenuContent")]
    pub m_map_unit_command_menu_content:
        crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
    ) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ResetSelectIndex", args = 0)]
    pub fn reset_select_index() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_CommandSkillMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.CommandSkillMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_CommandSkillMenuItem {
    #[rename(name = "m_CommandSkill")]
    pub m_command_skill: crate::app::skilldata::SkillData,
}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_CommandSkillMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, command_skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_CommandSkill", args = 0)]
    pub fn get_command_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_CommandSkillMenuItem {
    pub fn new(command_skill: crate::app::skilldata::SkillData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_CommandSkillMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_CommandSkillMenuItemMethods>::ctor(this, command_skill);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_TransporterMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.TransporterMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_BaseMenuItem)]
pub struct MapUnitCommandMenu_TransporterMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_TransporterMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_TransporterMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_TransporterMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_TransporterMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_BaseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.BaseMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct MapUnitCommandMenu_BaseMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_BaseMenuItem {
    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_CommandSkill", args = 0)]
    pub fn get_command_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "CanCancel", args = 0)]
    pub fn can_cancel(self) -> bool;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "SetupHelpText", args = 0)]
    pub fn setup_help_text(self) -> ();

    #[method(name = "CheckBuildableWithUnit", args = 0)]
    pub fn check_buildable_with_unit(self) -> bool;

    #[method(name = "CheckBuildableWithUnit", args = 1)]
    pub fn check_buildable_with_unit_2(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "JumpMindSequence", args = 1)]
    pub fn jump_mind_sequence(self, r#type: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "JumpSequence", args = 1)]
    pub fn jump_sequence(self, label: crate::app::mapsequencehuman::MapSequenceHuman_Label) -> ();

    #[method(name = "GetDisableAttribute", args = 0)]
    pub fn get_disable_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_BaseMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_BaseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_BaseMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageChargeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageChargeMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageActionBaseMenuItem)]
pub struct MapUnitCommandMenu_EngageChargeMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageChargeMenuItem {
    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActionMask", args = 0)]
    pub fn get_action_mask(self) -> crate::app::maptarget::MapTarget_ActionMask;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageChargeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageChargeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageChargeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EscapeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EscapeMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_TargetSelectMenuItem)]
pub struct MapUnitCommandMenu_EscapeMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EscapeMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EscapeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EscapeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EscapeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EnchantMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EnchantMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_JumpToMenuItem)]
pub struct MapUnitCommandMenu_EnchantMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EnchantMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsWeapon", args = 0)]
    pub fn get_is_weapon(self) -> bool;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "HasEnchantItem", args = 2)]
    pub fn has_enchant_item(self, unit: crate::app::unit::Unit, is_weapon: bool) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EnchantMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EnchantMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EnchantMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitcommandmenu/MapUnitCommandMenu_EngageRodMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitCommandMenu.EngageRodMenuItem")]
#[parent(crate::app::mapunitcommandmenu::MapUnitCommandMenu_EngageActionBaseMenuItem)]
pub struct MapUnitCommandMenu_EngageRodMenuItem {}

#[cfg(feature = "app-mapunitcommandmenu")]
#[::unity2::methods]
impl MapUnitCommandMenu_EngageRodMenuItem {
    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_ActionMask", args = 0)]
    pub fn get_action_mask(self) -> crate::app::maptarget::MapTarget_ActionMask;

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::mapsequencehuman::MapSequenceHuman_Label;

    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = "get_IsForecast", args = 0)]
    pub fn get_is_forecast(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapunitcommandmenu")]
impl MapUnitCommandMenu_EngageRodMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitCommandMenu_EngageRodMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitCommandMenu_EngageRodMenuItemMethods>::ctor(this);
        this
    }
}
