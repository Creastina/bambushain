use bounce::helmet::Helmet;
use bounce::query::use_query_value;
use yew::prelude::*;
use yew_cosmo::prelude::*;

use pandaparty_entities::prelude::*;

use crate::api::*;

#[derive(Properties, PartialEq, Clone)]
struct ModifyCrafterModalProps {
    on_close: Callback<()>,
    title: AttrValue,
    save_label: AttrValue,
    error_message: AttrValue,
    has_error: bool,
    #[prop_or_default]
    crafter: Crafter,
    on_save: Callback<Crafter>,
    on_error_close: Callback<()>,
}

#[function_component(ModifyCrafterModal)]
fn modify_crafter_modal(props: &ModifyCrafterModalProps) -> Html {
    let job_state: UseStateHandle<AttrValue> = use_state_eq(|| props.crafter.job.clone().into());
    let level_state: UseStateHandle<AttrValue> = use_state_eq(|| props.crafter.level.clone().unwrap_or_default().into());

    let on_close = props.on_close.clone();
    let on_save = use_callback(|_, (job_state, level_state, on_save)| on_save.emit(Crafter::new((*job_state).to_string(), (*level_state).to_string())), (job_state.clone(), level_state.clone(), props.on_save.clone()));
    let update_job = use_callback(|value: AttrValue, state| state.set(value), job_state.clone());
    let update_level = use_callback(|value: AttrValue, state| state.set(value), level_state.clone());

    html!(
        <>
            <CosmoModal title={props.title.clone()} is_form={true} on_form_submit={on_save} buttons={html!(
                <>
                    <CosmoButton on_click={on_close} label="Abbrechen" />
                    <CosmoButton label={props.save_label.clone()} is_submit={true} />
                </>
            )}>
                if props.has_error {
                    <CosmoMessage message_type={CosmoMessageType::Negative} message={props.error_message.clone()} />
                }
                <CosmoInputGroup>
                    <CosmoTextBox label="Job" on_input={update_job} value={(*job_state).clone()} required={true} />
                    <CosmoTextBox label="Level" on_input={update_level} value={(*level_state).clone()} required={true} />
                </CosmoInputGroup>
            </CosmoModal>
        </>
    )
}

#[derive(Properties, PartialEq, Clone)]
struct CrafterDetailsProps {
    crafter: Crafter,
    on_delete: Callback<()>,
}

#[derive(PartialEq, Clone)]
enum CrafterActions {
    Edit,
    Delete,
    Closed,
}

#[derive(PartialEq, Clone)]
enum ErrorState {
    Edit,
    Delete,
    None,
}

