use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

#[derive(PartialEq)]
pub enum FilterState {
    All,
    Active,
    Completed,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub checked: bool,
    pub contents: String,
}

pub fn app(cx: Scope<()>) -> Element {
    let (todos, set_todos) = use_state(&cx, im_rc::HashMap::<u32, TodoItem>::default);
    let (filter, set_filter) = use_state(&cx, || FilterState::All);
    let (draft, set_draft) = use_state(&cx, || "".to_string());
    let (todo_id, set_todo_id) = use_state(&cx, || 0);

    // Filter the todos based on the filter state
    let mut filtered_todos = todos
        .iter()
        .filter(|(_, item)| match filter {
            FilterState::All => true,
            FilterState::Active => !item.checked,
            FilterState::Completed => item.checked,
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
    filtered_todos.sort_unstable();

    let show_clear_completed = todos.values().any(|todo| todo.checked);
    let items_left = filtered_todos.len();
    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };

    cx.render(rsx!{
        section { class: "todoapp",
            style { [include_str!("./assets/todomvc.css")] }
            div {
                header { class: "header",
                    h1 {"todos"}
                    input {
                        class: "new-todo",
                        placeholder: "What needs to be done?",
                        value: "{draft}",
                        autofocus: "true",
                        oninput: move |evt| set_draft(evt.value.clone()),
                        onkeydown: move |evt| {
                            if evt.key == "Enter" && !draft.is_empty() {
                                set_todos.make_mut().insert(
                                    *todo_id,
                                    TodoItem {
                                        id: *todo_id,
                                        checked: false,
                                        contents: draft.clone(),
                                    },
                                );
                                set_todo_id(todo_id + 1);
                                set_draft("".to_string());
                            }
                        }
                    }
                }
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| rsx!(todo_entry( key: "{id}", id: *id, set_todos: set_todos  )))
                }
                (!todos.is_empty()).then(|| rsx!(
                    footer { class: "footer",
                        span { class: "todo-count",
                            strong {"{items_left} "}
                            span {"{item_text} left"}
                        }
                        ul { class: "filters",
                            li { class: "All", a { onclick: move |_| set_filter(FilterState::All), "All" }}
                            li { class: "Active", a { onclick: move |_| set_filter(FilterState::Active), "Active" }}
                            li { class: "Completed", a { onclick: move |_| set_filter(FilterState::Completed), "Completed" }}
                        }
                        (show_clear_completed).then(|| rsx!(
                            button {
                                class: "clear-completed",
                                onclick: move |_| set_todos.make_mut().retain(|_, todo| !todo.checked),
                                "Clear completed"
                            }
                        ))
                    }
                ))
            }
        }
        footer { class: "info",
            p {"Double-click to edit a todo"}
            p { "Created by ", a {  href: "http://github.com/jkelleyrtp/", "jkelleyrtp" }}
            p { "Part of ", a { href: "http://todomvc.com", "TodoMVC" }}
        }
    })
}

#[derive(Props)]
pub struct TodoEntryProps<'a> {
    set_todos: &'a UseState<im_rc::HashMap<u32, TodoItem>>,
    id: u32,
}

pub fn todo_entry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    let (is_editing, set_is_editing) = use_state(&cx, || false);

    let todos = cx.props.set_todos.get();
    let todo = &todos[&cx.props.id];
    let completed = if todo.checked { "completed" } else { "" };
    let editing = if *is_editing { "editing" } else { "" };

    rsx!(cx, li {
        class: "{completed} {editing}",
        onclick: move |_| set_is_editing(true),
        onfocusout: move |_| set_is_editing(false),
        div { class: "view",
            input { class: "toggle", r#type: "checkbox", id: "cbg-{todo.id}", checked: "{todo.checked}",
                onchange: move |evt| {
                    cx.props.set_todos.make_mut()[&cx.props.id].checked = evt.value.parse().unwrap();
                }
            }
            label { r#for: "cbg-{todo.id}", pointer_events: "none", "{todo.contents}" }
        }
        is_editing.then(|| rsx!{
            input {
                class: "edit",
                value: "{todo.contents}",
                oninput: move |evt| cx.props.set_todos.make_mut()[&cx.props.id].contents = evt.value.clone(),
                autofocus: "true",
                onkeydown: move |evt| {
                    match evt.key.as_str() {
                        "Enter" | "Escape" | "Tab" => set_is_editing(false),
                        _ => {}
                    }
                },
            }
        })
    })
}
