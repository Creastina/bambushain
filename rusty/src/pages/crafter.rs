use yew::prelude::*;
use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use web_sys::HtmlInputElement;
use crate::ui::modal::{PicoConfirm, PicoModal, PicoAlert};
use crate::api::crafter::{create_crafter, delete_crafter, MyCrafter, update_crafter};
use crate::api::{CONFLICT, NO_CONTENT, NOT_FOUND};

#[derive(Properties, PartialEq, Clone)]
struct ModifyCrafterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    is_loading: bool,
    #[prop_or_default]
    crafter: sheef_entities::Crafter,
    on_save: Callback<sheef_entities::Crafter>,
}

#[function_component(ModifyCrafterModal)]
fn modify_crafter_modal(props: &ModifyCrafterModalProps) -> Html {
    let job_state = use_state_eq(|| AttrValue::from(props.crafter.job.clone()));
    let level_state = use_state_eq(|| AttrValue::from(props.crafter.level.clone()));

    let on_close = props.on_close.clone();
    let on_save = {
        let job_state = job_state.clone();
        let level_state = level_state.clone();

        let on_save = props.on_save.clone();

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();
            let crafter = sheef_entities::Crafter {
                job: (*job_state).to_string(),
                level: (*level_state).to_string(),
            };

            on_save.emit(crafter);
        })
    };
    let update_job = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), job_state.clone());
    let update_level = use_callback(|evt: InputEvent, state| state.set(evt.target_unchecked_into::<HtmlInputElement>().value().into()), level_state.clone());

    html!(
        <PicoModal title="Crafter hinzufügen" on_close={on_close.clone()} open={true} buttons={html!(
            <>
                <button onclick={move |_| on_close.emit(())} type="button" class="secondary">{"Abbrechen"}</button>
                <button form="create-crafter-modal" aria-busy={props.is_loading.to_string()} type="submit">{props.save_label.clone()}</button>
            </>
        )}>
            {if props.has_error {
                html!(<p data-msg="negative">{props.error_message.clone()}</p>)
            } else {
                html!()
            }}
            <form onsubmit={on_save} id="create-crafter-modal">
                <label for="job">{"Job"}</label>
                <input oninput={update_job} readonly={props.is_loading} type="text" value={(*job_state).clone()} required={true} id="job" name="job" />
                <label for="level">{"Level"}</label>
                <input oninput={update_level} readonly={props.is_loading} type="text" value={(*level_state).clone()} required={true} id="level" name="level" />
            </form>
        </PicoModal>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct TableBodyProps {
    crafter: Vec<sheef_entities::Crafter>,
}

#[derive(PartialEq, Clone)]
enum CrafterActions {
    Edit(sheef_entities::Crafter),
    Delete(sheef_entities::Crafter),
    Closed,
}

#[derive(PartialEq, Clone)]
enum ErrorState {
    Edit,
    Delete,
    None,
}

#[function_component(TableBody)]
fn table_body(props: &TableBodyProps) -> Html {
    log::debug!("Initialize crafter table body state and callbacks");
    let action_state = use_state_eq(|| CrafterActions::Closed);

    let error_state = use_state_eq(|| ErrorState::None);
    let loading_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let crafter_query_state = use_query_value::<MyCrafter>(().into());

    let edit_crafter_click = use_callback(|crafter: sheef_entities::Crafter, state| state.set(CrafterActions::Edit(crafter)), action_state.clone());
    let delete_crafter_click = use_callback(|crafter: sheef_entities::Crafter, state| state.set(CrafterActions::Delete(crafter)), action_state.clone());

    let on_modal_close = {
        let action_state = action_state.clone();

        let crafter_query_state = crafter_query_state.clone();

        Callback::from(move |_| {
            let action_state = action_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            yew::platform::spawn_local(async move {
                action_state.set(CrafterActions::Closed);

                let _ = crafter_query_state.refresh().await;
            });
        })
    };
    let on_modal_delete = {
        let action_state = action_state.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();

        let crafter_query_state = crafter_query_state.clone();

        Callback::from(move |crafter: sheef_entities::Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            loading_state.set(true);

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let action_state = action_state.clone();

            let error_message_state = error_message_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_crafter(crafter.clone()).await {
                    NO_CONTENT => ErrorState::None,
                    NOT_FOUND => {
                        error_message_state.set(AttrValue::from("Der Crafter konnte nicht gefunden werden"));
                        ErrorState::Delete
                    }
                    _ => {
                        error_message_state.set(AttrValue::from("Der Crafter konnte nicht gelöscht werden, bitte wende dich an Azami"));
                        ErrorState::Delete
                    }
                });
                loading_state.set(false);
                action_state.set(CrafterActions::Closed);

                let _ = crafter_query_state.refresh().await;
            });
        })
    };
    let on_modal_save = {
        let on_modal_close = on_modal_close.clone();

        let error_state = error_state.clone();
        let loading_state = loading_state.clone();

        let error_message_state = error_message_state.clone();

        let action_state = action_state.clone();

        Callback::from(move |crafter: sheef_entities::Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            loading_state.set(true);
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();

            let error_message_state = error_message_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            let action_state = action_state.clone();

            yew::platform::spawn_local(async move {
                let event_crafter = match (*action_state).clone() {
                    CrafterActions::Edit(crafter) => crafter,
                    _ => unreachable!(),
                };

                error_state.set(match update_crafter(event_crafter.job, crafter).await {
                    NO_CONTENT => {
                        let _ = crafter_query_state.refresh().await;
                        on_modal_close.emit(());
                        ErrorState::Edit
                    }
                    CONFLICT => {
                        error_message_state.set(AttrValue::from("Ein Crafter mit diesem Job existiert bereits"));
                        ErrorState::Edit
                    }
                    NOT_FOUND => {
                        error_message_state.set(AttrValue::from("Der Crafter konnte nicht gefunden werden"));
                        ErrorState::Edit
                    }
                    _ => {
                        error_message_state.set(AttrValue::from("Der Crafter konnte nicht gespeichert werden, bitte wende dich an Azami"));
                        ErrorState::None
                    }
                });
                loading_state.set(false)
            })
        })
    };

    html!(
        <>
            <tbody>
                {for props.crafter.iter().map(|crafter|{
                    let edit_crafter = crafter.clone();
                    let delete_crafter = crafter.clone();

                    let edit_crafter_click = edit_crafter_click.clone();
                    let delete_crafter_click = delete_crafter_click.clone();

                    html!(
                        <tr>
                            <td>{crafter.job.clone()}</td>
                            <td>{crafter.level.clone()}</td>
                            <td>
                                <div class="gap-row">
                                    <button onclick={move |_| edit_crafter_click.emit(edit_crafter.clone())} type="button" class="outline">{"Bearbeiten"}</button>
                                    <button onclick={move |_| delete_crafter_click.emit(delete_crafter.clone())} type="button" class="outline">{"Löschen"}</button>
                                </div>
                            </td>
                        </tr>
                    )}
                )}
            </tbody>
            {match (*action_state).clone() {
                CrafterActions::Edit(crafter) => html!(<ModifyCrafterModal title={format!("Crafter {} bearbeiten", crafter.job)} save_label="Crafter speichern" on_save={on_modal_save} on_close={on_modal_close} crafter={crafter} error_message={(*error_message_state).clone()} has_error={*error_state == ErrorState::Edit} is_loading={*loading_state} />),
                CrafterActions::Delete(crafter) => {
                    let cloned_crafter = crafter.clone();
                    html!(<PicoConfirm open={true} on_confirm={move |_| on_modal_delete.emit(cloned_crafter.clone())} on_decline={on_modal_close} confirm_label="Crafter löschen" title="Crafter löschen" message={format!("Soll der Crafter {} auf Level {} wirklich gelöscht werden?", crafter.job, crafter.level)} />)
                }
                CrafterActions::Closed => html!(),
            }}
            {match (*error_state).clone() {
                ErrorState::Delete => html!(<PicoAlert open={true} title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={move |_| error_state.set(ErrorState::None)} />),
                _ => html!()
            }}
        </>
    )
}

