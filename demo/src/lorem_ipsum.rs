use pwt::prelude::*;
use pwt::widget::Column;

#[function_component]
pub fn LoremIpsum() -> Html {
    Column::new()
        .padding(2)
        .gap(2)
        .with_child(html!{<h3>{"Lorem Ipsum"}</h3>})
        .with_child(html!{<p>{r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Semper purus aliquet consequat amet fames sollicitudin. Praesent vulputate class malesuada euismod congue scelerisque fermentum viverra sagittis ex elementum consequat ipsum.
"#
        }</p>})
        .with_child(html!{<p>{r#"
Pulvinar tristique sit ex feugiat finibus pharetra lobortis venenatis commodo mi class rutrum tempus. Fames commodo lacus imperdiet feugiat tincidunt dui aliquet lorem ligula egestas sapien lectus cubilia conubia. Convallis nisi blandit sem cubilia nisl pellentesque placerat duis massa nibh commodo facilisis inceptos dapibus.
"#
        }</p>})
        .with_child(html!{<p>{r#"
Massa leo praesent ad pharetra amet et inceptos aptent placerat sit vulputate venenatis orci pulvinar feugiat. Elit fringilla aptent in erat leo mattis netus purus lacus. Porta imperdiet taciti laoreet ipsum vel tempor urna ipsum tellus ultricies pulvinar. At placerat lacinia elementum libero suscipit et suscipit imperdiet elementum. Nulla eros habitasse phasellus duis elementum vivamus auctor nisi cubilia tempor tincidunt sit placerat.
"#
        }</p>})
        .with_child(html!{<p>{r#"
Dictumst ornare consequat finibus ullamcorper lacinia interdum mattis arcu. Potenti venenatis curabitur orci netus non sociosqu quam torquent aptent tristique nulla tempus urna.
    "#
        }</p>})
        .into()
}
