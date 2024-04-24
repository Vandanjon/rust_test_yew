use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let text = "Salut à tous";

    html! {
            <>
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>

            <p>{text}</p>
    </>
        }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