#[function_component(CrafterPage)]
pub fn crafter_page() -> Html {
    log::debug!("Render crafter page");
    log::debug!("Initialize state and callbacks");
    let crafter_query_state = use_query_value::<MyCrafter>(().into());

    let initially_loaded_state = use_state_eq(|| false);
    let open_create_crafter_modal_state = use_state_eq(|| false);

    let state = use_state_eq(|| vec![] as Vec<sheef_entities::Crafter>);

    let error_state = use_state_eq(|| false);
    let loading_state = use_state_eq(|| false);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let open_create_crafter_modal_click = use_callback(|_, open_create_crafter_modal_state| open_create_crafter_modal_state.set(true), open_create_crafter_modal_state.clone());
    let on_modal_close = use_callback(|_, state| state.set(false), open_create_crafter_modal_state.clone());
    let on_modal_save = {
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();

        let error_message_state = error_message_state.clone();

        let crafter_query_state = crafter_query_state.clone();

        Callback::from(move |crafter: sheef_entities::Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            loading_state.set(true);

            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();

            let error_message_state = error_message_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_crafter(crafter).await {
                    Ok(_) => {
                        open_create_crafter_modal_state.clone().set(false);
                        let _ = crafter_query_state.refresh().await;
                        false
                    }
                    Err(err) => {
                        error_message_state.set(AttrValue::from(if err == CONFLICT {
                            "Ein Crafter mit diesem Job existiert bereits"
                        } else {
                            "Der Crafter konnte nicht hinzugefügt werden, bitte wende dich an Azami"
                        }));
                        true
                    }
                });
                loading_state.set(false);
            });
        })
    };

    match crafter_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initially_loaded_state {
                return html!(<p data-msg="info">{"Deine Crafter werden geladen"}</p>);
            }
        }
        Some(Ok(crafter)) => {
            log::debug!("Loaded crafter");
            initially_loaded_state.set(true);
            state.set(crafter.crafter.clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {}", err);
            return html!(<p data-msg="negative">{"Deine Crafter konnten nicht geladen werden, bitte wende dich an Azami"}</p>);
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Meine Crafter"}</title>
            </Helmet>
            <h1>{"Meine Crafter"}</h1>
            <nav>
                <ul>
                    <li>
                        <button onclick={open_create_crafter_modal_click} type="button">{"Crafter hinzufügen"}</button>
                        {if *open_create_crafter_modal_state {
                            html!(
                                <ModifyCrafterModal error_message={(*error_message_state).clone()} has_error={*error_state} is_loading={*loading_state} on_close={on_modal_close} title="Crafter hinzufügen" save_label="Crafter hinzufügen" on_save={on_modal_save} />
                            )
                        } else {
                            html!()
                        }}
                    </li>
                </ul>
            </nav>
            <table role="grid">
                <thead>
                <tr>
                    <th>{"Job"}</th>
                    <th>{"Level"}</th>
                    <th>{"Aktionen"}</th>
                </tr>
                </thead>
                <TableBody crafter={(*state).clone()} />
            </table>
        </>
    )
}