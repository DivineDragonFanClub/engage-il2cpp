
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/calculatortableadd1d/CalculatorTableAdd1D_TableCommand.md")))]
#[::unity2::class(namespace = "App", name = "CalculatorTableAdd1D.TableCommand")]
#[parent(crate::app::gamecalculatorcommand::GameCalculatorCommand)]
pub struct CalculatorTableAdd1D_TableCommand {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-calculatortableadd1d")]
#[::unity2::methods]
impl CalculatorTableAdd1D_TableCommand {
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

#[cfg(feature = "app-calculatortableadd1d")]
impl CalculatorTableAdd1D_TableCommand {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CalculatorTableAdd1D_TableCommand),
                ::core::stringify!(new),
            )
        });
        <Self as ICalculatorTableAdd1D_TableCommandMethods>::ctor(this, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/calculatortableadd1d/CalculatorTableAdd1D.md")))]
#[::unity2::class(namespace = "App", name = "CalculatorTableAdd1D")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: calculatortableadd1d :: CalculatorTableAdd1D >)]
pub struct CalculatorTableAdd1D {
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

#[cfg(feature = "app-calculatortableadd1d")]
#[::unity2::methods]
impl CalculatorTableAdd1D {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_N00", args = 0)]
    pub fn get_n00(self) -> i32;

    #[method(name = "set_N00", args = 1)]
    pub fn set_n00(self, value: i32) -> ();

    #[method(name = "get_N01", args = 0)]
    pub fn get_n01(self) -> i32;

    #[method(name = "set_N01", args = 1)]
    pub fn set_n01(self, value: i32) -> ();

    #[method(name = "get_N02", args = 0)]
    pub fn get_n02(self) -> i32;

    #[method(name = "set_N02", args = 1)]
    pub fn set_n02(self, value: i32) -> ();

    #[method(name = "get_N03", args = 0)]
    pub fn get_n03(self) -> i32;

    #[method(name = "set_N03", args = 1)]
    pub fn set_n03(self, value: i32) -> ();

    #[method(name = "get_N04", args = 0)]
    pub fn get_n04(self) -> i32;

    #[method(name = "set_N04", args = 1)]
    pub fn set_n04(self, value: i32) -> ();

    #[method(name = "get_N05", args = 0)]
    pub fn get_n05(self) -> i32;

    #[method(name = "set_N05", args = 1)]
    pub fn set_n05(self, value: i32) -> ();

    #[method(name = "get_N06", args = 0)]
    pub fn get_n06(self) -> i32;

    #[method(name = "set_N06", args = 1)]
    pub fn set_n06(self, value: i32) -> ();

    #[method(name = "get_N07", args = 0)]
    pub fn get_n07(self) -> i32;

    #[method(name = "set_N07", args = 1)]
    pub fn set_n07(self, value: i32) -> ();

    #[method(name = "get_N08", args = 0)]
    pub fn get_n08(self) -> i32;

    #[method(name = "set_N08", args = 1)]
    pub fn set_n08(self, value: i32) -> ();

    #[method(name = "get_N09", args = 0)]
    pub fn get_n09(self) -> i32;

    #[method(name = "set_N09", args = 1)]
    pub fn set_n09(self, value: i32) -> ();

    #[method(name = "get_N10", args = 0)]
    pub fn get_n10(self) -> i32;

    #[method(name = "set_N10", args = 1)]
    pub fn set_n10(self, value: i32) -> ();

    #[method(name = "get_N11", args = 0)]
    pub fn get_n11(self) -> i32;

    #[method(name = "set_N11", args = 1)]
    pub fn set_n11(self, value: i32) -> ();

    #[method(name = "get_N12", args = 0)]
    pub fn get_n12(self) -> i32;

    #[method(name = "set_N12", args = 1)]
    pub fn set_n12(self, value: i32) -> ();

    #[method(name = "get_N13", args = 0)]
    pub fn get_n13(self) -> i32;

    #[method(name = "set_N13", args = 1)]
    pub fn set_n13(self, value: i32) -> ();

    #[method(name = "get_N14", args = 0)]
    pub fn get_n14(self) -> i32;

    #[method(name = "set_N14", args = 1)]
    pub fn set_n14(self, value: i32) -> ();

    #[method(name = "get_N15", args = 0)]
    pub fn get_n15(self) -> i32;

    #[method(name = "set_N15", args = 1)]
    pub fn set_n15(self, value: i32) -> ();

    #[method(name = "get_N16", args = 0)]
    pub fn get_n16(self) -> i32;

    #[method(name = "set_N16", args = 1)]
    pub fn set_n16(self, value: i32) -> ();

    #[method(name = "get_N17", args = 0)]
    pub fn get_n17(self) -> i32;

    #[method(name = "set_N17", args = 1)]
    pub fn set_n17(self, value: i32) -> ();

    #[method(name = "get_N18", args = 0)]
    pub fn get_n18(self) -> i32;

    #[method(name = "set_N18", args = 1)]
    pub fn set_n18(self, value: i32) -> ();

    #[method(name = "get_N19", args = 0)]
    pub fn get_n19(self) -> i32;

    #[method(name = "set_N19", args = 1)]
    pub fn set_n19(self, value: i32) -> ();

    #[method(name = "get_N20", args = 0)]
    pub fn get_n20(self) -> i32;

    #[method(name = "set_N20", args = 1)]
    pub fn set_n20(self, value: i32) -> ();

    #[method(name = "get_N21", args = 0)]
    pub fn get_n21(self) -> i32;

    #[method(name = "set_N21", args = 1)]
    pub fn set_n21(self, value: i32) -> ();

    #[method(name = "get_N22", args = 0)]
    pub fn get_n22(self) -> i32;

    #[method(name = "set_N22", args = 1)]
    pub fn set_n22(self, value: i32) -> ();

    #[method(name = "get_N23", args = 0)]
    pub fn get_n23(self) -> i32;

    #[method(name = "set_N23", args = 1)]
    pub fn set_n23(self, value: i32) -> ();

    #[method(name = "get_N24", args = 0)]
    pub fn get_n24(self) -> i32;

    #[method(name = "set_N24", args = 1)]
    pub fn set_n24(self, value: i32) -> ();

    #[method(name = "get_N25", args = 0)]
    pub fn get_n25(self) -> i32;

    #[method(name = "set_N25", args = 1)]
    pub fn set_n25(self, value: i32) -> ();

    #[method(name = "get_N26", args = 0)]
    pub fn get_n26(self) -> i32;

    #[method(name = "set_N26", args = 1)]
    pub fn set_n26(self, value: i32) -> ();

    #[method(name = "get_N27", args = 0)]
    pub fn get_n27(self) -> i32;

    #[method(name = "set_N27", args = 1)]
    pub fn set_n27(self, value: i32) -> ();

    #[method(name = "get_N28", args = 0)]
    pub fn get_n28(self) -> i32;

    #[method(name = "set_N28", args = 1)]
    pub fn set_n28(self, value: i32) -> ();

    #[method(name = "get_N29", args = 0)]
    pub fn get_n29(self) -> i32;

    #[method(name = "set_N29", args = 1)]
    pub fn set_n29(self, value: i32) -> ();

    #[method(name = "get_N30", args = 0)]
    pub fn get_n30(self) -> i32;

    #[method(name = "set_N30", args = 1)]
    pub fn set_n30(self, value: i32) -> ();

    #[method(name = "get_N31", args = 0)]
    pub fn get_n31(self) -> i32;

    #[method(name = "set_N31", args = 1)]
    pub fn set_n31(self, value: i32) -> ();

    #[method(name = "get_N32", args = 0)]
    pub fn get_n32(self) -> i32;

    #[method(name = "set_N32", args = 1)]
    pub fn set_n32(self, value: i32) -> ();

    #[method(name = "get_N33", args = 0)]
    pub fn get_n33(self) -> i32;

    #[method(name = "set_N33", args = 1)]
    pub fn set_n33(self, value: i32) -> ();

    #[method(name = "get_N34", args = 0)]
    pub fn get_n34(self) -> i32;

    #[method(name = "set_N34", args = 1)]
    pub fn set_n34(self, value: i32) -> ();

    #[method(name = "get_N35", args = 0)]
    pub fn get_n35(self) -> i32;

    #[method(name = "set_N35", args = 1)]
    pub fn set_n35(self, value: i32) -> ();

    #[method(name = "get_N36", args = 0)]
    pub fn get_n36(self) -> i32;

    #[method(name = "set_N36", args = 1)]
    pub fn set_n36(self, value: i32) -> ();

    #[method(name = "get_N37", args = 0)]
    pub fn get_n37(self) -> i32;

    #[method(name = "set_N37", args = 1)]
    pub fn set_n37(self, value: i32) -> ();

    #[method(name = "get_N38", args = 0)]
    pub fn get_n38(self) -> i32;

    #[method(name = "set_N38", args = 1)]
    pub fn set_n38(self, value: i32) -> ();

    #[method(name = "get_N39", args = 0)]
    pub fn get_n39(self) -> i32;

    #[method(name = "set_N39", args = 1)]
    pub fn set_n39(self, value: i32) -> ();

    #[method(name = "get_N40", args = 0)]
    pub fn get_n40(self) -> i32;

    #[method(name = "set_N40", args = 1)]
    pub fn set_n40(self, value: i32) -> ();

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

#[cfg(feature = "app-calculatortableadd1d")]
impl CalculatorTableAdd1D {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CalculatorTableAdd1D),
                ::core::stringify!(new),
            )
        });
        <Self as ICalculatorTableAdd1DMethods>::ctor(this);
        this
    }
}
