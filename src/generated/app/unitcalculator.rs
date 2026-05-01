
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::app::gamecalculatorcommand::GameCalculatorCommand;
use crate::app::gamecalculatorcommand::IGameCalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_RangeGenderCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.RangeGenderCountCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_RangeGenderCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_RangeGenderCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_RangeGenderCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_RangeGenderCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_RangeGenderCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator")]
#[parent(crate::system::object::Object)]
pub struct UnitCalculator {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator {
    #[method(name = "GetItemEquipped", args = 1)]
    pub fn get_item_equipped(unit: crate::app::unit::Unit) -> crate::app::unititem::UnitItem;

    #[method(name = "HasForceUnit", args = 3)]
    pub fn has_force_unit(x: i32, z: i32, force_type: crate::app::force::Force_Type) -> bool;

    #[method(name = "AddCommand", args = 1)]
    pub fn add_command(calculator: crate::app::gamecalculator::GameCalculator) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculatorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponAvoidCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponAvoidCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponAvoidCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponAvoidCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponAvoidCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponAvoidCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponAvoidCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponEnduranceCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponEnduranceCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponEnduranceCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponEnduranceCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetEndurance", args = 1)]
    pub fn get_endurance(unit_item: crate::app::unititem::UnitItem) -> i32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponEnduranceCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponEnduranceCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponEnduranceCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleEscortCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleEscortCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleEscortCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleEscortCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleEscortCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleEscortCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleEscortCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_HitCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.HitCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_HitCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_HitCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_HitCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_HitCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_HitCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_GainGoldCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.GainGoldCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_GainGoldCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_GainGoldCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_GainGoldCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_GainGoldCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_GainGoldCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_GodLevelCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.GodLevelCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_GodLevelCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_GodLevelCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_GodLevelCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_GodLevelCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_GodLevelCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkPhysCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkPhysCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkPhysCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkPhysCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkPhysCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkPhysCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkPhysCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SupportLevelCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SupportLevelCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_SupportLevelCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SupportLevelCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SupportLevelCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SupportLevelCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SupportLevelCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponHitCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponHitCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponHitCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponHitCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponHitCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponHitCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponHitCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleAliveCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleAliveCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleAliveCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleAliveCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleAliveCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleAliveCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleAliveCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleParamCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleParamCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleParamCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleParamCommand {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "AddImpl", args = 2)]
    pub fn add_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "ScaleImpl", args = 2)]
    pub fn scale_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleParamCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleParamCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleParamCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleDistanceCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleDistanceCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleDistanceCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleDistanceCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleDistanceCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleDistanceCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleDistanceCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleArenaCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleArenaCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleArenaCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleArenaCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleArenaCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleArenaCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleArenaCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BlowDistanceCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BlowDistanceCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BlowDistanceCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BlowDistanceCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BlowDistanceCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BlowDistanceCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BlowDistanceCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleSceneResultHitCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleSceneResultHitCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleSceneResultHitCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleSceneResultHitCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleSceneResultHitCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleSceneResultHitCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleSceneResultHitCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponLevelCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponLevelCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponLevelCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponLevelCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetWeaponLevel", args = 1)]
    pub fn get_weapon_level(
        self,
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::weaponlevel::WeaponLevel_Kind;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponLevelCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponLevelCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponLevelCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleSceneResultCriticalCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitCalculator.BattleSceneResultCriticalCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleSceneResultCriticalCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleSceneResultCriticalCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleSceneResultCriticalCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleSceneResultCriticalCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleSceneResultCriticalCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_DamageCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.DamageCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_DamageCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_DamageCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_DamageCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_DamageCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_DamageCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_JobInternalLevelCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.JobInternalLevelCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_JobInternalLevelCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_JobInternalLevelCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_JobInternalLevelCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_JobInternalLevelCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_JobInternalLevelCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TotalLevelCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TotalLevelCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TotalLevelCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TotalLevelCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TotalLevelCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TotalLevelCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TotalLevelCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ProbabilityGodSkillCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ProbabilityGodSkillCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_ProbabilityCommand)]
pub struct UnitCalculator_ProbabilityGodSkillCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ProbabilityGodSkillCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCorrect", args = 1)]
    pub fn get_correct(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetCorrect", args = 1)]
    pub fn get_correct_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ProbabilityGodSkillCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ProbabilityGodSkillCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ProbabilityGodSkillCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_AvoidCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.AvoidCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_AvoidCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_AvoidCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_AvoidCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_AvoidCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_AvoidCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkGodCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkGodCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_LinkGodCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkGodCommand {
    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "GetCapability", args = 1)]
    pub fn get_capability(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::capabilityint::CapabilityInt;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkGodCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkGodCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkGodCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleStyleCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleStyleCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleStyleCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleStyleCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleStyleCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleStyleCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleStyleCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_RangeUnitCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.RangeUnitCountCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_RangeUnitCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_RangeUnitCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_RangeUnitCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_RangeUnitCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_RangeUnitCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_DropItemRatioCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.DropItemRatioCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_DropItemRatioCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_DropItemRatioCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_DropItemRatioCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_DropItemRatioCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_DropItemRatioCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SupportCriticalCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SupportCriticalCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_SupportCriticalCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SupportCriticalCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SupportCriticalCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SupportCriticalCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SupportCriticalCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SupportAvoidCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SupportAvoidCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_SupportAvoidCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SupportAvoidCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SupportAvoidCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SupportAvoidCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SupportAvoidCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ExpBattleGiveCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ExpBattleGiveCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ExpBattleGiveCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ExpBattleGiveCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ExpBattleGiveCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ExpBattleGiveCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ExpBattleGiveCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleSideCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleSideCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleSideCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleSideCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleSideCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleSideCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleSideCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ExpCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ExpCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ExpCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ExpCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ExpCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ExpCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ExpCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MoveTypeCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.MoveTypeCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_MoveTypeCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_MoveTypeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetInvalid", args = 0)]
    pub fn get_invalid(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_MoveTypeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_MoveTypeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_MoveTypeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LuukCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LuukCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_LuukCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LuukCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LuukCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LuukCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LuukCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_PosLowZCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.PosLowZCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_PosZCommand)]
