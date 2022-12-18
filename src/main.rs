use gloo_events::{EventListener, EventListenerOptions};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // モーダルの表示
    let is_show_modal = use_state(|| false);
    let show_modal = {
        let is_show_modal = is_show_modal.clone();
        Callback::from(move |_e| {
            is_show_modal.set(true);
        })
    };
    let close_modal = {
        let is_show_modal = is_show_modal.clone();
        Callback::from(move |_e| {
            is_show_modal.set(false);
        })
    };

    // スクロールの禁止
    let _lock_scroll = use_memo(
        |is_show_modal| {
            if **is_show_modal {
                let document = gloo_utils::document();
                let options = EventListenerOptions {
                    passive: false,
                    ..Default::default()
                };
                Some(vec![
                    EventListener::new_with_options(&document, "wheel", options, move |e| {
                        e.prevent_default();
                    }),
                    EventListener::new_with_options(&document, "touchmove", options, move |e| {
                        e.prevent_default();
                    }),
                ])
            } else {
                None
            }
        },
        is_show_modal.clone(),
    );

    html! {
        <>
        <button onclick={show_modal}>{"モーダルを表示"}</button>
        <div id="main-content">{"メインコンテンツ"}</div>
        if *is_show_modal {
            <div id="modal-overlay" onclick={close_modal.clone()}>
                <div id="modal-content" onclick={|e:MouseEvent|{e.stop_propagation();}}>
                    <p>{"これがモーダルウィンドウです。"}</p>
                    <p><button onclick={close_modal}>{"キャンセル"}</button></p>
                </div>
            </div>
        }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
