
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animaldata/AnimalData.md")))]
#[::unity2::class(namespace = "App", name = "AnimalData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: animaldata :: AnimalData >)]
pub struct AnimalData {
    #[rename(name = "Items")]
    pub items: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_catchNumFlagName")]
    pub m_catch_num_flag_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-animaldata")]
#[::unity2::methods]
impl AnimalData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetByPID", args = 1)]
    pub fn get_by_pid(pid: ::unity2::Il2CppString) -> crate::app::animaldata::AnimalData;

    #[method(name = "GetByANID", args = 1)]
    pub fn get_by_anid(anid: ::unity2::Il2CppString) -> crate::app::animaldata::AnimalData;

    #[method(name = "get_ANID", args = 0)]
    pub fn get_anid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ANID", args = 1)]
    pub fn set_anid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IconName", args = 0)]
    pub fn get_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IconName", args = 1)]
    pub fn set_icon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Category", args = 0)]
    pub fn get_category(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Radius", args = 0)]
    pub fn get_radius(self) -> f32;

    #[method(name = "set_Radius", args = 1)]
    pub fn set_radius(self, value: f32) -> ();

    #[method(name = "get_NID", args = 0)]
    pub fn get_nid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_NID", args = 1)]
    pub fn set_nid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PID", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PID", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Item", args = 0)]
    pub fn get_item(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Item", args = 1)]
    pub fn set_item(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Rare", args = 0)]
    pub fn get_rare(self) -> bool;

    #[method(name = "set_Rare", args = 1)]
    pub fn set_rare(self, value: bool) -> ();

    #[method(name = "get_M001", args = 0)]
    pub fn get_m001(self) -> bool;

    #[method(name = "set_M001", args = 1)]
    pub fn set_m001(self, value: bool) -> ();

    #[method(name = "get_M002", args = 0)]
    pub fn get_m002(self) -> bool;

    #[method(name = "set_M002", args = 1)]
    pub fn set_m002(self, value: bool) -> ();

    #[method(name = "get_M003", args = 0)]
    pub fn get_m003(self) -> bool;

    #[method(name = "set_M003", args = 1)]
    pub fn set_m003(self, value: bool) -> ();

    #[method(name = "get_M004", args = 0)]
    pub fn get_m004(self) -> bool;

    #[method(name = "set_M004", args = 1)]
    pub fn set_m004(self, value: bool) -> ();

    #[method(name = "get_M005", args = 0)]
    pub fn get_m005(self) -> bool;

    #[method(name = "set_M005", args = 1)]
    pub fn set_m005(self, value: bool) -> ();

    #[method(name = "get_M006", args = 0)]
    pub fn get_m006(self) -> bool;

    #[method(name = "set_M006", args = 1)]
    pub fn set_m006(self, value: bool) -> ();

    #[method(name = "get_M007", args = 0)]
    pub fn get_m007(self) -> bool;

    #[method(name = "set_M007", args = 1)]
    pub fn set_m007(self, value: bool) -> ();

    #[method(name = "get_M008", args = 0)]
    pub fn get_m008(self) -> bool;

    #[method(name = "set_M008", args = 1)]
    pub fn set_m008(self, value: bool) -> ();

    #[method(name = "get_M009", args = 0)]
    pub fn get_m009(self) -> bool;

    #[method(name = "set_M009", args = 1)]
    pub fn set_m009(self, value: bool) -> ();

    #[method(name = "get_M010", args = 0)]
    pub fn get_m010(self) -> bool;

    #[method(name = "set_M010", args = 1)]
    pub fn set_m010(self, value: bool) -> ();

    #[method(name = "get_M011", args = 0)]
    pub fn get_m011(self) -> bool;

    #[method(name = "set_M011", args = 1)]
    pub fn set_m011(self, value: bool) -> ();

    #[method(name = "get_M012", args = 0)]
    pub fn get_m012(self) -> bool;

    #[method(name = "set_M012", args = 1)]
    pub fn set_m012(self, value: bool) -> ();

    #[method(name = "get_M013", args = 0)]
    pub fn get_m013(self) -> bool;

    #[method(name = "set_M013", args = 1)]
    pub fn set_m013(self, value: bool) -> ();

    #[method(name = "get_M014", args = 0)]
    pub fn get_m014(self) -> bool;

    #[method(name = "set_M014", args = 1)]
    pub fn set_m014(self, value: bool) -> ();

    #[method(name = "get_M015", args = 0)]
    pub fn get_m015(self) -> bool;

    #[method(name = "set_M015", args = 1)]
    pub fn set_m015(self, value: bool) -> ();

    #[method(name = "get_M016", args = 0)]
    pub fn get_m016(self) -> bool;

    #[method(name = "set_M016", args = 1)]
    pub fn set_m016(self, value: bool) -> ();

    #[method(name = "get_M017", args = 0)]
    pub fn get_m017(self) -> bool;

    #[method(name = "set_M017", args = 1)]
    pub fn set_m017(self, value: bool) -> ();

    #[method(name = "get_M018", args = 0)]
    pub fn get_m018(self) -> bool;

    #[method(name = "set_M018", args = 1)]
    pub fn set_m018(self, value: bool) -> ();

    #[method(name = "get_M019", args = 0)]
    pub fn get_m019(self) -> bool;

    #[method(name = "set_M019", args = 1)]
    pub fn set_m019(self, value: bool) -> ();

    #[method(name = "get_M020", args = 0)]
    pub fn get_m020(self) -> bool;

    #[method(name = "set_M020", args = 1)]
    pub fn set_m020(self, value: bool) -> ();

    #[method(name = "get_M021", args = 0)]
    pub fn get_m021(self) -> bool;

    #[method(name = "set_M021", args = 1)]
    pub fn set_m021(self, value: bool) -> ();

    #[method(name = "get_M022", args = 0)]
    pub fn get_m022(self) -> bool;

    #[method(name = "set_M022", args = 1)]
    pub fn set_m022(self, value: bool) -> ();

    #[method(name = "get_M023", args = 0)]
    pub fn get_m023(self) -> bool;

    #[method(name = "set_M023", args = 1)]
    pub fn set_m023(self, value: bool) -> ();

    #[method(name = "get_M024", args = 0)]
    pub fn get_m024(self) -> bool;

    #[method(name = "set_M024", args = 1)]
    pub fn set_m024(self, value: bool) -> ();

    #[method(name = "get_M025", args = 0)]
    pub fn get_m025(self) -> bool;

    #[method(name = "set_M025", args = 1)]
    pub fn set_m025(self, value: bool) -> ();

    #[method(name = "get_M026", args = 0)]
    pub fn get_m026(self) -> bool;

    #[method(name = "set_M026", args = 1)]
    pub fn set_m026(self, value: bool) -> ();

    #[method(name = "get_S001", args = 0)]
    pub fn get_s001(self) -> bool;

    #[method(name = "set_S001", args = 1)]
    pub fn set_s001(self, value: bool) -> ();

    #[method(name = "get_S002", args = 0)]
    pub fn get_s002(self) -> bool;

    #[method(name = "set_S002", args = 1)]
    pub fn set_s002(self, value: bool) -> ();

    #[method(name = "get_S003", args = 0)]
    pub fn get_s003(self) -> bool;

    #[method(name = "set_S003", args = 1)]
    pub fn set_s003(self, value: bool) -> ();

    #[method(name = "get_S004", args = 0)]
    pub fn get_s004(self) -> bool;

    #[method(name = "set_S004", args = 1)]
    pub fn set_s004(self, value: bool) -> ();

    #[method(name = "get_S005", args = 0)]
    pub fn get_s005(self) -> bool;

    #[method(name = "set_S005", args = 1)]
    pub fn set_s005(self, value: bool) -> ();

    #[method(name = "get_S006", args = 0)]
    pub fn get_s006(self) -> bool;

    #[method(name = "set_S006", args = 1)]
    pub fn set_s006(self, value: bool) -> ();

    #[method(name = "get_S007", args = 0)]
    pub fn get_s007(self) -> bool;

    #[method(name = "set_S007", args = 1)]
    pub fn set_s007(self, value: bool) -> ();

    #[method(name = "get_S008", args = 0)]
    pub fn get_s008(self) -> bool;

    #[method(name = "set_S008", args = 1)]
    pub fn set_s008(self, value: bool) -> ();

    #[method(name = "get_S009", args = 0)]
    pub fn get_s009(self) -> bool;

    #[method(name = "set_S009", args = 1)]
    pub fn set_s009(self, value: bool) -> ();

    #[method(name = "get_S010", args = 0)]
    pub fn get_s010(self) -> bool;

    #[method(name = "set_S010", args = 1)]
    pub fn set_s010(self, value: bool) -> ();

    #[method(name = "get_S011", args = 0)]
    pub fn get_s011(self) -> bool;

    #[method(name = "set_S011", args = 1)]
    pub fn set_s011(self, value: bool) -> ();

    #[method(name = "get_S012", args = 0)]
    pub fn get_s012(self) -> bool;

    #[method(name = "set_S012", args = 1)]
    pub fn set_s012(self, value: bool) -> ();

    #[method(name = "get_S013", args = 0)]
    pub fn get_s013(self) -> bool;

    #[method(name = "set_S013", args = 1)]
    pub fn set_s013(self, value: bool) -> ();

    #[method(name = "get_S014", args = 0)]
    pub fn get_s014(self) -> bool;

    #[method(name = "set_S014", args = 1)]
    pub fn set_s014(self, value: bool) -> ();

    #[method(name = "get_S015", args = 0)]
    pub fn get_s015(self) -> bool;

    #[method(name = "set_S015", args = 1)]
    pub fn set_s015(self, value: bool) -> ();

    #[method(name = "get_G001", args = 0)]
    pub fn get_g001(self) -> bool;

    #[method(name = "set_G001", args = 1)]
    pub fn set_g001(self, value: bool) -> ();

    #[method(name = "get_G002", args = 0)]
    pub fn get_g002(self) -> bool;

    #[method(name = "set_G002", args = 1)]
    pub fn set_g002(self, value: bool) -> ();

    #[method(name = "get_G003", args = 0)]
    pub fn get_g003(self) -> bool;

    #[method(name = "set_G003", args = 1)]
    pub fn set_g003(self, value: bool) -> ();

    #[method(name = "get_G004", args = 0)]
    pub fn get_g004(self) -> bool;

    #[method(name = "set_G004", args = 1)]
    pub fn set_g004(self, value: bool) -> ();

    #[method(name = "get_G005", args = 0)]
    pub fn get_g005(self) -> bool;

    #[method(name = "set_G005", args = 1)]
    pub fn set_g005(self, value: bool) -> ();

    #[method(name = "get_G006", args = 0)]
    pub fn get_g006(self) -> bool;

    #[method(name = "set_G006", args = 1)]
    pub fn set_g006(self, value: bool) -> ();

    #[method(name = "get_E001", args = 0)]
    pub fn get_e001(self) -> bool;

    #[method(name = "set_E001", args = 1)]
    pub fn set_e001(self, value: bool) -> ();

    #[method(name = "get_E002", args = 0)]
    pub fn get_e002(self) -> bool;

    #[method(name = "set_E002", args = 1)]
    pub fn set_e002(self, value: bool) -> ();

    #[method(name = "get_E003", args = 0)]
    pub fn get_e003(self) -> bool;

    #[method(name = "set_E003", args = 1)]
    pub fn set_e003(self, value: bool) -> ();

    #[method(name = "get_E004", args = 0)]
    pub fn get_e004(self) -> bool;

    #[method(name = "set_E004", args = 1)]
    pub fn set_e004(self, value: bool) -> ();

    #[method(name = "get_E005", args = 0)]
    pub fn get_e005(self) -> bool;

    #[method(name = "set_E005", args = 1)]
    pub fn set_e005(self, value: bool) -> ();

    #[method(name = "get_E006", args = 0)]
    pub fn get_e006(self) -> bool;

    #[method(name = "set_E006", args = 1)]
    pub fn set_e006(self, value: bool) -> ();

    #[method(name = "GetCatchNumFlagName", args = 0)]
    pub fn get_catch_num_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "IsHome", args = 1)]
    pub fn is_home(self, chapter_id: ::unity2::Il2CppString) -> bool;

    #[method(name = "ConvertItemList", args = 0)]
    pub fn convert_item_list(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-animaldata")]
impl AnimalData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalData),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalDataMethods>::ctor(this);
        this
    }
}
