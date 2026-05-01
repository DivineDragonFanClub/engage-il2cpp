
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotfoodeatsequence/MascotFoodEatSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MascotFoodEatSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MascotFoodEatSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MascotFoodEatSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MascotFoodEatSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MascotFoodEatSequence_Label {
    pub fn eat_food() -> Self {
        Self { value: 0 }
    }

    pub fn exit() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotfoodeatsequence/MascotFoodEatSequence.md")))]
#[::unity2::class(namespace = "App", name = "MascotFoodEatSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: mascotfoodeatsequence :: MascotFoodEatSequence >)]
pub struct MascotFoodEatSequence {
    #[rename(name = "m_foodStuffData")]
    pub m_food_stuff_data: crate::app::foodstuffdata::FoodstuffData,
}

#[cfg(feature = "app-mascotfoodeatsequence")]
#[::unity2::methods]
impl MascotFoodEatSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, foodstuff_data: crate::app::foodstuffdata::FoodstuffData) -> ();

    #[method(name = "get_PlayerController", args = 0)]
    pub fn get_player_controller(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubunitcontroller::HubUnitController;

    #[method(name = "get_Mascot", args = 0)]
    pub fn get_mascot(self) -> crate::app::hubmascotcontroller::HubMascotController;

    #[method(name = "InitEatFood", args = 0)]
    pub fn init_eat_food(self) -> ();

    #[method(name = "EatFood", args = 0)]
    pub fn eat_food(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ExitEatFood", args = 0)]
    pub fn exit_eat_food(self) -> ();

    #[method(name = "GetBond", args = 0)]
    pub fn get_bond(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        foodstuff_data: crate::app::foodstuffdata::FoodstuffData,
    ) -> ();
}

#[cfg(feature = "app-mascotfoodeatsequence")]
impl MascotFoodEatSequence {
    pub fn new(foodstuff_data: crate::app::foodstuffdata::FoodstuffData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotFoodEatSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotFoodEatSequenceMethods>::ctor(this, foodstuff_data);
        this
    }
}
