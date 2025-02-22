use std::time::Duration;

use futures::StreamExt;
use tokio::{spawn, sync::mpsc, time::sleep};
use tokio_postgres::AsyncMessage;
use tracing::{debug, error, warn};

use crate::domain::db_id::DbId;

use super::{db::Repo, errors::NotificationCenterError};

#[derive(Clone)]
pub struct NotificationCenter {
    repo: Repo,
    daemon_tx: Option<mpsc::Sender<NotificationCenterDaemonCommand>>,
}

impl NotificationCenter {
    pub fn new(repo: Repo) -> Self {
        Self {
            repo,
            daemon_tx: None,
        }
    }

    pub async fn start_daemon(&mut self) -> Result<(), NotificationCenterError> {
        let (tx, rx) = mpsc::channel::<NotificationCenterDaemonCommand>(32);
        let maintainence_tx = tx.clone();
        self.daemon_tx = Some(tx.clone());
        let repo = self.repo.clone();
        let mut daemon = NotificationCenterDaemon::new(tx, rx);

        daemon.start_repo_listener(repo).await?;

        spawn(async move {
            daemon.listen().await;
            debug!("Notification daemon shut down");
        });

        spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;
                let _ = maintainence_tx
                    .send(NotificationCenterDaemonCommand::Maintainance)
                    .await
                    .inspect_err(|e| error!("Could not send the maintainance notification {}", e));
            }
        });

        Ok(())
    }

    pub async fn subscribe(
        &self,
        topics: Vec<ListenerTopic>,
    ) -> Result<ListenerHandle, NotificationCenterError> {
        let (tx, rx) = tokio::sync::mpsc::channel::<Notification>(16);
        let listener = Listener { topics, tx };

        self.daemon_tx
            .as_ref()
            .ok_or(NotificationCenterError::SubscriptionFailed)?
            .send(NotificationCenterDaemonCommand::AddListener(listener))
            .await
            .map_err(|e| NotificationCenterError::DaemonDead(e.to_string()))?;

        Ok(ListenerHandle::new(rx))
    }
}

#[derive(Debug, Clone)]
enum NotificationCenterDaemonCommand {
    AddListener(Listener),
    Maintainance,
    HandleNotification(Notification),
}

struct NotificationCenterDaemon {
    listeners: Vec<Listener>,
    tx: mpsc::Sender<NotificationCenterDaemonCommand>,
    rx: mpsc::Receiver<NotificationCenterDaemonCommand>,
}

impl NotificationCenterDaemon {
    fn new(
        tx: mpsc::Sender<NotificationCenterDaemonCommand>,
        rx: mpsc::Receiver<NotificationCenterDaemonCommand>,
    ) -> Self {
        Self {
            listeners: Vec::new(),
            tx,
            rx,
        }
    }

    async fn start_repo_listener(&self, repo: Repo) -> Result<(), NotificationCenterError> {
        let (client, mut connection) = repo
            .new_connection()
            .await
            .map_err(|e| NotificationCenterError::DaemonFailedToStart(e.to_string()))?;

        let (tx, mut rx) = futures::channel::mpsc::channel::<AsyncMessage>(64);

        let stream = futures::stream::poll_fn(move |cx| {
            connection.poll_message(cx).map_err(|e| panic!("{}", e))
        })
        .forward(tx);

        client
            .batch_execute(
                r"
                LISTEN post_notification;
                LISTEN comment_notification;
            ",
            )
            .await
            .map_err(|e| NotificationCenterError::DaemonFailedToStart(e.to_string()))?;

        spawn(stream);

        let daemon_tx = self.tx.clone();

        spawn(async move {
            debug!("Started listener");

            while let Some(AsyncMessage::Notification(msg)) = rx.next().await {
                if let Ok(notification) = Notification::try_from(msg) {
                    let _ = daemon_tx
                        .send(NotificationCenterDaemonCommand::HandleNotification(
                            notification,
                        ))
                        .await
                        .inspect_err(|e| warn!("Failed to send notification: {}", e));
                } else {
                    warn!("Received malformed db message");
                }
            }

            debug!("Stopped listener");
        });

        Ok(())
    }

