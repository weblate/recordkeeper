use yew::prelude::*;

use crate::{
    components::{
        edit::Editor,
        select::{HtmlName, Options, SearchSelect},
    },
    data::Data,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
pub struct SlotProps<E: Editor + PartialEq, I: PartialEq + 'static>
where
    E::Target: PartialEq,
{
    pub editor: E,
    pub possible_values: &'static [I],
    pub id_mapper: Callback<&'static I, E::Target>,
}

#[function_component]
pub fn SlotInput<E, I>(props: &SlotProps<E, I>) -> Html
where
    E: Editor + PartialEq,
    I: PartialEq + 'static + Clone + HtmlName,
    E::Target: PartialEq,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let current = props.editor.get(save_context.get().get().save());
    // TODO
    let current = props
        .possible_values
        .iter()
        .enumerate()
        .find(|(_, v)| props.id_mapper.emit(v) == current)
        .map(|(i, _)| i);

    let options: Options<_> = props.possible_values.into();

    let on_type_select = {
        let editor = props.editor;
        let save_context = save_context.clone();
        let id_mapper = props.id_mapper.clone();
        let values = props.possible_values;
        Callback::from(move |idx| {
            let item = &values[idx];
            let id = id_mapper.emit(item);
            save_context.edit(move |save| editor.set(save, id))
        })
    };

    html! {
        <SearchSelect<I>
            current={current}
            options={options}
            on_select={on_type_select}
            lang={lang}
        />
    }
}