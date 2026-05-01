
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubinvestmentbonusfood/HubInvestmentBonusFood.md")))]
#[::unity2::class(namespace = "App", name = "HubInvestmentBonusFood")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: hubinvestmentbonusfood :: HubInvestmentBonusFood >)]
pub struct HubInvestmentBonusFood {}

#[cfg(feature = "app-hubinvestmentbonusfood")]
#[::unity2::methods]
impl HubInvestmentBonusFood {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Foodstuff", args = 0)]
    pub fn get_foodstuff(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Foodstuff", args = 1)]
    pub fn set_foodstuff(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Num", args = 0)]
    pub fn get_num(self) -> u8;

    #[method(name = "set_Num", args = 1)]
    pub fn set_num(self, value: u8) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubinvestmentbonusfood")]
impl HubInvestmentBonusFood {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubInvestmentBonusFood),
                ::core::stringify!(new),
            )
        });
        <Self as IHubInvestmentBonusFoodMethods>::ctor(this);
        this
    }
}
