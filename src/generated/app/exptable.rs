
use crate::app::calculatorcommand::CalculatorCommand;
use crate::app::calculatorcommand::ICalculatorCommand;
use crate::app::gamecalculatorcommand::GameCalculatorCommand;
use crate::app::gamecalculatorcommand::IGameCalculatorCommand;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exptable/ExpTable_TableCommand.md")))]
#[::unity2::class(namespace = "App", name = "ExpTable.TableCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct ExpTable_TableCommand {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-exptable")]
#[::unity2::methods]
impl ExpTable_TableCommand {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FuncImpl", args = 2)]
    pub fn func_impl(
        self,
        unit: crate::app::unit::Unit,
        args: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> f32;
}

#[cfg(feature = "app-exptable")]
impl ExpTable_TableCommand {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExpTable_TableCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IExpTable_TableCommandMethods>::ctor(this, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exptable/ExpTable.md")))]
#[::unity2::class(namespace = "App", name = "ExpTable")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: exptable :: ExpTable >)]
pub struct ExpTable {
    #[static_field]
    #[rename(name = "Min")]
    pub min: i32,
    #[static_field]
    #[rename(name = "Max")]
    pub max: i32,
    #[static_field]
    #[rename(name = "Num")]
    pub num: i32,
    #[rename(name = "m_Table")]
    pub m_table: crate::app::calculatortable::CalculatorTable,
}

#[cfg(feature = "app-exptable")]
#[::unity2::methods]
impl ExpTable {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_M39", args = 0)]
    pub fn get_m39(self) -> i32;

    #[method(name = "set_M39", args = 1)]
    pub fn set_m39(self, value: i32) -> ();

    #[method(name = "get_M38", args = 0)]
    pub fn get_m38(self) -> i32;

    #[method(name = "set_M38", args = 1)]
    pub fn set_m38(self, value: i32) -> ();

    #[method(name = "get_M37", args = 0)]
    pub fn get_m37(self) -> i32;

    #[method(name = "set_M37", args = 1)]
    pub fn set_m37(self, value: i32) -> ();

    #[method(name = "get_M36", args = 0)]
    pub fn get_m36(self) -> i32;

    #[method(name = "set_M36", args = 1)]
    pub fn set_m36(self, value: i32) -> ();

    #[method(name = "get_M35", args = 0)]
    pub fn get_m35(self) -> i32;

    #[method(name = "set_M35", args = 1)]
    pub fn set_m35(self, value: i32) -> ();

    #[method(name = "get_M34", args = 0)]
    pub fn get_m34(self) -> i32;

    #[method(name = "set_M34", args = 1)]
    pub fn set_m34(self, value: i32) -> ();

    #[method(name = "get_M33", args = 0)]
    pub fn get_m33(self) -> i32;

    #[method(name = "set_M33", args = 1)]
    pub fn set_m33(self, value: i32) -> ();

    #[method(name = "get_M32", args = 0)]
    pub fn get_m32(self) -> i32;

    #[method(name = "set_M32", args = 1)]
    pub fn set_m32(self, value: i32) -> ();

    #[method(name = "get_M31", args = 0)]
    pub fn get_m31(self) -> i32;

    #[method(name = "set_M31", args = 1)]
    pub fn set_m31(self, value: i32) -> ();

    #[method(name = "get_M30", args = 0)]
    pub fn get_m30(self) -> i32;

    #[method(name = "set_M30", args = 1)]
    pub fn set_m30(self, value: i32) -> ();

    #[method(name = "get_M29", args = 0)]
    pub fn get_m29(self) -> i32;

    #[method(name = "set_M29", args = 1)]
    pub fn set_m29(self, value: i32) -> ();

    #[method(name = "get_M28", args = 0)]
    pub fn get_m28(self) -> i32;

    #[method(name = "set_M28", args = 1)]
    pub fn set_m28(self, value: i32) -> ();

    #[method(name = "get_M27", args = 0)]
    pub fn get_m27(self) -> i32;

    #[method(name = "set_M27", args = 1)]
    pub fn set_m27(self, value: i32) -> ();

    #[method(name = "get_M26", args = 0)]
    pub fn get_m26(self) -> i32;

    #[method(name = "set_M26", args = 1)]
    pub fn set_m26(self, value: i32) -> ();

    #[method(name = "get_M25", args = 0)]
    pub fn get_m25(self) -> i32;

    #[method(name = "set_M25", args = 1)]
    pub fn set_m25(self, value: i32) -> ();

    #[method(name = "get_M24", args = 0)]
    pub fn get_m24(self) -> i32;

    #[method(name = "set_M24", args = 1)]
    pub fn set_m24(self, value: i32) -> ();

    #[method(name = "get_M23", args = 0)]
    pub fn get_m23(self) -> i32;

    #[method(name = "set_M23", args = 1)]
    pub fn set_m23(self, value: i32) -> ();

    #[method(name = "get_M22", args = 0)]
    pub fn get_m22(self) -> i32;

    #[method(name = "set_M22", args = 1)]
    pub fn set_m22(self, value: i32) -> ();

    #[method(name = "get_M21", args = 0)]
    pub fn get_m21(self) -> i32;

    #[method(name = "set_M21", args = 1)]
    pub fn set_m21(self, value: i32) -> ();

    #[method(name = "get_M20", args = 0)]
    pub fn get_m20(self) -> i32;

    #[method(name = "set_M20", args = 1)]
    pub fn set_m20(self, value: i32) -> ();

    #[method(name = "get_M19", args = 0)]
    pub fn get_m19(self) -> i32;

    #[method(name = "set_M19", args = 1)]
    pub fn set_m19(self, value: i32) -> ();

    #[method(name = "get_M18", args = 0)]
    pub fn get_m18(self) -> i32;

    #[method(name = "set_M18", args = 1)]
    pub fn set_m18(self, value: i32) -> ();

    #[method(name = "get_M17", args = 0)]
    pub fn get_m17(self) -> i32;

    #[method(name = "set_M17", args = 1)]
    pub fn set_m17(self, value: i32) -> ();

    #[method(name = "get_M16", args = 0)]
    pub fn get_m16(self) -> i32;

    #[method(name = "set_M16", args = 1)]
    pub fn set_m16(self, value: i32) -> ();

    #[method(name = "get_M15", args = 0)]
    pub fn get_m15(self) -> i32;

    #[method(name = "set_M15", args = 1)]
    pub fn set_m15(self, value: i32) -> ();

    #[method(name = "get_M14", args = 0)]
    pub fn get_m14(self) -> i32;

    #[method(name = "set_M14", args = 1)]
    pub fn set_m14(self, value: i32) -> ();

    #[method(name = "get_M13", args = 0)]
    pub fn get_m13(self) -> i32;

    #[method(name = "set_M13", args = 1)]
    pub fn set_m13(self, value: i32) -> ();

    #[method(name = "get_M12", args = 0)]
    pub fn get_m12(self) -> i32;

    #[method(name = "set_M12", args = 1)]
    pub fn set_m12(self, value: i32) -> ();

    #[method(name = "get_M11", args = 0)]
    pub fn get_m11(self) -> i32;

    #[method(name = "set_M11", args = 1)]
    pub fn set_m11(self, value: i32) -> ();

    #[method(name = "get_M10", args = 0)]
    pub fn get_m10(self) -> i32;

    #[method(name = "set_M10", args = 1)]
    pub fn set_m10(self, value: i32) -> ();

    #[method(name = "get_M09", args = 0)]
    pub fn get_m09(self) -> i32;

    #[method(name = "set_M09", args = 1)]
    pub fn set_m09(self, value: i32) -> ();

    #[method(name = "get_M08", args = 0)]
    pub fn get_m08(self) -> i32;

    #[method(name = "set_M08", args = 1)]
    pub fn set_m08(self, value: i32) -> ();

    #[method(name = "get_M07", args = 0)]
    pub fn get_m07(self) -> i32;

    #[method(name = "set_M07", args = 1)]
    pub fn set_m07(self, value: i32) -> ();

    #[method(name = "get_M06", args = 0)]
    pub fn get_m06(self) -> i32;

    #[method(name = "set_M06", args = 1)]
    pub fn set_m06(self, value: i32) -> ();

    #[method(name = "get_M05", args = 0)]
    pub fn get_m05(self) -> i32;

    #[method(name = "set_M05", args = 1)]
    pub fn set_m05(self, value: i32) -> ();

    #[method(name = "get_M04", args = 0)]
    pub fn get_m04(self) -> i32;

    #[method(name = "set_M04", args = 1)]
    pub fn set_m04(self, value: i32) -> ();

    #[method(name = "get_M03", args = 0)]
    pub fn get_m03(self) -> i32;

    #[method(name = "set_M03", args = 1)]
    pub fn set_m03(self, value: i32) -> ();

    #[method(name = "get_M02", args = 0)]
    pub fn get_m02(self) -> i32;

    #[method(name = "set_M02", args = 1)]
    pub fn set_m02(self, value: i32) -> ();

    #[method(name = "get_M01", args = 0)]
    pub fn get_m01(self) -> i32;

    #[method(name = "set_M01", args = 1)]
    pub fn set_m01(self, value: i32) -> ();

    #[method(name = "get_N00", args = 0)]
    pub fn get_n00(self) -> i32;

    #[method(name = "set_N00", args = 1)]
    pub fn set_n00(self, value: i32) -> ();

    #[method(name = "get_P00", args = 0)]
    pub fn get_p00(self) -> i32;

    #[method(name = "set_P00", args = 1)]
    pub fn set_p00(self, value: i32) -> ();

    #[method(name = "get_P01", args = 0)]
    pub fn get_p01(self) -> i32;

    #[method(name = "set_P01", args = 1)]
    pub fn set_p01(self, value: i32) -> ();

    #[method(name = "get_P02", args = 0)]
    pub fn get_p02(self) -> i32;

    #[method(name = "set_P02", args = 1)]
    pub fn set_p02(self, value: i32) -> ();

    #[method(name = "get_P03", args = 0)]
    pub fn get_p03(self) -> i32;

    #[method(name = "set_P03", args = 1)]
    pub fn set_p03(self, value: i32) -> ();

    #[method(name = "get_P04", args = 0)]
    pub fn get_p04(self) -> i32;

    #[method(name = "set_P04", args = 1)]
    pub fn set_p04(self, value: i32) -> ();

    #[method(name = "get_P05", args = 0)]
    pub fn get_p05(self) -> i32;

    #[method(name = "set_P05", args = 1)]
    pub fn set_p05(self, value: i32) -> ();

    #[method(name = "get_P06", args = 0)]
    pub fn get_p06(self) -> i32;

    #[method(name = "set_P06", args = 1)]
    pub fn set_p06(self, value: i32) -> ();

    #[method(name = "get_P07", args = 0)]
    pub fn get_p07(self) -> i32;

    #[method(name = "set_P07", args = 1)]
    pub fn set_p07(self, value: i32) -> ();

    #[method(name = "get_P08", args = 0)]
    pub fn get_p08(self) -> i32;

    #[method(name = "set_P08", args = 1)]
    pub fn set_p08(self, value: i32) -> ();

    #[method(name = "get_P09", args = 0)]
    pub fn get_p09(self) -> i32;

    #[method(name = "set_P09", args = 1)]
    pub fn set_p09(self, value: i32) -> ();

    #[method(name = "get_P10", args = 0)]
    pub fn get_p10(self) -> i32;

    #[method(name = "set_P10", args = 1)]
    pub fn set_p10(self, value: i32) -> ();

    #[method(name = "get_P11", args = 0)]
    pub fn get_p11(self) -> i32;

    #[method(name = "set_P11", args = 1)]
    pub fn set_p11(self, value: i32) -> ();

    #[method(name = "get_P12", args = 0)]
    pub fn get_p12(self) -> i32;

    #[method(name = "set_P12", args = 1)]
    pub fn set_p12(self, value: i32) -> ();

    #[method(name = "get_P13", args = 0)]
    pub fn get_p13(self) -> i32;

    #[method(name = "set_P13", args = 1)]
    pub fn set_p13(self, value: i32) -> ();

    #[method(name = "get_P14", args = 0)]
    pub fn get_p14(self) -> i32;

    #[method(name = "set_P14", args = 1)]
    pub fn set_p14(self, value: i32) -> ();

    #[method(name = "get_P15", args = 0)]
    pub fn get_p15(self) -> i32;

    #[method(name = "set_P15", args = 1)]
    pub fn set_p15(self, value: i32) -> ();

    #[method(name = "get_P16", args = 0)]
    pub fn get_p16(self) -> i32;

    #[method(name = "set_P16", args = 1)]
    pub fn set_p16(self, value: i32) -> ();

    #[method(name = "get_P17", args = 0)]
    pub fn get_p17(self) -> i32;

    #[method(name = "set_P17", args = 1)]
    pub fn set_p17(self, value: i32) -> ();

    #[method(name = "get_P18", args = 0)]
    pub fn get_p18(self) -> i32;

    #[method(name = "set_P18", args = 1)]
    pub fn set_p18(self, value: i32) -> ();

    #[method(name = "get_P19", args = 0)]
    pub fn get_p19(self) -> i32;

    #[method(name = "set_P19", args = 1)]
    pub fn set_p19(self, value: i32) -> ();

    #[method(name = "get_P20", args = 0)]
    pub fn get_p20(self) -> i32;

    #[method(name = "set_P20", args = 1)]
    pub fn set_p20(self, value: i32) -> ();

    #[method(name = "get_P21", args = 0)]
    pub fn get_p21(self) -> i32;

    #[method(name = "set_P21", args = 1)]
    pub fn set_p21(self, value: i32) -> ();

    #[method(name = "get_P22", args = 0)]
    pub fn get_p22(self) -> i32;

    #[method(name = "set_P22", args = 1)]
    pub fn set_p22(self, value: i32) -> ();

    #[method(name = "get_P23", args = 0)]
    pub fn get_p23(self) -> i32;

    #[method(name = "set_P23", args = 1)]
    pub fn set_p23(self, value: i32) -> ();

    #[method(name = "get_P24", args = 0)]
    pub fn get_p24(self) -> i32;

    #[method(name = "set_P24", args = 1)]
    pub fn set_p24(self, value: i32) -> ();

    #[method(name = "get_P25", args = 0)]
    pub fn get_p25(self) -> i32;

    #[method(name = "set_P25", args = 1)]
    pub fn set_p25(self, value: i32) -> ();

    #[method(name = "get_P26", args = 0)]
    pub fn get_p26(self) -> i32;

    #[method(name = "set_P26", args = 1)]
    pub fn set_p26(self, value: i32) -> ();

    #[method(name = "get_P27", args = 0)]
    pub fn get_p27(self) -> i32;

    #[method(name = "set_P27", args = 1)]
    pub fn set_p27(self, value: i32) -> ();

    #[method(name = "get_P28", args = 0)]
    pub fn get_p28(self) -> i32;

    #[method(name = "set_P28", args = 1)]
    pub fn set_p28(self, value: i32) -> ();

    #[method(name = "get_P29", args = 0)]
    pub fn get_p29(self) -> i32;

    #[method(name = "set_P29", args = 1)]
    pub fn set_p29(self, value: i32) -> ();

    #[method(name = "get_P30", args = 0)]
    pub fn get_p30(self) -> i32;

    #[method(name = "set_P30", args = 1)]
    pub fn set_p30(self, value: i32) -> ();

    #[method(name = "get_P31", args = 0)]
    pub fn get_p31(self) -> i32;

    #[method(name = "set_P31", args = 1)]
    pub fn set_p31(self, value: i32) -> ();

    #[method(name = "get_P32", args = 0)]
    pub fn get_p32(self) -> i32;

    #[method(name = "set_P32", args = 1)]
    pub fn set_p32(self, value: i32) -> ();

    #[method(name = "get_P33", args = 0)]
    pub fn get_p33(self) -> i32;

    #[method(name = "set_P33", args = 1)]
    pub fn set_p33(self, value: i32) -> ();

    #[method(name = "get_P34", args = 0)]
    pub fn get_p34(self) -> i32;

    #[method(name = "set_P34", args = 1)]
    pub fn set_p34(self, value: i32) -> ();

    #[method(name = "get_P35", args = 0)]
    pub fn get_p35(self) -> i32;

    #[method(name = "set_P35", args = 1)]
    pub fn set_p35(self, value: i32) -> ();

    #[method(name = "get_P36", args = 0)]
    pub fn get_p36(self) -> i32;

    #[method(name = "set_P36", args = 1)]
    pub fn set_p36(self, value: i32) -> ();

    #[method(name = "get_P37", args = 0)]
    pub fn get_p37(self) -> i32;

    #[method(name = "set_P37", args = 1)]
    pub fn set_p37(self, value: i32) -> ();

    #[method(name = "get_P38", args = 0)]
    pub fn get_p38(self) -> i32;

    #[method(name = "set_P38", args = 1)]
    pub fn set_p38(self, value: i32) -> ();

    #[method(name = "get_P39", args = 0)]
    pub fn get_p39(self) -> i32;

    #[method(name = "set_P39", args = 1)]
    pub fn set_p39(self, value: i32) -> ();

    #[method(name = "get_P40", args = 0)]
    pub fn get_p40(self) -> i32;

    #[method(name = "set_P40", args = 1)]
    pub fn set_p40(self, value: i32) -> ();

    #[method(name = "GetResult", args = 2)]
    pub fn get_result(name: ::unity2::Il2CppString, value: i32) -> i32;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-exptable")]
impl ExpTable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExpTable),
                ::core::stringify!(new),
            )
        });
        <Self as IExpTableMethods>::ctor(this);
        this
    }
}
