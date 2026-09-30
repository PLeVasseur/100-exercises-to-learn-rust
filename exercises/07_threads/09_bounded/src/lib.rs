// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError};
use std::time::Duration;

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

#[derive(Debug)]
pub enum InsertError {
    ChannelBusy { draft: TicketDraft },
    Timeout { draft: TicketDraft },
}

#[derive(Debug)]
pub enum GetError {
    ChannelBusy { id: TicketId },
    Timeout { id: TicketId },
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, InsertError> {
        let (response_channel, reply_channel) = std::sync::mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft: draft.clone(),
                response_channel,
            })
            .map_err(|_| InsertError::ChannelBusy {
                draft: draft.clone(),
            })?;

        let ticket_id = reply_channel
            .recv_timeout(Duration::from_secs(2))
            .map_err(|_| InsertError::Timeout { draft })?;

        Ok(ticket_id)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, GetError> {
        let (response_channel, reply_channel) = std::sync::mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id: id.clone(),
                response_channel,
            })
            .map_err(|_| GetError::ChannelBusy { id: id.clone() })?;

        let possible_ticket = reply_channel
            .recv_timeout(Duration::from_secs(2))
            .map_err(|_| GetError::Timeout { id })?;

        Ok(possible_ticket)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).expect("Client should be alive");
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel
                    .send(ticket.cloned())
                    .expect("Client should be alive");
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