#[function_component(CrafterDetails)]
fn crafter_details(props: &CrafterDetailsProps) -> Html {
    log::debug!("Initialize crafter details state and callbacks");
    let action_state = use_state_eq(|| CrafterActions::Closed);

    let error_state = use_state_eq(|| ErrorState::None);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let crafter_query_state = use_query_value::<MyCrafter>(().into());

    let edit_crafter_click = use_callback(|_, state| state.set(CrafterActions::Edit), action_state.clone());
    let delete_crafter_click = use_callback(|_, state| state.set(CrafterActions::Delete), action_state.clone());

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

        let error_message_state = error_message_state.clone();

        let crafter_query_state = crafter_query_state.clone();

        let on_delete = props.on_delete.clone();

        let id = props.crafter.id;

        Callback::from(move |_| {
            log::debug!("Modal was confirmed lets execute the request");

            let error_state = error_state.clone();

            let action_state = action_state.clone();

            let error_message_state = error_message_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            let on_delete = on_delete.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match delete_crafter(id).await {
                    Ok(_) => {
                        on_delete.emit(());
                        ErrorState::None
                    }
                    Err(err) => {
                        match err.code {
                            NOT_FOUND => {
                                error_message_state.set("Der Crafter konnte nicht gefunden werden".into());
                                ErrorState::Delete
                            }
                            _ => {
                                error_message_state.set("Der Crafter konnte nicht gelöscht werden, bitte wende dich an Azami".into());
                                ErrorState::Delete
                            }
                        }
                    }
                });
                action_state.set(CrafterActions::Closed);

                let _ = crafter_query_state.refresh().await;
            });
        })
    };
    let on_modal_save = {
        let on_modal_close = on_modal_close.clone();

        let error_state = error_state.clone();

        let error_message_state = error_message_state.clone();

        let action_state = action_state.clone();

        let id = props.crafter.id;

        Callback::from(move |crafter: Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            let on_modal_close = on_modal_close.clone();

            let error_state = error_state.clone();

            let error_message_state = error_message_state.clone();

            let action_state = action_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match update_crafter(id, crafter).await {
                    Ok(_) => {
                        let _ = crafter_query_state.refresh().await;
                        on_modal_close.emit(());
                        action_state.set(CrafterActions::Closed);
                        ErrorState::None
                    }
                    Err(err) => {
                        match err.code {
                            CONFLICT => {
                                error_message_state.set("Ein Crafter mit diesem Job existiert bereits".into());
                                ErrorState::Edit
                            }
                            NOT_FOUND => {
                                error_message_state.set("Der Crafter konnte nicht gefunden werden".into());
                                ErrorState::Edit
                            }
                            _ => {
                                error_message_state.set("Der Crafter konnte nicht gespeichert werden, bitte wende dich an Azami".into());
                                ErrorState::None
                            }
                        }
                    }
                });
            })
        })
    };
    let on_error_close = use_callback(|_, state| state.set(ErrorState::None), error_state.clone());

    html!(
        <>
            <CosmoTitle title={props.crafter.job.clone()} />
            <CosmoToolbar>
                <CosmoToolbarGroup>
                    <CosmoButton on_click={edit_crafter_click} label="Bearbeiten" />
                    <CosmoButton on_click={delete_crafter_click} label="Löschen" />
                </CosmoToolbarGroup>
            </CosmoToolbar>
            <CosmoKeyValueList>
                if let Some(level) = props.crafter.level.clone() {
                    <CosmoKeyValueListItem title="Level">{level}</CosmoKeyValueListItem>
                } else {
                    <CosmoKeyValueListItem title="Level">{"Kein Level festgelegt"}</CosmoKeyValueListItem>
                }
            </CosmoKeyValueList>
            {match (*action_state).clone() {
                CrafterActions::Edit => html!(
                    <ModifyCrafterModal on_error_close={on_error_close.clone()} title={format!("Crafter {} bearbeiten", props.crafter.job)} save_label="Crafter speichern" on_save={on_modal_save} on_close={on_modal_close} crafter={props.crafter.clone()} error_message={(*error_message_state).clone()} has_error={*error_state == ErrorState::Edit} />
                ),
                CrafterActions::Delete => {
                    let crafter = props.crafter.clone();
                    html!(
                        <CosmoConfirm on_confirm={on_modal_delete} on_decline={on_modal_close} confirm_label="Crafter löschen" decline_label="Crafter behalten" title="Crafter löschen" message={format!("Soll der Crafter {} auf Level {} wirklich gelöscht werden?", crafter.job, crafter.level.unwrap_or_default())} />
                    )
                }
                CrafterActions::Closed => html!(),
            }}
            if *error_state == ErrorState::Delete {
                <CosmoAlert alert_type={CosmoAlertType::Negative} close_label="Schließen" title="Ein Fehler ist aufgetreten" message={(*error_message_state).clone()} on_close={on_error_close} />
            }
        </>
    )
}

