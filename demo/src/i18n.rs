use pwt::prelude::*;
use pwt::widget::Panel;

use pwt::replace_param1;

#[function_component]
pub fn I18NDemo() -> Html {
    Panel::new()
        .class("pwt-flex-fit")
        .border(false)
        .title(gettext("Internationalization with gettext"))
        .with_child(html!{
            <div>{replace_param1(gettext("You have {0} new messages"), "5")}</div>
        })
        .into()
}
