use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::event::LiveEvent;

type Sender = Box<dyn Fn(LiveEvent) -> bool + Send + Sync>;

pub struct Channel {
    pub name: String,
    max_clients: usize,
    clients: Mutex<HashMap<u64, Arc<Sender>>>,
    next_id: Mutex<u64>,
}

impl Channel {
    pub fn new(name: impl Into<String>, max_clients: usize) -> Self {
        Self {
            name: name.into(),
            max_clients,
            clients: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }

    pub fn subscribe(&self, sender: Sender) -> Option<u64> {
        let mut clients = self.clients.lock().unwrap();
        if clients.len() >= self.max_clients {
            return None;
        }
        let mut id = self.next_id.lock().unwrap();
        let client_id = *id;
        *id += 1;
        clients.insert(client_id, Arc::new(sender));
        Some(client_id)
    }

    pub fn unsubscribe(&self, client_id: u64) {
        self.clients.lock().unwrap().remove(&client_id);
    }

    pub fn broadcast(&self, event: LiveEvent) -> usize {
        let clients = self.clients.lock().unwrap();
        let mut delivered = 0;
        let mut dead = Vec::new();
        for (id, sender) in clients.iter() {
            if sender(event.clone()) {
                delivered += 1;
            } else {
                dead.push(*id);
            }
        }
        drop(clients);
        if !dead.is_empty() {
            let mut clients = self.clients.lock().unwrap();
            for id in dead {
                clients.remove(&id);
            }
        }
        delivered
    }

    pub fn client_count(&self) -> usize {
        self.clients.lock().unwrap().len()
    }
}

pub struct ChannelRegistry {
    max_clients: usize,
    channels: Mutex<HashMap<String, Arc<Channel>>>,
}

impl ChannelRegistry {
    pub fn new(max_clients: usize) -> Self {
        Self { max_clients, channels: Mutex::new(HashMap::new()) }
    }

    pub fn get_or_create(&self, name: &str) -> Arc<Channel> {
        let mut channels = self.channels.lock().unwrap();
        if let Some(ch) = channels.get(name) {
            return ch.clone();
        }
        let ch = Arc::new(Channel::new(name, self.max_clients));
        channels.insert(name.to_string(), ch.clone());
        ch
    }

    pub fn broadcast(&self, channel: &str, event: LiveEvent) -> usize {
        let channels = self.channels.lock().unwrap();
        if let Some(ch) = channels.get(channel) {
            ch.broadcast(event)
        } else {
            0
        }
    }

    pub fn channel_names(&self) -> Vec<String> {
        self.channels.lock().unwrap().keys().cloned().collect()
    }
}
