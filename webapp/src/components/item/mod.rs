use game_data::{item::Item, lang::Nameable, LanguageData};
use recordkeeper::dlc::CRAFTED_ITEM_ID;
use yew::prelude::*;

use crate::{data::Data, lang::Text};

use super::select::HtmlName;

pub mod edit;

#[derive(Clone, PartialEq, Copy)]
pub struct HtmlItem(pub Item);

#[derive(Properties, PartialEq)]
pub struct ItemDisplayProps {
    pub item: Item,
}

#[function_component]
pub fn ItemDisplay(props: &ItemDisplayProps) -> Html {
    let data = use_context::<Data>().unwrap();

    html! {
        <>
            <span><small>{props.item.id}{". "}</small></span>
            <span>
                {if props.item.id != u32::from(CRAFTED_ITEM_ID) {
                    html!(<>{props.item.get_name_str(data.lang())}</>)
                } else {
                    html!(<b><Text path="item_masha" /></b>)
                }}
            </span>
            <span><small>{" ("}<Text path={format!("item_rarity_{}", props.item.rarity.lang_id())} />{")"}</small></span>
        </>
    }
}

impl HtmlName for HtmlItem {
    fn get_name_html(&self, language: &LanguageData) -> Html {
        html!(<ItemDisplay item={self.0} />)
    }

    fn get_search_query<'a, 'b: 'a>(&'a self, language: &'b LanguageData) -> Option<&'a str> {
        self.0.get_name(language).map(|t| t.text())
    }

    fn get_name_for_filter<'a, 'b: 'a>(&'a self, language: &'b LanguageData) -> Option<&'a str> {
        self.0.get_name(language).map(|t| t.text_lower())
    }
}
