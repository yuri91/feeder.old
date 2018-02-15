#[macro_use]
extern crate yew;
extern crate frontend;

use yew::prelude::*;

use frontend::feeder;
use frontend::{Channel, Item, Category};

struct Context {
    feeder: feeder::FeederService,
}

struct Model {
    channels: Vec<Channel>,
    items: Vec<Item>,
    current_item: Option<usize>,
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

    fn create(_: &mut Env<Context, Self>) -> Self {
        Model {
            channels: Vec::new(),
            items: Vec::new(),
            current_item: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::FetchChannels => {
                let callback = context.send_back(Msg::ChannelsReady);
                context.feeder.channels(callback);
            },
            Msg::ChannelsReady(Ok(channels)) => {
                self.channels = channels;
            },
            Msg::FetchItems(id) => {
                let callback = context.send_back(Msg::ItemsReady);
                context.feeder.items(id, callback);
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
                <li class="pure-menu-item",>
                    <a href="#", class="pure-menu-link", onclick=move|_| Msg::FetchItems(id),>{ &chan.title }</a>
                </li>
            }
        };
        let view_item_details = |item: &Item| {
            let link = item.link.clone().unwrap_or("".to_owned());
            let description = item.description.clone().unwrap_or("".to_owned());
            let author = item.author.clone().unwrap_or("".to_owned());
            let guid = item.guid.clone().unwrap_or("".to_owned());
            let pub_date = item.pub_date.clone().unwrap_or("".to_owned());
            html! {
                <div>
                    <h6><a href=link,>{"link"}</a></h6>
                    <article><iframe sandbox="", srcdoc=description,></iframe></article>
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
                    <h4 onclick=move|_| Msg::Details(idx),>{title}</h4>
                    <section>
                    {
                        if self.current_item == Some(idx) {
                            view_item_details(&self.items[idx])
                        } else {
                            html!{<div></div>}
                        }
                    }
                    </section>
                </div>
            }
        };
        html! {
            <div id="site",>
                <header class="site-header",> {"My Header"} </header>
                <nav class="site-nav",>
                    <button onclick=move|_| Msg::FetchChannels,>
                        { "fetch channels!" }
                    </button>
                </nav>
                <aside class="site-aside",>
                    { for self.channels.iter().map(view_channel) }
                </aside>
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

    let app: App<_, Model> = App::new(context);
    app.mount_to_body();

    yew::run_loop();
}
