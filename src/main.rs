use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{"The VALUE:"} { self.value }</p>
                <pre><code>
                {pretty_print(&L::List(vec![L::Atom("if"), L::Atom("q"), L::Atom("a"), L::Atom("e")]), 0)}
                </code></pre>
            </div>
        }
    }
}

fn pretty_print(obj: &L, indent:usize) -> Html {
    match obj {
        L::Atom(x) => html!{
            <span class="foo">
            {print_indented(x, indent)}
            </span>
        },
        L::List(xs) => html!{
            <div>
            {print_indented("(", indent)}
            {for xs.iter().map(|x| pretty_print(x, indent + 1))}
            {print_indented(")", indent)}
            </div>
        },
    }
}

fn print_indented(x: impl std::fmt::Display, indent: usize) -> Html {
    html!{{(" ".repeat(indent) + &x.to_string())}}
}

enum L {
    Atom(&'static str),
    List(Vec<L>)
}

fn main() {
    yew::start_app::<Model>();
}
