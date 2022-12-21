mod modal_v1;
mod modal_v2;

use crate::modal_v1::ModalV1;
use crate::modal_v2::ModalV2;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <ModalV1/>
        <ModalV2/>
        <div id="main-content">{"メインコンテンツ"}</div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
