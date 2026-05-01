
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::events::unityevent_1::IUnityEvent_1;
use crate::unity_engine::events::unityevent_1::UnityEvent_1;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::selectable::ISelectable;
use crate::unity_engine::ui::selectable::Selectable;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_dropdown/TMP_Dropdown_OptionDataList.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Dropdown.OptionDataList")]
#[parent(crate::system::object::Object)]
pub struct TMP_Dropdown_OptionDataList {
    #[rename(name = "m_Options")]
    pub m_options: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
    >,
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
#[::unity2::methods]
impl TMP_Dropdown_OptionDataList {
    #[method(name = "get_options", args = 0)]
    pub fn get_options(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
    >;

    #[method(name = "set_options", args = 1)]
    pub fn set_options(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
impl TMP_Dropdown_OptionDataList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown_OptionDataList),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_Dropdown_OptionDataListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_dropdown/TMP_Dropdown_DropdownEvent.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Dropdown.DropdownEvent")]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < i32 >)]
pub struct TMP_Dropdown_DropdownEvent {}

#[cfg(feature = "tm_pro-tmp_dropdown")]
#[::unity2::methods]
impl TMP_Dropdown_DropdownEvent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
impl TMP_Dropdown_DropdownEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown_DropdownEvent),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_Dropdown_DropdownEventMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_dropdown/TMP_Dropdown_DropdownItem.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Dropdown.DropdownItem")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TMP_Dropdown_DropdownItem {
    #[rename(name = "m_Text")]
    pub m_text: crate::tm_pro::tmp_text::TMP_Text,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_RectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Toggle")]
    pub m_toggle: crate::unity_engine::ui::toggle::Toggle,
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
#[::unity2::methods]
impl TMP_Dropdown_DropdownItem {
    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> crate::tm_pro::tmp_text::TMP_Text;

    #[method(name = "set_text", args = 1)]
    pub fn set_text(self, value: crate::tm_pro::tmp_text::TMP_Text) -> ();

    #[method(name = "get_image", args = 0)]
    pub fn get_image(self) -> crate::unity_engine::ui::image::Image;

    #[method(name = "set_image", args = 1)]
    pub fn set_image(self, value: crate::unity_engine::ui::image::Image) -> ();

    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "set_rectTransform", args = 1)]
    pub fn set_rect_transform(self, value: crate::unity_engine::recttransform::RectTransform)
        -> ();

    #[method(name = "get_toggle", args = 0)]
    pub fn get_toggle(self) -> crate::unity_engine::ui::toggle::Toggle;

    #[method(name = "set_toggle", args = 1)]
    pub fn set_toggle(self, value: crate::unity_engine::ui::toggle::Toggle) -> ();

    #[method(name = "OnPointerEnter", args = 1)]
    pub fn on_pointer_enter(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnCancel", args = 1)]
    pub fn on_cancel(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
impl TMP_Dropdown_DropdownItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown_DropdownItem),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_Dropdown_DropdownItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_dropdown/TMP_Dropdown_OptionData.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Dropdown.OptionData")]
#[parent(crate::system::object::Object)]
pub struct TMP_Dropdown_OptionData {
    #[rename(name = "m_Text")]
    pub m_text: ::unity2::Il2CppString,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::sprite::Sprite,
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
#[::unity2::methods]
impl TMP_Dropdown_OptionData {
    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_text", args = 1)]
    pub fn set_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_image", args = 0)]
    pub fn get_image(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "set_image", args = 1)]
    pub fn set_image(self, value: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, image: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(
        self,
        text: ::unity2::Il2CppString,
        image: crate::unity_engine::sprite::Sprite,
    ) -> ();
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
impl TMP_Dropdown_OptionData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown_OptionData),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_Dropdown_OptionDataMethods>::ctor(this);
        this
    }

    pub fn new_2(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown_OptionData),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITMP_Dropdown_OptionDataMethods>::ctor_2(this, text);
        this
    }

