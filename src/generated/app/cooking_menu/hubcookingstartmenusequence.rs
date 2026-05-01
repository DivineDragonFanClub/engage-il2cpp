
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/hubcookingstartmenusequence/HubCookingStartMenuSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubCookingStartMenuSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubCookingStartMenuSequence_Label {
    const NAMESPACE: &'static str = "App.CookingMenu";

    const NAME: &'static str = "HubCookingStartMenuSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubCookingStartMenuSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubCookingStartMenuSequence_Label {
    pub fn select_unit() -> Self {
        Self { value: 0 }
    }

    pub fn select_food() -> Self {
        Self { value: 1 }
    }

    pub fn select_foodstuff() -> Self {
        Self { value: 2 }
    }

    pub fn confirm() -> Self {
        Self { value: 3 }
    }

    pub fn decide() -> Self {
        Self { value: 4 }
    }

    pub fn end() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/hubcookingstartmenusequence/HubCookingStartMenuSequence.md")))]
#[::unity2::class(namespace = "App.CookingMenu", name = "HubCookingStartMenuSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubCookingStartMenuSequence {
# [rename (name = "m_DecideCallback")] pub m_decide_callback : crate :: app :: cooking_menu :: hubcookingstartmenusequence :: HubCookingStartMenuSequence_DecideEventHandler ,
# [rename (name = "m_EndCallback")] pub m_end_callback : crate :: system :: action :: Action ,
# [rename (name = "m_SelectUnitList")] pub m_select_unit_list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: unit :: Unit > ,
# [rename (name = "m_SelectFood")] pub m_select_food : crate :: app :: fooddata :: FoodData ,
# [rename (name = "m_SelectFoodstuffList")] pub m_select_foodstuff_list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: foodstuffdata :: FoodstuffData > ,
# [rename (name = "m_AllContent")] pub m_all_content : crate :: app :: cooking_menu :: dishallmenucontent :: DishAllMenuContent ,
}

#[cfg(feature = "app-cooking_menu-hubcookingstartmenusequence")]
#[::unity2::methods]
impl HubCookingStartMenuSequence {
    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources() -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources() -> bool;

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources() -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        decide_event_handler : crate :: app :: cooking_menu :: hubcookingstartmenusequence :: HubCookingStartMenuSequence_DecideEventHandler,
        end_event_handler: crate::system::action::Action,
    ) -> ();

    #[method(name = "CreateHistory", args = 0)]
    pub fn create_history(self) -> ();

    #[method(name = "OpenSelectUnitMenu", args = 0)]
    pub fn open_select_unit_menu(self) -> ();

    #[method(name = "PostSelectUnit", args = 0)]
    pub fn post_select_unit(self) -> ();

    #[method(name = "OpenSelectFoodMenu", args = 0)]
    pub fn open_select_food_menu(self) -> ();

    #[method(name = "PostOpenFoodMenu", args = 0)]
    pub fn post_open_food_menu(self) -> ();

    #[method(name = "OpenSelectFoodstuffMenu", args = 0)]
    pub fn open_select_foodstuff_menu(self) -> ();

    #[method(name = "OpenConfirmDialog", args = 0)]
    pub fn open_confirm_dialog(self) -> ();

    #[method(name = "ReturnSelectFoodstuff", args = 0)]
    pub fn return_select_foodstuff(self) -> ();

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> ();

    #[method(name = "CloseAllContent", args = 0)]
    pub fn close_all_content(self) -> ();

    #[method(name = "DeleteHistory", args = 0)]
    pub fn delete_history(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        decide_event_handler : crate :: app :: cooking_menu :: hubcookingstartmenusequence :: HubCookingStartMenuSequence_DecideEventHandler,
        end_event_handler: crate::system::action::Action,
    ) -> ();

    #[method(name = "GetCook", args = 0)]
    pub fn get_cook() -> crate::app::cookdata::CookData;
}

#[cfg(feature = "app-cooking_menu-hubcookingstartmenusequence")]
impl HubCookingStartMenuSequence {
    pub fn new(
        decide_event_handler : crate :: app :: cooking_menu :: hubcookingstartmenusequence :: HubCookingStartMenuSequence_DecideEventHandler,
        end_event_handler: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubCookingStartMenuSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IHubCookingStartMenuSequenceMethods>::ctor(
            this,
            decide_event_handler,
            end_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/hubcookingstartmenusequence/HubCookingStartMenuSequence_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App.CookingMenu",
    name = "HubCookingStartMenuSequence.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct HubCookingStartMenuSequence_DecideEventHandler {}

#[cfg(feature = "app-cooking_menu-hubcookingstartmenusequence")]
#[::unity2::methods]
impl HubCookingStartMenuSequence_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        units: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
        food: crate::app::fooddata::FoodData,
        foodstuffs: crate::system::collections::generic::list_1::List_1<
            crate::app::foodstuffdata::FoodstuffData,
        >,
        add_action: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-cooking_menu-hubcookingstartmenusequence")]
impl HubCookingStartMenuSequence_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubCookingStartMenuSequence_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IHubCookingStartMenuSequence_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}
