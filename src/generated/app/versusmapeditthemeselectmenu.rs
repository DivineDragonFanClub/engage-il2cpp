
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusmapeditthemeselectmenu/VersusMapEditThemeSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusMapEditThemeSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusMapEditThemeSelectMenu {
    #[rename(name = "m_Content")]
    pub m_content: crate::app::versusmapeditthemeselectcontent::VersusMapEditThemeSelectContent,
    #[rename(name = "m_SelectedCategory")]
    pub m_selected_category:
        crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData_Categories,
    #[rename(name = "m_SelectList")]
    pub m_select_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuselect::BasicMenuSelect,
    >,
}

#[cfg(feature = "app-versusmapeditthemeselectmenu")]
#[::unity2::methods]
impl VersusMapEditThemeSelectMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        now_theme: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
        func : crate :: app :: versusmapeditthemeselectmenu :: VersusMapEditThemeSelectMenu_SelectedFunction,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        content: crate::app::versusmapeditthemeselectcontent::VersusMapEditThemeSelectContent,
        init_category : crate :: app :: profilecardthemeofeditmapdata :: ProfileCardThemeOfEditMapData_Categories,
    ) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "ChangeCategoryLeft", args = 0)]
    pub fn change_category_left(self) -> ();

    #[method(name = "ChangeCategoryRight", args = 0)]
    pub fn change_category_right(self) -> ();

    #[method(name = "ChangeCategory", args = 1)]
    pub fn change_category(
        self,
        next: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData_Categories,
    ) -> ();
}

#[cfg(feature = "app-versusmapeditthemeselectmenu")]
impl VersusMapEditThemeSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        content: crate::app::versusmapeditthemeselectcontent::VersusMapEditThemeSelectContent,
        init_category : crate :: app :: profilecardthemeofeditmapdata :: ProfileCardThemeOfEditMapData_Categories,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusMapEditThemeSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusMapEditThemeSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            content,
            init_category,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusmapeditthemeselectmenu/VersusMapEditThemeSelectMenu_VersusMapEditThemeSelectMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusMapEditThemeSelectMenu.VersusMapEditThemeSelectMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusMapEditThemeSelectMenu_VersusMapEditThemeSelectMenuItem {
    #[rename(name = "m_Data")]
    pub m_data: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
    #[rename(name = "m_IsChecked")]
    pub m_is_checked: bool,
    #[rename(name = "m_SelectedFunc")]
    pub m_selected_func:
        crate::app::versusmapeditthemeselectmenu::VersusMapEditThemeSelectMenu_SelectedFunction,
}

#[cfg(feature = "app-versusmapeditthemeselectmenu")]
#[::unity2::methods]
impl VersusMapEditThemeSelectMenu_VersusMapEditThemeSelectMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        data: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
        is_checked: bool,
        func : crate :: app :: versusmapeditthemeselectmenu :: VersusMapEditThemeSelectMenu_SelectedFunction,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsChecked", args = 0)]
    pub fn is_checked(self) -> bool;

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "UpdateFixedCursor", args = 0)]
    pub fn update_fixed_cursor(self) -> ();
}

#[cfg(feature = "app-versusmapeditthemeselectmenu")]
impl VersusMapEditThemeSelectMenu_VersusMapEditThemeSelectMenuItem {
    pub fn new(
        data: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
        is_checked: bool,
        func : crate :: app :: versusmapeditthemeselectmenu :: VersusMapEditThemeSelectMenu_SelectedFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusMapEditThemeSelectMenu_VersusMapEditThemeSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusMapEditThemeSelectMenu_VersusMapEditThemeSelectMenuItemMethods>::ctor(
            this, data, is_checked, func,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusmapeditthemeselectmenu/VersusMapEditThemeSelectMenu_SelectedFunction.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusMapEditThemeSelectMenu.SelectedFunction"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VersusMapEditThemeSelectMenu_SelectedFunction {}

#[cfg(feature = "app-versusmapeditthemeselectmenu")]
#[::unity2::methods]
impl VersusMapEditThemeSelectMenu_SelectedFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        selected_theme: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
    ) -> ();
}

#[cfg(feature = "app-versusmapeditthemeselectmenu")]
impl VersusMapEditThemeSelectMenu_SelectedFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusMapEditThemeSelectMenu_SelectedFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusMapEditThemeSelectMenu_SelectedFunctionMethods>::ctor(this, object, method);
        this
    }
}
