#[macro_use]
extern crate yew;
extern crate frontend;

use yew::prelude::*;
use yew::html::{Scope,ComponentUpdate};
use yew::services::fetch::FetchTask;

use frontend::feeder;
use frontend::{Channel, Item, Category};

struct Context {
    feeder: feeder::FeederService,
}

struct Model {
    channels: Vec<Channel>,
    items: Vec<Item>,
    current_item: Option<usize>,
    task: Option<FetchTask>,
}

enum Msg {
    FetchChannels,
    ChannelsReady(Result<Vec<Channel>, ()>),
    FetchItems(i32),
    ItemsReady(Result<Vec<Item>, ()>),
    Details(usize),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            channels: Vec::new(),
            items: Vec::new(),
            current_item: None,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::FetchChannels => {
                let callback = context.send_back(Msg::ChannelsReady);
                self.task = Some(context.feeder.channels(callback));
            },
            Msg::ChannelsReady(Ok(channels)) => {
                self.channels = channels;
            },
            Msg::FetchItems(id) => {
                let callback = context.send_back(Msg::ItemsReady);
                self.task = Some(context.feeder.items(id, callback));
            },
            Msg::ItemsReady(Ok(items)) => {
                self.items = items;
            },
            Msg::Details(i) => {
                self.current_item = if self.current_item == Some(i) {
                    None
                } else {
                    Some(i)
                };
            }
            _ => {},
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let view_channel = |chan: &Channel| {
            let id = chan.id;
            html! {
                <a href="#", onclick=move|_| Msg::FetchItems(id),>{ &chan.title }</a>
            }
        };
        let view_item_details = |item: &Item| {
            let title = item.title.clone().unwrap_or("link".to_owned());
            let link = item.link.clone().unwrap_or("".to_owned());
            let description = item.description.clone().unwrap_or("".to_owned());
            let author = item.author.clone().unwrap_or("".to_owned());
            let guid = item.guid.clone().unwrap_or("".to_owned());
            let pub_date = item.pub_date.clone().unwrap_or("".to_owned());
            html! {
                <div class="details-content",>
                    <h4><a href=link,>{title}</a></h4>
                    <div class="details-iframe-container",>
                        <iframe class="details-iframe", sandbox="", srcdoc=description,></iframe>
                    </div>
                    <footer>
                        <div>{pub_date}</div>
                        <div>{author}</div>
                        <div>{guid}</div>
                    </footer>
                </div>
            }
        };
        let view_item = |(idx, item): (usize, &Item)| {
            let title = item.title.clone().unwrap_or(String::new());
            html! {
                <div>
                    <div class="title", onclick=move|_| Msg::Details(idx),>{title}</div>
                </div>
            }
        };
        html! {
            <div id="site",>
                <header class="site-header",> {"Feeder"} </header>
                <nav class="site-nav",>
                    { for self.channels.iter().map(view_channel) }
                </nav>
                <section class="site-details",>
                {
                    if let Some(idx) = self.current_item {
                        view_item_details(&self.items[idx])
                    } else {
                        html!{<div class="details-content",></div>}
                    }
                }
                </section>
                <main class="site-main",>
                    { for self.items.iter().enumerate().map(view_item) }
                </main>
                <footer class="site-footer",> { "footer" } </footer>
            </div>
        }
    }
}

fn main() {
    yew::initialize();

    let context = Context {
        feeder: feeder::FeederService::new(),
    };

    let mut app: Scope<_, Model> = Scope::new(context);
    let mut sender = app.get_env().sender();
    sender.send(ComponentUpdate::Message(Msg::FetchChannels));
    app.mount_to_body();

    yew::run_loop();
}