#[function_component(CrafterPage)]
pub fn crafter_page() -> Html {
    log::debug!("Render crafter page");
    log::debug!("Initialize state and callbacks");
    let crafter_query_state = use_query_value::<MyCrafter>(().into());

    let open_create_crafter_modal_state = use_state_eq(|| false);
    let error_state = use_state_eq(|| false);
    let initial_loaded_state = use_state_eq(|| false);

    let crafter_state = use_state_eq(|| vec![] as Vec<Crafter>);

    let selected_crafter_state = use_state_eq(|| 0);

    let error_message_state = use_state_eq(|| AttrValue::from(""));

    let open_create_crafter_modal_click = use_callback(|_, open_create_crafter_modal_state| open_create_crafter_modal_state.set(true), open_create_crafter_modal_state.clone());
    let on_crafter_select = use_callback(|idx, state| state.set(idx), selected_crafter_state.clone());
    let on_modal_close = use_callback(|_, state| state.set(false), open_create_crafter_modal_state.clone());
    let on_modal_save = {
        let error_state = error_state.clone();
        let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();

        let error_message_state = error_message_state.clone();

        let crafter_query_state = crafter_query_state.clone();

        let selected_crafter_state = selected_crafter_state.clone();

        Callback::from(move |crafter: Crafter| {
            log::debug!("Modal was confirmed lets execute the request");
            let error_state = error_state.clone();
            let open_create_crafter_modal_state = open_create_crafter_modal_state.clone();

            let selected_crafter_state = selected_crafter_state.clone();

            let job = crafter.job.clone();

            let error_message_state = error_message_state.clone();

            let crafter_query_state = crafter_query_state.clone();

            yew::platform::spawn_local(async move {
                error_state.set(match create_crafter(crafter).await {
                    Ok(_) => {
                        open_create_crafter_modal_state.clone().set(false);
                        if let Ok(res) = crafter_query_state.refresh().await {
                            selected_crafter_state.set(res.crafter.iter().position(move |crafter| crafter.job.eq(&job)).unwrap_or(0));
                        }
                        false
                    }
                    Err(err) => {
                        error_message_state.set(if err.code == CONFLICT {
                            "Ein Crafter mit diesem Job existiert bereits"
                        } else {
                            "Der Crafter konnte nicht hinzugefügt werden, bitte wende dich an Azami"
                        }.into());
                        true
                    }
                });
            });
        })
    };
    let on_delete = {
        let crafter_query_state = crafter_query_state.clone();

        let selected_crafter_state = selected_crafter_state.clone();

        Callback::from(move |_| {
            let crafter_query_state = crafter_query_state.clone();

            let selected_crafter_state = selected_crafter_state.clone();

            yew::platform::spawn_local(async move {
                let _ = crafter_query_state.refresh().await;
                selected_crafter_state.set(0);
            })
        })
    };
    let on_error_close = use_callback(|_, state| state.set(false), error_state.clone());

    match crafter_query_state.result() {
        None => {
            log::debug!("Still loading");
            if !*initial_loaded_state {
                return html!(
                    <CosmoProgressRing />
                );
            }
        }
        Some(Ok(res)) => {
            log::debug!("Loaded crafter");
            initial_loaded_state.set(true);
            crafter_state.set(res.crafter.clone());
        }
        Some(Err(err)) => {
            log::warn!("Failed to load {err}");
            return html!(
                <CosmoMessage header="Fehler beim Laden" message="Deine Crafter konnten nicht geladen werden, bitte wende dich an Azami" message_type={CosmoMessageType::Negative} />
            );
        }
    }

    html!(
        <>
            <Helmet>
                <title>{"Meine Crafter"}</title>
            </Helmet>
            <CosmoSideList on_select_item={on_crafter_select} selected_index={*selected_crafter_state} has_add_button={true} add_button_on_click={open_create_crafter_modal_click} add_button_label="Crafter hinzufügen">
                {for (*crafter_state).clone().into_iter().map(|crafter| {
                    CosmoSideListItem::from_label_and_children(crafter.job.clone().into(), html!(
                        <CrafterDetails on_delete={on_delete.clone()} crafter={crafter} />
                    ))
                })}
            </CosmoSideList>
            if *open_create_crafter_modal_state {
                <ModifyCrafterModal on_error_close={on_error_close} error_message={(*error_message_state).clone()} has_error={*error_state} on_close={on_modal_close} title="Crafter hinzufügen" save_label="Crafter hinzufügen" on_save={on_modal_save} />
            }
        </>
    )
}
