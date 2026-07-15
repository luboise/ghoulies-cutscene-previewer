use bnl::asset::loctext::LoctextEvent;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

const DEFAULT_TEXT: &str = "{piratecaptain}{sfx GDIALOGUE_PIRATE_CHEER}{centre}Ye stayed in one piece long enough{linek}ta wallop {finalscore} lubbers, but that ain't{linek}enough ta win ye any booty! Harrrr!{end}";

#[component]
fn App() -> Html {
    let cutscene_data = use_state(|| DEFAULT_TEXT.to_owned());
    let cutscene_data_value = (*cutscene_data).clone();

    let on_text_change = {
        let cutscene_data = cutscene_data.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input = target.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                cutscene_data.set(input.value());
            }
        })
    };

    let text_parsed = use_memo(cutscene_data, |data| {
        bnl::asset::loctext::LoctextValue::try_from((*data).clone().as_str())
    });

    let custom_image_url = use_state(|| "".to_owned());

    let on_image_url_change = {
        let custom_image_url = custom_image_url.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input = target.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                custom_image_url.set(input.value());
            }
        })
    };

    let custom_image_url_value = Some((*custom_image_url).clone()).filter(|v| !v.is_empty());

    html! {
        <div>
            <main>
                <div id="preview" display="flex" position="relative">
                    { if let Ok(v) = text_parsed.as_ref() {
                            html!{
                                <LoctextComponent value={v.clone()} custom_image_url={custom_image_url_value.clone()}/>
                            }
                        } else {html!{}} }
                </div>
                <div>
                    <textarea oninput={on_text_change} r#type="text" value={cutscene_data_value} />
                </div>
                <div>
                    <label>{ "custom image url" }</label>
                    <textarea
                        oninput={on_image_url_change}
                        r#type="text"
                        value={custom_image_url_value}
                    />
                </div>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    value: bnl::asset::loctext::LoctextValue,
    custom_image_url: Option<String>,
}

#[component]
fn LoctextComponent(props: &Props) -> Html {
    const DIALOG_HEADS: [&str; 18] = [
        "boy",
        "girl",
        "butler",
        "cook",
        "groundskeeper",
        "housekeeper",
        "krackpot",
        "baroness",
        "baron",
        "crone",
        "piratecaptain",
        "skeletonbad",
        "mrribs",
        "pirate",
        "littleguy",
        "zombiestandard",
        "hunchback",
        "binklogo",
    ];

    let aid = format!(
        "aid_texture_ghoulies_scoreboard_dialogheads_{}",
        props.value.character
    );

    let hash = bnl::asset::hash_aid(aid);

    let img_src = props
        .custom_image_url
        .clone()
        .unwrap_or(format!("assets/images/Texture_{hash:08x}.png"));

    let mut text_rows = vec![];
    let mut current_text_row = String::new();

    let canvas_ref = use_node_ref();

    for event in &props.value.events {
        match event {
            LoctextEvent::PlaySfx(_) => (),
            // TODO: text alignment
            LoctextEvent::TextAlignment(_text_alignment) => (),
            LoctextEvent::Text(t) => current_text_row += t,
            LoctextEvent::Newline => text_rows.push(std::mem::take(&mut current_text_row)),
            // TODO: Variable + injection
            LoctextEvent::Variable(_) => current_text_row += " 10 ",
            LoctextEvent::End => (),
        }
    }

    if !current_text_row.is_empty() {
        text_rows.push(current_text_row);
    }

    html! {
        <>
            <img position="absolute" src={img_src} />
            <div id="text">
                <div id="chars">
                    { text_rows.into_iter().map(|row|{
                                   html!{
                                       <div class="text-row">
                                       {
                                            row.chars().into_iter().map(|c| {
                                                let filename = if ['.', '/'].contains(&c) {
                                                    format!("assets/font/{:02x}.png", u8::try_from(c).unwrap_or(b'a'))
                                                }
                                                else {
                                                    format!("assets/font/{c}_0.png")
                                                };
                                                html!{
                                                   <div class="char-div">
                                                       <img class="text-char" src={filename.clone()}/>
                                                       <img class="text-shadow" src={filename}/>
                                                   </div>
                                                }
                                            }).collect::<Html>()
                                       }
                                       </div>
                                   }
                               }).collect::<Html>() }
                </div>
            </div>
        </>
    }
}
