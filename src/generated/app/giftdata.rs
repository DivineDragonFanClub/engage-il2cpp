
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/giftdata/GiftData.md")))]
#[::unity2::class(namespace = "App", name = "GiftData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: giftdata :: GiftData >)]
pub struct GiftData {
    #[static_field]
    #[rename(name = "DataMax")]
    pub data_max: i32,
    #[rename(name = "m_Values")]
    pub m_values: ::unity2::Array<i8>,
}

#[cfg(feature = "app-giftdata")]
#[::unity2::methods]
impl GiftData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> ::unity2::Array<i8>;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(unit: crate::app::unit::Unit, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value_2(
        person: crate::app::persondata::PersonData,
        item: crate::app::itemdata::ItemData,
    ) -> i32;

    #[method(name = "GetGiftStartIndex", args = 0)]
    pub fn get_gift_start_index() -> i32;

    #[method(name = "get_V00", args = 0)]
    pub fn get_v00(self) -> i8;

    #[method(name = "set_V00", args = 1)]
    pub fn set_v00(self, value: i8) -> ();

    #[method(name = "get_V01", args = 0)]
    pub fn get_v01(self) -> i8;

    #[method(name = "set_V01", args = 1)]
    pub fn set_v01(self, value: i8) -> ();

    #[method(name = "get_V02", args = 0)]
    pub fn get_v02(self) -> i8;

    #[method(name = "set_V02", args = 1)]
    pub fn set_v02(self, value: i8) -> ();

    #[method(name = "get_V03", args = 0)]
    pub fn get_v03(self) -> i8;

    #[method(name = "set_V03", args = 1)]
    pub fn set_v03(self, value: i8) -> ();

    #[method(name = "get_V04", args = 0)]
    pub fn get_v04(self) -> i8;

    #[method(name = "set_V04", args = 1)]
    pub fn set_v04(self, value: i8) -> ();

    #[method(name = "get_V05", args = 0)]
    pub fn get_v05(self) -> i8;

    #[method(name = "set_V05", args = 1)]
    pub fn set_v05(self, value: i8) -> ();

    #[method(name = "get_V06", args = 0)]
    pub fn get_v06(self) -> i8;

    #[method(name = "set_V06", args = 1)]
    pub fn set_v06(self, value: i8) -> ();

    #[method(name = "get_V07", args = 0)]
    pub fn get_v07(self) -> i8;

    #[method(name = "set_V07", args = 1)]
    pub fn set_v07(self, value: i8) -> ();

    #[method(name = "get_V08", args = 0)]
    pub fn get_v08(self) -> i8;

    #[method(name = "set_V08", args = 1)]
    pub fn set_v08(self, value: i8) -> ();

    #[method(name = "get_V09", args = 0)]
    pub fn get_v09(self) -> i8;

    #[method(name = "set_V09", args = 1)]
    pub fn set_v09(self, value: i8) -> ();

    #[method(name = "get_V10", args = 0)]
    pub fn get_v10(self) -> i8;

    #[method(name = "set_V10", args = 1)]
    pub fn set_v10(self, value: i8) -> ();

    #[method(name = "get_V11", args = 0)]
    pub fn get_v11(self) -> i8;

    #[method(name = "set_V11", args = 1)]
    pub fn set_v11(self, value: i8) -> ();

    #[method(name = "get_V12", args = 0)]
    pub fn get_v12(self) -> i8;

    #[method(name = "set_V12", args = 1)]
    pub fn set_v12(self, value: i8) -> ();

    #[method(name = "get_V13", args = 0)]
    pub fn get_v13(self) -> i8;

    #[method(name = "set_V13", args = 1)]
    pub fn set_v13(self, value: i8) -> ();

    #[method(name = "get_V14", args = 0)]
    pub fn get_v14(self) -> i8;

    #[method(name = "set_V14", args = 1)]
    pub fn set_v14(self, value: i8) -> ();

    #[method(name = "get_V15", args = 0)]
    pub fn get_v15(self) -> i8;

    #[method(name = "set_V15", args = 1)]
    pub fn set_v15(self, value: i8) -> ();

    #[method(name = "get_V16", args = 0)]
    pub fn get_v16(self) -> i8;

    #[method(name = "set_V16", args = 1)]
    pub fn set_v16(self, value: i8) -> ();

    #[method(name = "get_V17", args = 0)]
    pub fn get_v17(self) -> i8;

    #[method(name = "set_V17", args = 1)]
    pub fn set_v17(self, value: i8) -> ();

    #[method(name = "get_V18", args = 0)]
    pub fn get_v18(self) -> i8;

    #[method(name = "set_V18", args = 1)]
    pub fn set_v18(self, value: i8) -> ();

    #[method(name = "get_V19", args = 0)]
    pub fn get_v19(self) -> i8;

    #[method(name = "set_V19", args = 1)]
    pub fn set_v19(self, value: i8) -> ();

    #[method(name = "get_V20", args = 0)]
    pub fn get_v20(self) -> i8;

    #[method(name = "set_V20", args = 1)]
    pub fn set_v20(self, value: i8) -> ();

    #[method(name = "get_V21", args = 0)]
    pub fn get_v21(self) -> i8;

    #[method(name = "set_V21", args = 1)]
    pub fn set_v21(self, value: i8) -> ();

    #[method(name = "get_V22", args = 0)]
    pub fn get_v22(self) -> i8;

    #[method(name = "set_V22", args = 1)]
    pub fn set_v22(self, value: i8) -> ();

    #[method(name = "get_V23", args = 0)]
    pub fn get_v23(self) -> i8;

    #[method(name = "set_V23", args = 1)]
    pub fn set_v23(self, value: i8) -> ();

    #[method(name = "get_V24", args = 0)]
    pub fn get_v24(self) -> i8;

    #[method(name = "set_V24", args = 1)]
    pub fn set_v24(self, value: i8) -> ();

    #[method(name = "get_V25", args = 0)]
    pub fn get_v25(self) -> i8;

    #[method(name = "set_V25", args = 1)]
    pub fn set_v25(self, value: i8) -> ();

    #[method(name = "get_V26", args = 0)]
    pub fn get_v26(self) -> i8;

    #[method(name = "set_V26", args = 1)]
    pub fn set_v26(self, value: i8) -> ();

    #[method(name = "get_V27", args = 0)]
    pub fn get_v27(self) -> i8;

    #[method(name = "set_V27", args = 1)]
    pub fn set_v27(self, value: i8) -> ();

    #[method(name = "get_V28", args = 0)]
    pub fn get_v28(self) -> i8;

    #[method(name = "set_V28", args = 1)]
    pub fn set_v28(self, value: i8) -> ();

    #[method(name = "get_V29", args = 0)]
    pub fn get_v29(self) -> i8;

    #[method(name = "set_V29", args = 1)]
    pub fn set_v29(self, value: i8) -> ();

    #[method(name = "get_V30", args = 0)]
    pub fn get_v30(self) -> i8;

    #[method(name = "set_V30", args = 1)]
    pub fn set_v30(self, value: i8) -> ();

    #[method(name = "get_V31", args = 0)]
    pub fn get_v31(self) -> i8;

    #[method(name = "set_V31", args = 1)]
    pub fn set_v31(self, value: i8) -> ();

    #[method(name = "get_V32", args = 0)]
    pub fn get_v32(self) -> i8;

    #[method(name = "set_V32", args = 1)]
    pub fn set_v32(self, value: i8) -> ();

    #[method(name = "get_V33", args = 0)]
    pub fn get_v33(self) -> i8;

    #[method(name = "set_V33", args = 1)]
    pub fn set_v33(self, value: i8) -> ();

    #[method(name = "get_V34", args = 0)]
    pub fn get_v34(self) -> i8;

    #[method(name = "set_V34", args = 1)]
    pub fn set_v34(self, value: i8) -> ();

    #[method(name = "get_V35", args = 0)]
    pub fn get_v35(self) -> i8;

    #[method(name = "set_V35", args = 1)]
    pub fn set_v35(self, value: i8) -> ();

    #[method(name = "get_V36", args = 0)]
    pub fn get_v36(self) -> i8;

    #[method(name = "set_V36", args = 1)]
    pub fn set_v36(self, value: i8) -> ();

    #[method(name = "get_V37", args = 0)]
    pub fn get_v37(self) -> i8;

    #[method(name = "set_V37", args = 1)]
    pub fn set_v37(self, value: i8) -> ();

    #[method(name = "get_V38", args = 0)]
    pub fn get_v38(self) -> i8;

    #[method(name = "set_V38", args = 1)]
    pub fn set_v38(self, value: i8) -> ();

    #[method(name = "get_V39", args = 0)]
    pub fn get_v39(self) -> i8;

    #[method(name = "set_V39", args = 1)]
    pub fn set_v39(self, value: i8) -> ();

    #[method(name = "get_V40", args = 0)]
    pub fn get_v40(self) -> i8;

    #[method(name = "set_V40", args = 1)]
    pub fn set_v40(self, value: i8) -> ();

    #[method(name = "get_V41", args = 0)]
    pub fn get_v41(self) -> i8;

    #[method(name = "set_V41", args = 1)]
    pub fn set_v41(self, value: i8) -> ();

    #[method(name = "get_V42", args = 0)]
    pub fn get_v42(self) -> i8;

    #[method(name = "set_V42", args = 1)]
    pub fn set_v42(self, value: i8) -> ();

    #[method(name = "get_V43", args = 0)]
    pub fn get_v43(self) -> i8;

    #[method(name = "set_V43", args = 1)]
    pub fn set_v43(self, value: i8) -> ();

    #[method(name = "get_V44", args = 0)]
    pub fn get_v44(self) -> i8;

    #[method(name = "set_V44", args = 1)]
    pub fn set_v44(self, value: i8) -> ();

    #[method(name = "get_V45", args = 0)]
    pub fn get_v45(self) -> i8;

    #[method(name = "set_V45", args = 1)]
    pub fn set_v45(self, value: i8) -> ();

    #[method(name = "get_V46", args = 0)]
    pub fn get_v46(self) -> i8;

    #[method(name = "set_V46", args = 1)]
    pub fn set_v46(self, value: i8) -> ();

    #[method(name = "get_V47", args = 0)]
    pub fn get_v47(self) -> i8;

    #[method(name = "set_V47", args = 1)]
    pub fn set_v47(self, value: i8) -> ();

    #[method(name = "get_V48", args = 0)]
    pub fn get_v48(self) -> i8;

    #[method(name = "set_V48", args = 1)]
    pub fn set_v48(self, value: i8) -> ();

    #[method(name = "get_V49", args = 0)]
    pub fn get_v49(self) -> i8;

    #[method(name = "set_V49", args = 1)]
    pub fn set_v49(self, value: i8) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-giftdata")]
impl GiftData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GiftData),
                ::core::stringify!(new),
            )
        });
        <Self as IGiftDataMethods>::ctor(this);
        this
    }
}
