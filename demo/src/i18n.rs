use pwt::prelude::*;
use pwt::widget::Panel;

use pwt::replace_param1;

#[function_component]
pub fn I18NDemo() -> Html {

    log::info!("{}", tr!("This is a test!"));

    Panel::new()
        .class("pwt-flex-fit")
        .border(false)
        .title(tr!("Internationalization with gettext"))
        .with_child(html!{
            <div>{replace_param1(tr!("You have {0} new messages"), "5")}</div>
        })
        .into()
}
