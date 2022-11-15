mod broadcaster;
mod users;

use broadcaster::{Broadcast, Broadcaster, BroadcasterHandler};
use lunatic::process::{ProcessRef, StartProcess};
use serde::{Deserialize, Serialize};
use submillisecond::http::Uri;
use submillisecond::{router, static_router, Application};
use submillisecond_live_view::prelude::*;
use users::{Users, UsersHandler};

fn main() -> std::io::Result<()> {
    Broadcaster::start_link((), Some("broadcaster"));
    Users::start_link((), Some("usernames"));
    Application::new(router! {
        "/" => Chat::handler()
        "/static" => static_router!("./static")
    })
    .serve("127.0.0.1:3000")
}

#[derive(Clone, Serialize, Deserialize)]
struct Chat {
    name: String,
    color: String,
    broadcaster: Option<ProcessRef<Broadcaster>>,
    messages: Vec<String>,
}

impl LiveView for Chat {
    type Events = (Submit, Broadcast);

    fn render(&self) -> Rendered {
        html! {
            section style={"border: 1px solid " (self.color) ";"} {
                div.messages {
                    @for message in self.messages.iter() {
                        div.message style={"border-bottom: 1px solid " (self.color) ";"} { (message) }
                    }
                }
                form method="post" url="#" @submit=(Submit)
                {
                    p { "Your username is " b { (self.name) } }
                    input
                        autofocus name="message" placeholder="Message" type="text" onfocus="this.value=''"
                        style={"border: 1px solid " (self.color) ";"};
                }
            }
        }
    }

    fn mount(_uri: Uri, socket: Option<Socket>) -> Self {
        let mut broadcaster = None;
        let (name, color) = if socket.is_some() {
            // Mount is also called on the first render too, but we want to only look up names for
            // live view connections.
            broadcaster = Some(ProcessRef::<Broadcaster>::lookup("broadcaster").unwrap());
            broadcaster
                .as_ref()
                .unwrap()
                .subscribe(socket.unwrap().clone());

            let users: ProcessRef<Users> = ProcessRef::lookup("usernames").unwrap();
            users.get_user()
        } else {
            ("unknown".to_owned(), "black".to_owned())
        };

        Chat {
            name,
            color,
            broadcaster,
            messages: vec![],
        }
    }

    fn head() -> Head {
        Head::defaults()
            .with_title("LiveView Chat")
            .with_style(Style::Link("/static/chat.css"))
    }
}

#[derive(Deserialize)]
struct Submit {
    message: String,
}

impl LiveViewEvent<Submit> for Chat {
    fn handle(state: &mut Self, event: Submit) {
        let message = format!("{}: {}", state.name, event.message);
        state.broadcaster.as_mut().unwrap().broadcast(message);
    }
}

impl LiveViewEvent<Broadcast> for Chat {
    fn handle(state: &mut Self, broadcast: Broadcast) {
        state.messages.push(broadcast.0);
    }
}
