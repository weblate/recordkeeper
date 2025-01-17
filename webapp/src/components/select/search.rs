use std::{borrow::Cow, rc::Rc};

use fluent::{FluentArgs, FluentValue};
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlInputElement, HtmlSelectElement};
use ybc::*;
use yew::prelude::*;
use yew_feather::ChevronDown;

use game_data::lang::{Filterable, Id, Nameable};
use game_data::LanguageData;

use super::{HtmlName, Options};
use crate::data::Data;
use crate::lang::{Lang, LangManager, Text};

#[derive(Properties)]
pub struct SearchSelectProps<O: Clone + 'static> {
    pub current: Option<usize>,
    /// List of searchable/selectable options
    pub options: Options<O>,
    pub on_select: Callback<usize, ()>,
    pub lang: Rc<LanguageData>,

    #[prop_or_default]
    pub placeholder: AttrValue,
}

#[derive(Properties)]
struct DropdownProps<O>
where
    O: Clone + 'static,
{
    pub open: bool,
    pub visible_options: Vec<(usize, O)>,
    pub on_select: Callback<usize, ()>,
    pub lang: Rc<LanguageData>,
}

/// Select dropdown with searchable options.
#[function_component]
pub fn SearchSelect<O>(props: &SearchSelectProps<O>) -> Html
where
    O: HtmlName + Clone + 'static,
{
    let data = use_context::<Data>().unwrap();
    let lang = use_context::<Lang>().unwrap();

    let value = use_state(|| props.current);
    let value_state = value.clone();
    let lang_id = data.lang_hash();
    use_effect_with_deps(
        move |(_, new, _)| value_state.set(*new),
        (props.options.id(), props.current, lang_id),
    );

    let default_search = use_memo(
        |_| props.search_query(*value, &lang),
        (props.options.id(), *value, lang_id),
    );

    let search_query = use_state(|| (*default_search).clone());
    let focused = use_state(|| false);

    let search_state = search_query.clone();
    // Refresh search query when current value is changed as a prop
    use_effect_with_deps(
        move |_| search_state.set((*default_search).clone()),
        (props.options.id(), *value, lang_id),
    );

    if value.is_some_and(|v| v >= props.options.len()) {
        return html!();
    }

    let lower_search = (*search_query).to_lowercase();
    let visible = props
        .options
        .iter()
        .enumerate()
        .filter(|(_, o)| {
            o.get_name_for_filter(&props.lang, &lang)
                .is_some_and(|n| n.contains(&lower_search))
        })
        .map(|(i, o)| (i, o.clone()))
        .collect::<Vec<_>>();

    let on_select = props.on_select.clone();
    let value_state = value.clone();
    let focus_state = focused.clone();
    let select_callback = Callback::from(move |option| {
        value_state.set(Some(option));
        focus_state.set(false);
        on_select.emit(option);
    });

    let search_state = search_query.clone();
    let update_search_query = Callback::from(move |e: InputEvent| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            search_state.set(input.value().to_string().into());
        }
    });

    let update_focus = |has_focus: bool| {
        let focus_state = focused.clone();
        Callback::from(move |e: FocusEvent| {
            if !has_focus
                && e.related_target().is_some_and(|e| {
                    let html = e.dyn_into::<Element>().unwrap();
                    let tag = html.tag_name();
                    tag == "A" || tag == "DIV"
                })
            {
                return;
            }
            e.prevent_default();
            focus_state.set(has_focus);
        })
    };

    // Having the dropdown inside form control gives it the correct width

    html! {
        <>
            <div class={classes!("control", "has-icons-right")} tabindex="-1">
                <input class="input"
                    value={(*search_query).clone()}
                    oninput={update_search_query}
                    onfocus={update_focus(true)}
                    onblur={update_focus(false)}
                    placeholder={props.placeholder.clone()}
                />
                <Icon classes={classes!("is-right")}>
                    <ChevronDown />
                </Icon>
                <Dropdown<O>
                    open={*focused}
                    visible_options={visible}
                    on_select={select_callback}
                    lang={props.lang.clone()}
                />
            </div>
        </>
    }
}

