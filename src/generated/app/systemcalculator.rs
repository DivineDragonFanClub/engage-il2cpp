
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::app::calculatormanager::CalculatorManager;
use crate::app::calculatormanager::ICalculatorManager;
use crate::app::calculatorutil::CalculatorUtil;
use crate::app::calculatorutil::ICalculatorUtil;
use crate::app::gamecalculatorcommand::GameCalculatorCommand;
use crate::app::gamecalculatorcommand::IGameCalculatorCommand;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_DifficultyHardCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.DifficultyHardCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_DifficultyHardCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_DifficultyHardCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_DifficultyHardCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_DifficultyHardCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_DifficultyHardCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_IceAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.IceAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_IceAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_IceAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_IceAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_IceAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_IceAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_PhaseOtherCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.PhaseOtherCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct SystemCalculator_PhaseOtherCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_PhaseOtherCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_PhaseOtherCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_PhaseOtherCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_PhaseOtherCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_WalkAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.WalkAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_WalkAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_WalkAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_WalkAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_WalkAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_WalkAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_SkillIndexCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.SkillIndexCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_SkillIndexCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_SkillIndexCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 1)]
    pub fn func(self, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_SkillIndexCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_SkillIndexCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_SkillIndexCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_RodTypeBasicCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.RodTypeBasicCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_RodTypeBasicCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_RodTypeBasicCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_RodTypeBasicCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_RodTypeBasicCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_RodTypeBasicCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_WeaponLevelACommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.WeaponLevelACommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_WeaponLevelACommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_WeaponLevelACommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_WeaponLevelACommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_WeaponLevelACommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_WeaponLevelACommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_WeaponLevelSCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.WeaponLevelSCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_WeaponLevelSCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_WeaponLevelSCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_WeaponLevelSCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_WeaponLevelSCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_WeaponLevelSCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_FireAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.FireAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_FireAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_FireAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_FireAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_FireAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_FireAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_RodTypeInterferenceCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SystemCalculator.RodTypeInterferenceCommand"
)]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_RodTypeInterferenceCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_RodTypeInterferenceCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_RodTypeInterferenceCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_RodTypeInterferenceCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_RodTypeInterferenceCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ThunderAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ThunderAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ThunderAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ThunderAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ThunderAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ThunderAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ThunderAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_VsyncCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.VsyncCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_VsyncCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_VsyncCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 0)]
    pub fn func(self) -> f32;

    #[method(name = "Func", args = 1)]
    pub fn func_2(self, args: crate::system::collections::generic::list_1::List_1<f32>) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_VsyncCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_VsyncCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_VsyncCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_Force3rdCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.Force3rdCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_Force3rdCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_Force3rdCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_Force3rdCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_Force3rdCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_Force3rdCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleFlyCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStyleFlyCommand")]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStyleFlyCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleFlyCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleFlyCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleFlyCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleFlyCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_WeaponLevelCCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.WeaponLevelCCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_WeaponLevelCCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_WeaponLevelCCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_WeaponLevelCCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_WeaponLevelCCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_WeaponLevelCCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_GenderFemaleCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.GenderFemaleCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_GenderFemaleCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_GenderFemaleCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_GenderFemaleCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_GenderFemaleCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_GenderFemaleCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindBowCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindBowCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindBowCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindBowCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindBowCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindBowCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindBowCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_DifficultyLunaticCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.DifficultyLunaticCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_DifficultyLunaticCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_DifficultyLunaticCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_DifficultyLunaticCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_DifficultyLunaticCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_DifficultyLunaticCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleDragonCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStyleDragonCommand")]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStyleDragonCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleDragonCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleDragonCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleDragonCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleDragonCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_GameModeClassicCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.GameModeClassicCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_GameModeClassicCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_GameModeClassicCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_GameModeClassicCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_GameModeClassicCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_GameModeClassicCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_MoveCostCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.MoveCostCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_MoveCostCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_MoveCostCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ArgNum", args = 0)]
    pub fn get_arg_num(self) -> i32;

    #[method(name = "Func", args = 1)]
    pub fn func(self, args: crate::system::collections::generic::list_1::List_1<f32>) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_MoveCostCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_MoveCostCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_MoveCostCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_WindAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.WindAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_WindAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_WindAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_WindAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_WindAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_WindAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_GameModeCasualCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.GameModeCasualCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_GameModeCasualCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_GameModeCasualCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_GameModeCasualCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_GameModeCasualCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_GameModeCasualCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_WeaponLevelBCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.WeaponLevelBCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_WeaponLevelBCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_WeaponLevelBCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_WeaponLevelBCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_WeaponLevelBCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_WeaponLevelBCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_JobRankLowCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.JobRankLowCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_JobRankLowCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_JobRankLowCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_JobRankLowCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_JobRankLowCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_JobRankLowCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_MorphAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.MorphAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_MorphAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_MorphAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_MorphAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_MorphAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_MorphAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleSideChainDefenseCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SystemCalculator.BattleSideChainDefenseCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct SystemCalculator_BattleSideChainDefenseCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleSideChainDefenseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleSideChainDefenseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleSideChainDefenseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleSideChainDefenseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_PhysicalAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.PhysicalAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_PhysicalAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_PhysicalAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_PhysicalAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_PhysicalAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_PhysicalAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_NoneAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.NoneAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_NoneAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_NoneAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_NoneAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_NoneAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_NoneAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindAxeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindAxeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindAxeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindAxeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindAxeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindAxeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindAxeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_PhaseCurrentCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.PhaseCurrentCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct SystemCalculator_PhaseCurrentCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_PhaseCurrentCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_PhaseCurrentCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_PhaseCurrentCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_PhaseCurrentCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleHeavyCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStyleHeavyCommand")]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStyleHeavyCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleHeavyCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleHeavyCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleHeavyCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleHeavyCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator")]