    pub fn new_3(image: crate::unity_engine::sprite::Sprite) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown_OptionData),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITMP_Dropdown_OptionDataMethods>::ctor_3(this, image);
        this
    }

    pub fn new_4(text: ::unity2::Il2CppString, image: crate::unity_engine::sprite::Sprite) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown_OptionData),
                ::core::stringify!(new_4),
            )
        });
        <Self as ITMP_Dropdown_OptionDataMethods>::ctor_4(this, text, image);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_dropdown/TMP_Dropdown.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Dropdown")]
#[parent(crate::unity_engine::ui::selectable::Selectable)]
pub struct TMP_Dropdown {
    #[rename(name = "m_Template")]
    pub m_template: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_CaptionText")]
    pub m_caption_text: crate::tm_pro::tmp_text::TMP_Text,
    #[rename(name = "m_CaptionImage")]
    pub m_caption_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Placeholder")]
    pub m_placeholder: crate::unity_engine::ui::graphic::Graphic,
    #[rename(name = "m_ItemText")]
    pub m_item_text: crate::tm_pro::tmp_text::TMP_Text,
    #[rename(name = "m_ItemImage")]
    pub m_item_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Value")]
    pub m_value: i32,
    #[rename(name = "m_Options")]
    pub m_options: crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionDataList,
    #[rename(name = "m_OnValueChanged")]
    pub m_on_value_changed: crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownEvent,
    #[rename(name = "m_AlphaFadeSpeed")]
    pub m_alpha_fade_speed: f32,
    #[rename(name = "m_Dropdown")]
    pub m_dropdown: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Blocker")]
    pub m_blocker: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Items")]
    pub m_items: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownItem,
    >,
    #[rename(name = "m_AlphaTweenRunner")]
    pub m_alpha_tween_runner:
        crate::tm_pro::tweenrunner_1_2::TweenRunner_1_2<crate::tm_pro::floattween_2::FloatTween_2>,
    #[rename(name = "validTemplate")]
    pub valid_template: bool,
    #[rename(name = "m_Coroutine")]
    pub m_coroutine: crate::unity_engine::coroutine::Coroutine,
    #[static_field]
    #[rename(name = "s_NoOptionData")]
    pub s_no_option_data: crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
#[::unity2::methods]
impl TMP_Dropdown {
    #[method(name = "get_template", args = 0)]
    pub fn get_template(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "set_template", args = 1)]
    pub fn set_template(self, value: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "get_captionText", args = 0)]
    pub fn get_caption_text(self) -> crate::tm_pro::tmp_text::TMP_Text;

    #[method(name = "set_captionText", args = 1)]
    pub fn set_caption_text(self, value: crate::tm_pro::tmp_text::TMP_Text) -> ();

    #[method(name = "get_captionImage", args = 0)]
    pub fn get_caption_image(self) -> crate::unity_engine::ui::image::Image;

    #[method(name = "set_captionImage", args = 1)]
    pub fn set_caption_image(self, value: crate::unity_engine::ui::image::Image) -> ();

    #[method(name = "get_placeholder", args = 0)]
    pub fn get_placeholder(self) -> crate::unity_engine::ui::graphic::Graphic;

    #[method(name = "set_placeholder", args = 1)]
    pub fn set_placeholder(self, value: crate::unity_engine::ui::graphic::Graphic) -> ();

    #[method(name = "get_itemText", args = 0)]
    pub fn get_item_text(self) -> crate::tm_pro::tmp_text::TMP_Text;

    #[method(name = "set_itemText", args = 1)]
    pub fn set_item_text(self, value: crate::tm_pro::tmp_text::TMP_Text) -> ();

    #[method(name = "get_itemImage", args = 0)]
    pub fn get_item_image(self) -> crate::unity_engine::ui::image::Image;

    #[method(name = "set_itemImage", args = 1)]
    pub fn set_item_image(self, value: crate::unity_engine::ui::image::Image) -> ();

    #[method(name = "get_options", args = 0)]
    pub fn get_options(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
    >;

    #[method(name = "set_options", args = 1)]
    pub fn set_options(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
        >,
    ) -> ();

    #[method(name = "get_onValueChanged", args = 0)]
    pub fn get_on_value_changed(self) -> crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownEvent;

    #[method(name = "set_onValueChanged", args = 1)]
    pub fn set_on_value_changed(
        self,
        value: crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownEvent,
    ) -> ();

    #[method(name = "get_alphaFadeSpeed", args = 0)]
    pub fn get_alpha_fade_speed(self) -> f32;

    #[method(name = "set_alphaFadeSpeed", args = 1)]
    pub fn set_alpha_fade_speed(self, value: f32) -> ();

    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "SetValueWithoutNotify", args = 1)]
    pub fn set_value_without_notify(self, input: i32) -> ();

    #[method(name = "SetValue", args = 2)]
    pub fn set_value_2(self, value: i32, send_callback: bool) -> ();

    #[method(name = "get_IsExpanded", args = 0)]
    pub fn get_is_expanded(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "RefreshShownValue", args = 0)]
    pub fn refresh_shown_value(self) -> ();

    #[method(name = "AddOptions", args = 1)]
    pub fn add_options(
        self,
        options: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
        >,
    ) -> ();

    #[method(name = "AddOptions", args = 1)]
    pub fn add_options_2(
        self,
        options: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "AddOptions", args = 1)]
    pub fn add_options_3(
        self,
        options: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::sprite::Sprite,
        >,
    ) -> ();

    #[method(name = "ClearOptions", args = 0)]
    pub fn clear_options(self) -> ();

    #[method(name = "SetupTemplate", args = 0)]
    pub fn setup_template(self) -> ();

    #[method(name = "OnPointerClick", args = 1)]
    pub fn on_pointer_click(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnSubmit", args = 1)]
    pub fn on_submit(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnCancel", args = 1)]
    pub fn on_cancel(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "CreateBlocker", args = 1)]
    pub fn create_blocker(
        self,
        root_canvas: crate::unity_engine::canvas::Canvas,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "DestroyBlocker", args = 1)]
    pub fn destroy_blocker(self, blocker: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "CreateDropdownList", args = 1)]
    pub fn create_dropdown_list(
        self,
        template: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "DestroyDropdownList", args = 1)]
    pub fn destroy_dropdown_list(
        self,
        dropdown_list: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "CreateItem", args = 1)]
    pub fn create_item(
        self,
        item_template: crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownItem,
    ) -> crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownItem;

    #[method(name = "DestroyItem", args = 1)]
    pub fn destroy_item(self, item: crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownItem) -> ();

    #[method(name = "AddItem", args = 4)]
    pub fn add_item(
        self,
        data: crate::tm_pro::tmp_dropdown::TMP_Dropdown_OptionData,
        selected: bool,
        item_template: crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownItem,
        items: crate::system::collections::generic::list_1::List_1<
            crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownItem,
        >,
    ) -> crate::tm_pro::tmp_dropdown::TMP_Dropdown_DropdownItem;

    #[method(name = "AlphaFadeList", args = 2)]
    pub fn alpha_fade_list(self, duration: f32, alpha: f32) -> ();

    #[method(name = "AlphaFadeList", args = 3)]
    pub fn alpha_fade_list_2(self, duration: f32, start: f32, end: f32) -> ();

    #[method(name = "SetAlpha", args = 1)]
    pub fn set_alpha(self, alpha: f32) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "DelayedDestroyDropdownList", args = 1)]
    pub fn delayed_destroy_dropdown_list(
        self,
        delay: f32,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ImmediateDestroyDropdownList", args = 0)]
    pub fn immediate_destroy_dropdown_list(self) -> ();

    #[method(name = "OnSelectItem", args = 1)]
    pub fn on_select_item(self, toggle: crate::unity_engine::ui::toggle::Toggle) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "tm_pro-tmp_dropdown")]
impl TMP_Dropdown {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Dropdown),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_DropdownMethods>::ctor(this);
        this
    }
}
