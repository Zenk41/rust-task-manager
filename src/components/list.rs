use leptos::*;
use stylers::style;

#[component]
pub fn List(
    #[prop(default = "Menyapu rumah sampai halaman".to_string())] label: String,
    #[prop(default = false)] _is_done: bool,
) -> impl IntoView {
    let _styler_class = style! {
        "List",
        h6 {
            color: purple;
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;

        }
        div {
            display:flex;
            flex-direction:row;
            width:40%;
        }
    };
    view! { class=_styler_class,
        <div class="list">
            <h6>{label}</h6>
            <input type="checkbox" checked=_is_done/>
        </div>
    }
}