#[parent(crate::app::calculatormanager::CalculatorManager)]
pub struct SystemCalculator {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculatorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindRodCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindRodCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindRodCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindRodCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindRodCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindRodCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindRodCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BadCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BadCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_BadCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BadCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BadCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BadCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BadCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_JobRankHighCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.JobRankHighCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_JobRankHighCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_JobRankHighCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_JobRankHighCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_JobRankHighCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_JobRankHighCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_EvilAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.EvilAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_EvilAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_EvilAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_EvilAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_EvilAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_EvilAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_HeavyAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.HeavyAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_HeavyAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_HeavyAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_HeavyAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_HeavyAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_HeavyAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_FlyAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.FlyAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_FlyAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_FlyAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_FlyAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_FlyAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_FlyAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemIndexCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemIndexCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemIndexCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemIndexCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 1)]
    pub fn func(self, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemIndexCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemIndexCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemIndexCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleMagicCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStyleMagicCommand")]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStyleMagicCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleMagicCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleMagicCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleMagicCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleMagicCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleSideOffenseCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleSideOffenseCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct SystemCalculator_BattleSideOffenseCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleSideOffenseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleSideOffenseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleSideOffenseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleSideOffenseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleSideChainOffenseCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SystemCalculator.BattleSideChainOffenseCommand"
)]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct SystemCalculator_BattleSideChainOffenseCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleSideChainOffenseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleSideChainOffenseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleSideChainOffenseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleSideChainOffenseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindSpecialCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindSpecialCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindSpecialCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindSpecialCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindSpecialCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindSpecialCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindSpecialCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindLanceCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindLanceCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindLanceCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindLanceCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindLanceCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindLanceCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindLanceCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindSwordCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindSwordCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindSwordCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindSwordCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindSwordCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindSwordCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindSwordCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ScreenShotCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ScreenShotCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ScreenShotCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ScreenShotCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ScreenShotCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ScreenShotCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ScreenShotCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStyleCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_BattleStyleCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_DifficultyNormalCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.DifficultyNormalCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_DifficultyNormalCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_DifficultyNormalCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_DifficultyNormalCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_DifficultyNormalCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_DifficultyNormalCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_JobIndexCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.JobIndexCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_JobIndexCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_JobIndexCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 1)]
    pub fn func(self, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_JobIndexCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_JobIndexCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_JobIndexCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_Force2ndCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.Force2ndCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_Force2ndCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_Force2ndCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_Force2ndCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_Force2ndCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_Force2ndCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleCooperationCommand.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SystemCalculator.BattleStyleCooperationCommand"
)]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStyleCooperationCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleCooperationCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleCooperationCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleCooperationCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleCooperationCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindMagicCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindMagicCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindMagicCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindMagicCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindMagicCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindMagicCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindMagicCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_GenderMaleCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.GenderMaleCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_GenderMaleCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_GenderMaleCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_GenderMaleCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_GenderMaleCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_GenderMaleCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_Force1stCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.Force1stCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_Force1stCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_Force1stCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_Force1stCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_Force1stCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_Force1stCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_DragonAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.DragonAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_DragonAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_DragonAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_DragonAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_DragonAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_DragonAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_MagicAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.MagicAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_MagicAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_MagicAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_MagicAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_MagicAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_MagicAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleHorseCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStyleHorseCommand")]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStyleHorseCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleHorseCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleHorseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleHorseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleHorseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_WeaponLevelDCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.WeaponLevelDCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_WeaponLevelDCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_WeaponLevelDCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_WeaponLevelDCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_WeaponLevelDCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_WeaponLevelDCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_RodTypeHealCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.RodTypeHealCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_RodTypeHealCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_RodTypeHealCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_RodTypeHealCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_RodTypeHealCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_RodTypeHealCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_GoodCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.GoodCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_GoodCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_GoodCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_GoodCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_GoodCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_GoodCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindDaggerCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindDaggerCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindDaggerCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindDaggerCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindDaggerCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindDaggerCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindDaggerCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_DarkAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.DarkAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_DarkAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_DarkAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_DarkAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_DarkAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_DarkAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStylePranaCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStylePranaCommand")]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStylePranaCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStylePranaCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStylePranaCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStylePranaCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStylePranaCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_PersonIndexCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.PersonIndexCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_PersonIndexCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_PersonIndexCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Func", args = 1)]
    pub fn func(self, arg: ::unity2::Il2CppString) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_PersonIndexCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_PersonIndexCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_PersonIndexCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_HorseAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.HorseAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_HorseAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_HorseAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_HorseAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_HorseAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_HorseAttributeCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_ItemKindFistCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.ItemKindFistCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_ItemKindFistCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_ItemKindFistCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_ItemKindFistCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_ItemKindFistCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_ItemKindFistCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleSideDefenseCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleSideDefenseCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct SystemCalculator_BattleSideDefenseCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleSideDefenseCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleSideDefenseCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleSideDefenseCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleSideDefenseCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_BattleStyleCovertCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.BattleStyleCovertCommand")]
#[parent(crate::app::systemcalculator::SystemCalculator_BattleStyleCommand)]
pub struct SystemCalculator_BattleStyleCovertCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_BattleStyleCovertCommand {
    #[method(name = "get_Style", args = 0)]
    pub fn get_style(self) -> crate::app::battlestyle::BattleStyle_Types;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_BattleStyleCovertCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_BattleStyleCovertCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_BattleStyleCovertCommandMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/systemcalculator/SystemCalculator_LightAttributeCommand.md")))]
#[::unity2::class(namespace = "App", name = "SystemCalculator.LightAttributeCommand")]
#[parent(crate::app::calculatorcommand::CalculatorCommand)]
pub struct SystemCalculator_LightAttributeCommand {}

#[cfg(feature = "app-systemcalculator")]
#[::unity2::methods]
impl SystemCalculator_LightAttributeCommand {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-systemcalculator")]
impl SystemCalculator_LightAttributeCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SystemCalculator_LightAttributeCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ISystemCalculator_LightAttributeCommandMethods>::ctor(this);
        this
    }
}
