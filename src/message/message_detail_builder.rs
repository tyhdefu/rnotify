use crate::message::formatted_detail::{FormattedMessageComponent, FormattedMessageDetail, FormattedString, Style};
use crate::message::MessageDetail;

pub struct MessageDetailBuilder {
    contents: Vec<FormattedMessageComponent>,
    raw: String,
}

impl MessageDetailBuilder {
    pub fn new() -> Self {
        Self::with_raw(String::from("Raw not available"))
    }

    pub fn with_raw(raw: String) -> Self {
        Self {
            contents: vec![],
            raw,
        }
    }

    pub fn section<F, S>(mut self, name: S, apply: F) -> Self
        where F: FnOnce(&mut SectionBuilder),
              S: ToString {
        let mut section = SectionBuilder {
            name: name.to_string(),
            contents: vec![],
        };
        apply(&mut section);
        self.contents.push(section.build());
        self
    }

    pub fn text(mut self, parts: Vec<FormattedString>) -> Self {
        self.contents.push(FormattedMessageComponent::Text(parts));
        self
    }

    pub fn build(self) -> MessageDetail {
        MessageDetail::Formatted(FormattedMessageDetail::new(self.raw, self.contents))
    }
}

pub struct SectionBuilder {
    name: String,
    contents: Vec<FormattedString>,
}

impl SectionBuilder {
    pub fn append(&mut self, s: FormattedString) -> &mut Self {
        self.contents.push(s);
        self
    }

    pub fn append_styled<S: ToString>(&mut self, s: S, style: Style) -> &mut Self {
        self.contents.push(FormattedString::styled(s, style));
        self
    }

    pub fn append_plain<S: ToString>(&mut self, s: S) -> &mut Self {
        self.contents.push(FormattedString::plain(s));
        self
    }

    pub fn build(self) -> FormattedMessageComponent {
        FormattedMessageComponent::Section(self.name, self.contents)
    }
}

#[cfg(test)]
mod test {
    use crate::message::formatted_detail::{FormattedMessageComponent, FormattedMessageDetail, FormattedString, Style};
    use crate::message::message_detail_builder::MessageDetailBuilder;
    use crate::message::MessageDetail;

    #[test]
    fn test_detail_builder() {
        let built = MessageDetailBuilder::new()
            .section("New section", |section| {
                section.append_styled("hello", Style::Monospace);
            })
            .build();

        let test = MessageDetail::Formatted(FormattedMessageDetail::new(
            "Raw not available".to_string(),
            vec![FormattedMessageComponent::Section("New section".to_owned(),
                                                    vec![FormattedString::styled("hello", Style::Monospace)])]
        ));

        assert_eq!(built, test);
    }
}