    async fn listen(&mut self) {
        while let Some(message) = self.rx.recv().await {
            use NotificationCenterDaemonCommand::*;

            match message {
                AddListener(listener) => self.add_listener(listener),
                Maintainance => self.maintainence(),
                HandleNotification(notification) => self.handle_notification(notification).await,
            }
        }
    }

    fn add_listener(&mut self, listener: Listener) {
        self.listeners.push(listener);
    }

    fn maintainence(&mut self) {
        self.listeners.retain(|listener| !listener.tx.is_closed());
    }

    async fn handle_notification(&self, notification: Notification) {
        for listener in &self.listeners {
            if listener.cares_about(&notification) {
                let _ = listener
                    .tx
                    .send(notification.clone())
                    .await
                    .inspect_err(|e| debug!("Could not inform listener: {}", e));
            }
        }
    }
}

pub struct ListenerHandle {
    buffer: Vec<Notification>,
    rx: mpsc::Receiver<Notification>,
}

impl ListenerHandle {
    pub fn new(rx: mpsc::Receiver<Notification>) -> Self {
        let buffer = Vec::with_capacity(16);
        Self { buffer, rx }
    }

    pub async fn receive(&mut self) -> Option<Vec<Notification>> {
        let length = self.buffer.capacity();
        let received = self.rx.recv_many(&mut self.buffer, length).await;

        if received == 0 {
            None
        } else {
            Some(self.buffer.drain(..).collect())
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ListenerTopic {
    User(DbId),
    Post(DbId),
}

impl ListenerTopic {
    fn matches(&self, notification: &Notification) -> bool {
        match (self, notification) {
            (ListenerTopic::User(user), Notification::Post(note_post)) => {
                *user == note_post.author_id
            }
            (ListenerTopic::User(user), Notification::Comment(note_comment)) => {
                *user == note_comment.author_id
            }
            (ListenerTopic::Post(post), Notification::Post(note_post)) => {
                *post == note_post.post_id
            }
            (ListenerTopic::Post(post), Notification::Comment(note_comment)) => {
                *post == note_comment.post_id
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Listener {
    topics: Vec<ListenerTopic>,
    tx: mpsc::Sender<Notification>,
}

impl Listener {
    fn cares_about(&self, notification: &Notification) -> bool {
        self.topics.iter().any(|topic| topic.matches(notification))
    }
}

#[derive(Debug, Clone)]
pub struct PostNotification {
    pub author_id: DbId,
    pub post_id: DbId,
}

impl TryFrom<&str> for PostNotification {
    type Error = NotificationCenterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            return Err(NotificationCenterError::ParsingFailed);
        }

        let post_id = parts[0]
            .parse()
            .map_err(|_| NotificationCenterError::ParsingFailed)?;
        let author_id = parts[1]
            .parse()
            .map_err(|_| NotificationCenterError::ParsingFailed)?;

        Ok(PostNotification { author_id, post_id })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommentNotification {
    pub author_id: DbId,
    pub post_id: DbId,
    pub comment_id: DbId,
}

impl TryFrom<&str> for CommentNotification {
    type Error = NotificationCenterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 3 {
            return Err(NotificationCenterError::ParsingFailed);
        }

        let comment_id = parts[0]
            .parse()
            .map_err(|_| NotificationCenterError::ParsingFailed)?;
        let post_id = parts[1]
            .parse()
            .map_err(|_| NotificationCenterError::ParsingFailed)?;
        let author_id = parts[2]
            .parse()
            .map_err(|_| NotificationCenterError::ParsingFailed)?;

        Ok(CommentNotification {
            author_id,
            post_id,
            comment_id,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Notification {
    Post(PostNotification),
    Comment(CommentNotification),
}

impl TryFrom<tokio_postgres::Notification> for Notification {
    type Error = NotificationCenterError;

    fn try_from(value: tokio_postgres::Notification) -> Result<Self, Self::Error> {
        match value.channel() {
            "post_notification" => PostNotification::try_from(value.payload())
                .map(|notification| Notification::Post(notification)),
            "comment_notification" => CommentNotification::try_from(value.payload())
                .map(|notification| Notification::Comment(notification)),
            _ => Err(NotificationCenterError::ParsingFailed),
        }
    }
}
