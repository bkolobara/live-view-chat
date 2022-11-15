use lunatic::{abstract_process, process::ProcessRef};
use serde::{Deserialize, Serialize};
use submillisecond_live_view::socket::Socket;

#[derive(Serialize, Deserialize)]
pub struct Broadcast(pub String);

pub struct Broadcaster(Vec<Socket>);

#[abstract_process(visibility = pub)]
impl Broadcaster {
    #[init]
    fn init(_: ProcessRef<Self>, _: ()) -> Self {
        Self(vec![])
    }

    #[handle_message]
    fn subscribe(&mut self, socket: Socket) {
        self.0.push(socket);
    }

    #[handle_message]
    fn broadcast(&mut self, message: String) {
        self.0.iter_mut().for_each(|client| {
            let _ = client.send_event(Broadcast(message.clone()));
        });
    }
}
