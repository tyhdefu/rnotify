use crate::message::formatted_detail::{FormattedMessageComponent, FormattedMessageDetail, FormattedString, Style};

pub trait HtmlMessageDetail {
    fn create_html(&self) -> String;
}

impl HtmlMessageDetail for FormattedMessageDetail {
    fn create_html(&self) -> String {
        formatted_to_html(&self)
    }
}

fn formatted_to_html(formatted: &FormattedMessageDetail) -> String {
    let mut html = String::with_capacity(100);
    for component in formatted.components() {
        match component {
            FormattedMessageComponent::Section(section, formatted_string) => {
                html.push_str(&format!("<div><h2>{}</h2><p>{}</p></div>",
                                       escape_html(section),
                                       parse_formatted(formatted_string)
                ));
            }
            FormattedMessageComponent::Text(formatted_string) => {
                html.push_str(&format!("<p>{}</p>", parse_formatted(formatted_string)))
            }
        }
    }
    html
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('>', "&gt;")
        .replace('<', "&lt;")
}

fn parse_formatted(formatted: &Vec<FormattedString>) -> String {
    let mut html = String::new();
    for part in formatted {
        for style in part.get_styles() {
            let start_tag = match style {
                Style::Bold => "<b>",
                Style::Italics => "<i>",
                Style::Monospace => "<code>",
                Style::Code { lang: _ } => "<code>",
            };
            html.push_str(start_tag);
        }
        html.push_str(&escape_html(part.get_string()));
        for style in part.get_styles() {
            let end_tag = match style {
                Style::Bold => "</b>",
                Style::Italics => "</i>",
                Style::Monospace => "</code>",
                Style::Code { lang: _ } => "</code>",
            };
            html.push_str(end_tag);
        }
    }
    html
}

#[cfg(test)]
mod test {
    use crate::message::detail_builder::{FormattedStringAppendable, MessageDetailBuilder};
    use crate::message::MessageDetail::Formatted;
    use super::*;

    #[test]
    fn test_html_conversion() {
        let mut builder = MessageDetailBuilder::new();
        builder.section("hello world", |body| {
            body.append_plain("Dear fellow inhabitants. it has come to my attention that ");
            body.append_styled("you have not been doing your job.", Style::Bold);
        });
        builder.text_block(|block| {
            block.append_plain("That is all.");
        });
        let message = builder.build();
        if let Formatted(formatted_detail) = message {
            let html = formatted_to_html(&formatted_detail);
            assert_eq!(html, "<div><h2>hello world</h2><p>Dear fellow inhabitants. it has come to my attention that <b>you have not been doing your job.</b></p></div><p>That is all.</p>")
        }
        else {
            panic!("oops");
        }
    }
}