#[function_component]
fn Dropdown<O>(props: &DropdownProps<O>) -> Html
where
    O: Clone + HtmlName + 'static,
{
    if !props.open {
        return html!();
    }

    let callback = |index| {
        let on_select = props.on_select.clone();
        Callback::from(move |_: MouseEvent| on_select.emit(index))
    };

    html!(
        <Menu classes={classes!("card", "recordkeeper-dropdown")}>
            <MenuList classes={classes!("recordkeeper-dropdown-list")}>
                {for props.visible_options.iter().map(|(index, item)| {
                    html!(<li><a onclick={callback(*index)} class="dropdown-item">{item.get_name_html(&props.lang)}</a></li>)
                })}
            </MenuList>
        </Menu>
    )
}

impl<O: 'static> Options<O> {
    pub fn get(&self, i: usize) -> &O {
        &self.as_slice()[i]
    }

    fn get_if_present(&self, i: usize) -> Option<&O> {
        self.as_slice().get(i)
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn iter(&self) -> std::slice::Iter<O> {
        self.as_slice().iter()
    }

    fn id(&self) -> usize {
        self.as_slice().as_ptr() as usize
    }

    fn as_slice(&self) -> &[O] {
        match self {
            Self::Owned(v) => &v,
            Self::Borrowed(s) => &s,
        }
    }
}

impl<O: Clone + HtmlName + 'static> SearchSelectProps<O> {
    fn search_query(&self, current: Option<usize>, lang: &LangManager) -> AttrValue {
        current
            .and_then(|o| {
                self.options
                    .get_if_present(o)?
                    .get_search_query(&self.lang.clone(), lang)
                    .map(|s| AttrValue::from(s.to_string()))
            })
            .unwrap_or_default()
    }
}

impl<O: 'static> PartialEq for Options<O> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Owned(s1), Self::Owned(s2)) => Rc::ptr_eq(s1, s2),
            (Self::Borrowed(s1), Self::Borrowed(s2)) => std::ptr::eq(*s1, *s2),
            _ => false,
        }
    }
}

impl<O: 'static> From<&'static [O]> for Options<O> {
    fn from(value: &'static [O]) -> Self {
        Self::Borrowed(value)
    }
}

impl<O: 'static> From<Rc<[O]>> for Options<O> {
    fn from(value: Rc<[O]>) -> Self {
        Self::Owned(value)
    }
}

impl<O: 'static> FromIterator<O> for Options<O> {
    fn from_iter<T: IntoIterator<Item = O>>(iter: T) -> Self {
        Self::Owned(iter.into_iter().collect())
    }
}

impl<O: Clone + 'static> PartialEq for SearchSelectProps<O> {
    fn eq(&self, other: &Self) -> bool {
        self.on_select == other.on_select
            && self.options == other.options
            && self.current == other.current
            && Rc::ptr_eq(&self.lang, &other.lang)
    }
}

impl<O: Clone + 'static> PartialEq for DropdownProps<O> {
    fn eq(&self, other: &Self) -> bool {
        self.open == other.open
            && self.on_select == other.on_select
            && std::ptr::eq(
                self.visible_options.as_ptr(),
                other.visible_options.as_ptr(),
            )
            && Rc::ptr_eq(&self.lang, &other.lang)
    }
}

impl<T> HtmlName for T
where
    T: Filterable + Id,
{
    fn get_name_html(&self, language: &LanguageData) -> Html {
        html! {
            <>
                <span><small>{self.id()}{". "}</small></span>
                <span>
                    {self.get_name_str(language).map(Html::from).unwrap_or_else(|| html! {
                        <Text path="unnamed" args={vec![("id".into(), self.id().into())]} />
                    })}
                </span>
            </>
        }
    }

    fn get_search_query<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        html_lang: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        Some(
            self.get_filter(language)
                .map(|t| t.text().into())
                .unwrap_or_else(|| {
                    let args = FluentArgs::from(
                        [(Cow::from("id"), FluentValue::from(self.id()))]
                            .into_iter()
                            .collect(),
                    );
                    html_lang
                        .translate_with_args("unnamed", Some(&args))
                        .to_string()
                        .into()
                }),
        )
    }

    fn get_name_for_filter<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        html_lang: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        Some(
            self.get_filter(language)
                .map(|t| t.text_lower().into())
                .unwrap_or_else(|| {
                    let args = FluentArgs::from(
                        [(Cow::from("id"), FluentValue::from(self.id()))]
                            .into_iter()
                            .collect(),
                    );
                    html_lang
                        .translate_with_args("unnamed", Some(&args))
                        .to_lowercase()
                        .into()
                }),
        )
    }
}
