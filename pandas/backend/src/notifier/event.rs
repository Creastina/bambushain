use std::sync::Arc;
use std::time::Duration;

use actix_web::rt::time::interval;
use actix_web::Responder;
use actix_web_lab::sse;
use parking_lot::Mutex;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

use bamboo_common::core::entities::{Event, User};

use crate::sse::event;

pub(crate) struct EventBroadcaster {
    inner: Mutex<EventBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct EventBroadcasterInner {
    clients: Vec<(Sender<sse::Event>, User)>,
}

impl EventBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(EventBroadcaster {
            inner: Mutex::new(EventBroadcasterInner::default()),
        });
        EventBroadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        let mut ok_clients = Vec::new();
        for (client, user) in clients {
            if let Err(err) = Self::send_comment(client.clone(), event::Comment::Ping).await {
                log::info!("Failed to send ping {err}");
            } else {
                ok_clients.push((client.clone(), user));
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(&self, user: User) -> impl Responder {
        log::debug!("Open channel using tokio");
        let (tx, rx) = tokio::sync::mpsc::channel::<sse::Event>(10);

        log::debug!("Send connected message");
        if let Err(err) = Self::send_comment(tx.clone(), event::Comment::Connected).await {
            log::error!("Failed to send message {err}")
        }
        self.inner.lock().clients.push((tx, user));

        sse::Sse::from_infallible_receiver(rx).with_keep_alive(Duration::from_secs(60))
    }

    fn send_event(&self, evt: event::Event) {
        let clients = self.inner.lock().clients.clone();
        log::debug!("Has {} clients registered", clients.len());
        for (client, user) in clients {
            Self::send_message(client, user, evt.clone())
        }
    }

    fn send_message(client: Sender<sse::Event>, user: User, evt: event::Event) {
        let is_private_event_of_current_user =
            evt.event.is_private && Some(user.id) == evt.event.user_id;
        let is_in_same_grove = !evt.event.is_private && evt.event.grove_id == user.grove_id;
        if is_private_event_of_current_user || is_in_same_grove {
            actix_web::rt::spawn(async move {
                log::debug!("Send event data");
                log::debug!("Sending message with data {evt:#?}");
                if let Err(err) = client.send(evt.into()).await {
                    log::error!("Failed to send message {err}");
                }
            });
        }
    }

    async fn send_comment(
        client: Sender<sse::Event>,
        evt: event::Comment,
    ) -> Result<(), SendError<sse::Event>> {
        client
            .send(sse::Event::Comment(bytestring::ByteString::from(
                serde_json::to_string(&evt).unwrap(),
            )))
            .await
    }

    pub fn notify_create(&self, evt: Event) {
        self.send_event(event::Event::created(evt))
    }

    pub fn notify_update(&self, evt: Event) {
        self.send_event(event::Event::updated(evt))
    }

    pub fn notify_delete(&self, evt: Event) {
        self.send_event(event::Event::deleted(evt))
    }
}
