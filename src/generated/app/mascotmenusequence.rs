
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotmenusequence/MascotMenuSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MascotMenuSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MascotMenuSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MascotMenuSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MascotMenuSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MascotMenuSequence_Label {
    pub fn load() -> Self {
        Self { value: 0 }
    }

    pub fn main() -> Self {
        Self { value: 1 }
    }

    pub fn food() -> Self {
        Self { value: 2 }
    }

    pub fn food_eat() -> Self {
        Self { value: 3 }
    }

    pub fn strok() -> Self {
        Self { value: 4 }
    }

    pub fn custom() -> Self {
        Self { value: 5 }
    }

    pub fn custom_top() -> Self {
        Self { value: 6 }
    }

    pub fn custom_parts() -> Self {
        Self { value: 7 }
    }

    pub fn custom_color() -> Self {
        Self { value: 8 }
    }

    pub fn exit() -> Self {
        Self { value: 9 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotmenusequence/MascotMenuSequence.md")))]
#[::unity2::class(namespace = "App", name = "MascotMenuSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mascotmenusequence :: MascotMenuSequence >)]
pub struct MascotMenuSequence {
    #[rename(name = "m_FriendlyGauge")]
    pub m_friendly_gauge: crate::app::mascotfriendlycontent::MascotFriendlyContent,
    #[rename(name = "m_MascotPresentationRoot")]
    pub m_mascot_presentation_root: crate::app::mascotpresentationroot::MascotPresentationRoot,
    #[rename(name = "m_TopMenuResult")]
    pub m_top_menu_result: crate::app::mascottopmenu::MascotTopMenu_MenuResult,
    #[rename(name = "m_CustomMenuResult")]
    pub m_custom_menu_result: crate::app::mascotcustomizemenu::MascotCustomizeMenu_MenuResult,
    #[rename(name = "m_Handle")]
    pub m_handle: crate::app::resourcehandle_2::ResourceHandle_2,
    #[rename(name = "m_Reserved")]
    pub m_reserved: bool,
    #[rename(name = "m_MascotReservePosition")]
    pub m_mascot_reserve_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_MascotReserveRotation")]
    pub m_mascot_reserve_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_MascotCamera")]
    pub m_mascot_camera: crate::app::hubmascotcamera::HubMascotCamera,
}

#[cfg(feature = "app-mascotmenusequence")]
#[::unity2::methods]
impl MascotMenuSequence {
    #[method(name = "get_Foodstuff", args = 0)]
    pub fn get_foodstuff(self) -> crate::app::foodstuffdata::FoodstuffData;

    #[method(name = "set_Foodstuff", args = 1)]
    pub fn set_foodstuff(self, value: crate::app::foodstuffdata::FoodstuffData) -> ();

    #[method(name = "get_FriendlyGauge", args = 0)]
    pub fn get_friendly_gauge(self) -> crate::app::mascotfriendlycontent::MascotFriendlyContent;

    #[method(name = "get_PresentationRoot", args = 0)]
    pub fn get_presentation_root(
        self,
    ) -> crate::app::mascotpresentationroot::MascotPresentationRoot;

    #[method(name = "get_PlayerController", args = 0)]
    pub fn get_player_controller(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubunitcontroller::HubUnitController;

    #[method(name = "get_Mascot", args = 0)]
    pub fn get_mascot(self) -> crate::app::hubmascotcontroller::HubMascotController;

    #[method(name = "SetShadowModel", args = 2)]
    pub fn set_shadow_model(
        game_object: crate::unity_engine::gameobject::GameObject,
        enabled: bool,
    ) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "OpenTitle", args = 0)]
    pub fn open_title(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "CreateCustomMenu", args = 0)]
    pub fn create_custom_menu(self) -> ();

    #[method(name = "InitStrok", args = 0)]
    pub fn init_strok(self) -> ();

    #[method(name = "Strok", args = 0)]
    pub fn strok(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ExitStrok", args = 0)]
    pub fn exit_strok(self) -> ();

    #[method(name = "GetBond", args = 0)]
    pub fn get_bond(self) -> ();

    #[method(name = "InitCustom", args = 0)]
    pub fn init_custom(self) -> ();

    #[method(name = "ExitCustom", args = 0)]
    pub fn exit_custom(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateFoodSelect", args = 0)]
    pub fn create_food_select(self) -> ();

    #[method(name = "CreateFoodEat", args = 0)]
    pub fn create_food_eat(self) -> ();

    #[method(name = "CreateAccChange", args = 0)]
    pub fn create_acc_change(self) -> ();

    #[method(name = "CreateColorChange", args = 0)]
    pub fn create_color_change(self) -> ();

    #[method(name = "FriendlyPopup", args = 0)]
    pub fn friendly_popup(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mascotmenusequence")]
impl MascotMenuSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotMenuSequenceMethods>::ctor(this);
        this
    }
}