pub struct UnitCalculator_PosLowZCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_PosLowZCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_PosLowZCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_PosLowZCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_PosLowZCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TotalResultCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TotalResultCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_IBattleSceneResultCommand)]
pub struct UnitCalculator_TotalResultCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TotalResultCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl_2(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TotalResultCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TotalResultCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TotalResultCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ProbabilityCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ProbabilityCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ProbabilityCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ProbabilityCommand {
    #[method(name = "GetResult", args = 2)]
    pub fn get_result(self, side: crate::app::battleside::BattleSide_Type, percent: i32) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl_2(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "GetCorrect", args = 1)]
    pub fn get_correct(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetCorrect", args = 1)]
    pub fn get_correct_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ProbabilityCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ProbabilityCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ProbabilityCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_FixedFriendCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.FixedFriendCountCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_FixedFriendCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_FixedFriendCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_FixedFriendCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_FixedFriendCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_FixedFriendCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_RangeFriendCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.RangeFriendCountCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_RangeFriendCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_RangeFriendCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_RangeFriendCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_RangeFriendCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_RangeFriendCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TotalOrderCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TotalOrderCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TotalOrderCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TotalOrderCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TotalOrderCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TotalOrderCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TotalOrderCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LevelCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LevelCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_LevelCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LevelCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LevelCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LevelCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LevelCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkDefCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkDefCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkDefCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkDefCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkDefCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkDefCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkDefCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitPersonCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitPersonCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_UnitPersonCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitPersonCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(self, unit: crate::app::unit::Unit, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitPersonCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitPersonCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitPersonCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_QuickCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.QuickCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_QuickCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_QuickCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_QuickCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_QuickCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_QuickCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ReciveAttackCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ReciveAttackCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ReciveAttackCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ReciveAttackCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ReciveAttackCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ReciveAttackCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ReciveAttackCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleSceneResultBreakCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitCalculator.BattleSceneResultBreakCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleSceneResultBreakCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleSceneResultBreakCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleSceneResultBreakCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleSceneResultBreakCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleSceneResultBreakCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_PosLowXCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.PosLowXCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_PosXCommand)]
pub struct UnitCalculator_PosLowXCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_PosLowXCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_PosLowXCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_PosLowXCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_PosLowXCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkLuukCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkLuukCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkLuukCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkLuukCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkLuukCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkLuukCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkLuukCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_InternalLevelCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.InternalLevelCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_InternalLevelCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_InternalLevelCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_InternalLevelCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_InternalLevelCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_InternalLevelCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ProbabilitySkillCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ProbabilitySkillCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_ProbabilityCommand)]
pub struct UnitCalculator_ProbabilitySkillCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ProbabilitySkillCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCorrect", args = 1)]
    pub fn get_correct(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetCorrect", args = 1)]
    pub fn get_correct_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ProbabilitySkillCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ProbabilitySkillCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ProbabilitySkillCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_GodGoodWeaponCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.GodGoodWeaponCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_GodGoodWeaponCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_GodGoodWeaponCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetValue", args = 1)]
    pub fn get_value(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_GodGoodWeaponCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_GodGoodWeaponCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_GodGoodWeaponCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SupportHitCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SupportHitCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_SupportHitCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SupportHitCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SupportHitCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SupportHitCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SupportHitCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_DefCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.DefCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_DefCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_DefCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_DefCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_DefCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_DefCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_HPCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.HPCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_HPCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_HPCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, unit: crate::app::unit::Unit, value: f32) -> ();

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_HPCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_HPCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_HPCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_JobRankCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.JobRankCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_JobRankCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_JobRankCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetInvalid", args = 0)]
    pub fn get_invalid(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_JobRankCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_JobRankCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_JobRankCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SkillCorrectCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SkillCorrectCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_SkillCorrectCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SkillCorrectCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SkillCorrectCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SkillCorrectCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SkillCorrectCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MdefCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.MdefCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_MdefCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_MdefCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_MdefCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_MdefCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_MdefCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_GainExpCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.GainExpCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_GainExpCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_GainExpCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_GainExpCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_GainExpCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_GainExpCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SimpleHitCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SimpleHitCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_SimpleHitCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SimpleHitCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SimpleHitCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SimpleHitCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SimpleHitCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_AttackAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.AttackAttributeCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_AttackAttributeCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_AttackAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_AttackAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_AttackAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_AttackAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ExpRodCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ExpRodCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ExpRodCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ExpRodCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ExpRodCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ExpRodCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ExpRodCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ExpInterferenceCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ExpInterferenceCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ExpInterferenceCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ExpInterferenceCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ExpInterferenceCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ExpInterferenceCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ExpInterferenceCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponExpendCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponExpendCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponExpendCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponExpendCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponExpendCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponExpendCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponExpendCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SimpleCriticalCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SimpleCriticalCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_SimpleCriticalCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SimpleCriticalCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SimpleCriticalCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SimpleCriticalCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SimpleCriticalCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_OverlapCanRemoveCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.OverlapCanRemoveCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_OverlapCanRemoveCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_OverlapCanRemoveCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_OverlapCanRemoveCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_OverlapCanRemoveCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_OverlapCanRemoveCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TechCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TechCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TechCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TechCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TechCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TechCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TechCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitGodCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitGodCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_UnitGodCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitGodCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(self, unit: crate::app::unit::Unit, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitGodCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitGodCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitGodCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitAroundCountdCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitAroundCountdCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_UnitAroundCountdCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitAroundCountdCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitAroundCountdCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitAroundCountdCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitAroundCountdCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SimplePowerCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SimplePowerCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_SimplePowerCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SimplePowerCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SimplePowerCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SimplePowerCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SimplePowerCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleSceneResultCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleSceneResultCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_IBattleSceneResultCommand)]
pub struct UnitCalculator_BattleSceneResultCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleSceneResultCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleSceneResultCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleSceneResultCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleSceneResultCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponSecureCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponSecureCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponSecureCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponSecureCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponSecureCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponSecureCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponSecureCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkMdefCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkMdefCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkMdefCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkMdefCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkMdefCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkMdefCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkMdefCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ActionCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ActionCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ActionCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ActionCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ActionCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ActionCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ActionCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_GenderCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.GenderCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_GenderCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_GenderCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_GenderCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_GenderCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_GenderCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkQuickCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkQuickCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkQuickCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkQuickCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkQuickCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkQuickCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkQuickCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitIdentCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitIdentCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_UnitIdentCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitIdentCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitIdentCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitIdentCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitIdentCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_RelianceAroundTotalCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.RelianceAroundTotalCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_RelianceAroundTotalCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_RelianceAroundTotalCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_RelianceAroundTotalCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_RelianceAroundTotalCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_RelianceAroundTotalCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MapUnitCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.MapUnitCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_MapUnitCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_MapUnitCountCommand {
    #[method(name = "GetTargetForceMask", args = 2)]
    pub fn get_target_force_mask(
        unit: crate::app::unit::Unit,
        candidate: crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand_Candidates,
    ) -> u32;

    #[method(name = "GetRangeCount", args = 4)]
    pub fn get_range_count(
        unit: crate::app::unit::Unit,
        range: i32,
        kind: crate::app::itemdata::ItemData_Kinds,
        candidate: crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand_Candidates,
    ) -> i32;

    #[method(name = "GetRangeCount", args = 3)]
    pub fn get_range_count_2(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
        candidate: crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand_Candidates,
    ) -> i32;

    #[method(name = "GetFixedCount", args = 2)]
    pub fn get_fixed_count(
        unit: crate::app::unit::Unit,
        candidate: crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand_Candidates,
    ) -> i32;

    #[method(name = "GetGenderCount", args = 3)]
    pub fn get_gender_count(
        unit: crate::app::unit::Unit,
        range: i32,
        gender: crate::app::gender::Gender,
    ) -> i32;

    #[method(name = "GetGenderCount", args = 4)]
    pub fn get_gender_count_2(
        unit: crate::app::unit::Unit,
        range: i32,
        gender1: crate::app::gender::Gender,
        gender2: crate::app::gender::Gender,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_MapUnitCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_MapUnitCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_MapUnitCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_IBattleSceneResultCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.IBattleSceneResultCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_IBattleSceneResultCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_IBattleSceneResultCommand {
    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(
        self,
        result: crate::app::battlescene::BattleScene_Result,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(
        self,
        result: crate::app::battlescene::BattleScene_Result,
        value: crate::app::battlescene::BattleScene_Result,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        result: crate::app::battlescene::BattleScene_Result,
        value: crate::app::battlescene::BattleScene_Result,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_IBattleSceneResultCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_IBattleSceneResultCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_IBattleSceneResultCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitSkillCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitSkillCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_UnitSkillCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitSkillCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        arg: ::unity2::Il2CppString,
    ) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl_2(self, unit: crate::app::unit::Unit, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitSkillCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitSkillCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitSkillCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkMagicCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkMagicCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkMagicCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkMagicCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkMagicCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkMagicCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkMagicCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponWeightCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponWeightCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponWeightCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponWeightCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetWeight", args = 1)]
    pub fn get_weight(self, item: crate::app::itemdata::ItemData) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponWeightCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponWeightCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponWeightCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ContinuousCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ContinuousCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_ContinuousCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ContinuousCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ContinuousCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ContinuousCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ContinuousCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_GodEngagingCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.GodEngagingCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_GodEngagingCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_GodEngagingCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetValue", args = 1)]
    pub fn get_value(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_GodEngagingCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_GodEngagingCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_GodEngagingCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleChainGuardCountCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitCalculator.BattleChainGuardCountCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleChainGuardCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleChainGuardCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleChainGuardCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleChainGuardCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleChainGuardCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_GodSkillCorrectCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.GodSkillCorrectCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_GodSkillCorrectCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_GodSkillCorrectCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_GodSkillCorrectCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_GodSkillCorrectCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_GodSkillCorrectCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_DefenseCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.DefenseCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_DefenseCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_DefenseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_DefenseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_DefenseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_DefenseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SecureCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SecureCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_SecureCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SecureCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SecureCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SecureCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SecureCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleSceneResultMissCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitCalculator.BattleSceneResultMissCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleSceneResultMissCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleSceneResultMissCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleSceneResultMissCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleSceneResultMissCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleSceneResultMissCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ExpDestroyCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ExpDestroyCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ExpDestroyCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ExpDestroyCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ExpDestroyCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ExpDestroyCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ExpDestroyCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TerrainAvoidCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TerrainAvoidCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TerrainAvoidCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TerrainAvoidCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TerrainAvoidCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TerrainAvoidCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TerrainAvoidCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_HealCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.HealCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_HealCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_HealCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_HealCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_HealCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_HealCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TotalDamageCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TotalDamageCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TotalDamageCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TotalDamageCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TotalDamageCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TotalDamageCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TotalDamageCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleChainAttackDefeatCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitCalculator.BattleChainAttackDefeatCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleChainAttackDefeatCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleChainAttackDefeatCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleChainAttackDefeatCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleChainAttackDefeatCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleChainAttackDefeatCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponAttackCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponAttackCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponAttackCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponAttackCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponAttackCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponAttackCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponAttackCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitJobCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitJobCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_UnitJobCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitJobCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(self, unit: crate::app::unit::Unit, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitJobCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitJobCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitJobCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_AroundGenderCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.AroundGenderCountCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_AroundGenderCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_AroundGenderCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_AroundGenderCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_AroundGenderCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_AroundGenderCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TerrainDefenseCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TerrainDefenseCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TerrainDefenseCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TerrainDefenseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TerrainDefenseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TerrainDefenseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TerrainDefenseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_PhysCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.PhysCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_PhysCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_PhysCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_PhysCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_PhysCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_PhysCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponKindCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponKindCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponKindCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponKindCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetKind", args = 1)]
    pub fn get_kind(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponKindCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponKindCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponKindCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponInteractCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponInteractCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponInteractCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponInteractCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponInteractCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponInteractCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponInteractCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitDefenseCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitDefenseCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_UnitDefenseCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitDefenseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitDefenseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitDefenseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitDefenseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MagicCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.MagicCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_MagicCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_MagicCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_MagicCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_MagicCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_MagicCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_PickupItemCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.PickupItemCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_PickupItemCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_PickupItemCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_PickupItemCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_PickupItemCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_PickupItemCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattlePinchingCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattlePinchingCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattlePinchingCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattlePinchingCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattlePinchingCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattlePinchingCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattlePinchingCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitAttributeCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_UnitAttributeCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_FixedEnemyCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.FixedEnemyCountCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_FixedEnemyCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_FixedEnemyCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_FixedEnemyCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_FixedEnemyCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_FixedEnemyCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BlowRatioCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BlowRatioCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BlowRatioCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BlowRatioCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BlowRatioCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BlowRatioCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BlowRatioCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponEfficacyCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponEfficacyCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponEfficacyCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponEfficacyCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponEfficacyCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponEfficacyCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponEfficacyCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_RodTypeCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.RodTypeCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_RodTypeCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_RodTypeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetKind", args = 1)]
    pub fn get_kind(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_RodTypeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_RodTypeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_RodTypeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_PosZCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.PosZCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_PosZCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_PosZCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetInvalid", args = 0)]
    pub fn get_invalid(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_PosZCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_PosZCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_PosZCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_AttackCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.AttackCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_AttackCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_AttackCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_AttackCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_AttackCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_AttackCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_EngageCountLimitCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.EngageCountLimitCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_EngageCountLimitCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_EngageCountLimitCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_EngageCountLimitCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_EngageCountLimitCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_EngageCountLimitCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_CriticalCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.CriticalCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_CriticalCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_CriticalCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_CriticalCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_CriticalCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_CriticalCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleDeadCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleDeadCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleDeadCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleDeadCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleDeadCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleDeadCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleDeadCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ExpDanceCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ExpDanceCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ExpDanceCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ExpDanceCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ExpDanceCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ExpDanceCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ExpDanceCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_AttackCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.AttackCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_AttackCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_AttackCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_AttackCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_AttackCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_AttackCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponCriticalCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponCriticalCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponCriticalCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponCriticalCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponCriticalCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponCriticalCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponCriticalCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TotalAttackCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TotalAttackCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TotalAttackCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TotalAttackCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TotalAttackCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TotalAttackCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TotalAttackCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MapPhaseCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.MapPhaseCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_MapPhaseCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_MapPhaseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetInvalid", args = 0)]
    pub fn get_invalid(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_MapPhaseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_MapPhaseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_MapPhaseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleReviveCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleReviveCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleReviveCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleReviveCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleReviveCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleReviveCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleReviveCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleChainAttackCountCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitCalculator.BattleChainAttackCountCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleChainAttackCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleChainAttackCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleChainAttackCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleChainAttackCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleChainAttackCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_ForceCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.ForceCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_ForceCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_ForceCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetInvalid", args = 0)]
    pub fn get_invalid(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_ForceCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_ForceCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_ForceCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponAttributeCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponAttributeCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_SupportSecureCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.SupportSecureCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_SupportSecureCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_SupportSecureCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_SupportSecureCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_SupportSecureCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_SupportSecureCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkStrCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkStrCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkStrCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkStrCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkStrCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkStrCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkStrCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleLastTargetCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleLastTargetCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleLastTargetCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleLastTargetCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleLastTargetCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleLastTargetCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleLastTargetCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_UnitAttackCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.UnitAttackCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_BattleParamCommand)]
pub struct UnitCalculator_UnitAttackCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_UnitAttackCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleparam::BattleParam;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_UnitAttackCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_UnitAttackCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_UnitAttackCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MaxHPCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.MaxHPCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_MaxHPCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_MaxHPCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_MaxHPCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_MaxHPCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_MaxHPCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_EngageCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.EngageCountCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_EngageCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_EngageCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, unit: crate::app::unit::Unit, value: f32) -> ();

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_EngageCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_EngageCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_EngageCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_RodExpCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.RodExpCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_RodExpCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_RodExpCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_RodExpCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_RodExpCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_RodExpCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_PosXCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.PosXCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_PosXCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_PosXCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetInvalid", args = 0)]
    pub fn get_invalid(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_PosXCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_PosXCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_PosXCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TotalActionCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TotalActionCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TotalActionCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TotalActionCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TotalActionCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TotalActionCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TotalActionCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_RangeEnemyCountCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.RangeEnemyCountCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_MapUnitCountCommand)]
pub struct UnitCalculator_RangeEnemyCountCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_RangeEnemyCountCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_RangeEnemyCountCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_RangeEnemyCountCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_RangeEnemyCountCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleTimesCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.BattleTimesCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleTimesCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleTimesCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleTimesCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleTimesCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleTimesCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_BattleSceneResultEfficacyCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "UnitCalculator.BattleSceneResultEfficacyCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_BattleSceneResultEfficacyCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_BattleSceneResultEfficacyCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_BattleSceneResultEfficacyCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_BattleSceneResultEfficacyCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_BattleSceneResultEfficacyCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MapUnitCountCommand_Candidates.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitCalculator_MapUnitCountCommand_Candidates {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitCalculator_MapUnitCountCommand_Candidates {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitCalculator.MapUnitCountCommand.Candidates";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitCalculator_MapUnitCountCommand_Candidates {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitCalculator_MapUnitCountCommand_Candidates {
    pub fn friend() -> Self {
        Self { value: 0 }
    }

    pub fn enemy() -> Self {
        Self { value: 1 }
    }

    pub fn whole() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_StrCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.StrCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_StrCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_StrCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_StrCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_StrCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_StrCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_MoveDistanceCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.MoveDistanceCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_MoveDistanceCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_MoveDistanceCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_MoveDistanceCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_MoveDistanceCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_MoveDistanceCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_LinkTechCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.LinkTechCommand")]
#[parent(crate::app::unitcalculator::UnitCalculator_LinkGodCommand)]
pub struct UnitCalculator_LinkTechCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_LinkTechCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_CapabilityType", args = 0)]
    pub fn get_capability_type(self)
        -> crate::app::capabilitydefinition::CapabilityDefinition_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_LinkTechCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_LinkTechCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_LinkTechCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_TemporaryCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.TemporaryCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_TemporaryCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_TemporaryCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, side: crate::app::battleinfoside::BattleInfoSide, value: f32) -> ();

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl_2(self, unit: crate::app::unit::Unit, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_TemporaryCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_TemporaryCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_TemporaryCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitcalculator/UnitCalculator_WeaponLevelBaseCommand.md")))]
#[::unity2::class(namespace = "App", name = "UnitCalculator.WeaponLevelBaseCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct UnitCalculator_WeaponLevelBaseCommand {}

#[cfg(feature = "app-unitcalculator")]
#[::unity2::methods]
impl UnitCalculator_WeaponLevelBaseCommand {
    #[method(name = "GetUnitWeaponLevel", args = 2)]
    pub fn get_unit_weapon_level(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> crate::app::weaponlevel::WeaponLevel_Kind;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, side: crate::app::battleinfoside::BattleInfoSide) -> f32;

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl_2(self, unit: crate::app::unit::Unit) -> f32;

    #[method(name = "GetValue", args = 1)]
    pub fn get_value(self, level: crate::app::weaponlevel::WeaponLevel_Kind) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitcalculator")]
impl UnitCalculator_WeaponLevelBaseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitCalculator_WeaponLevelBaseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitCalculator_WeaponLevelBaseCommandMethods>::ctor(this);
        this
    }
}
