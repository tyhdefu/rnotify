use crate::message::formatted_detail::{FormattedMessageComponent, FormattedMessageDetail, FormattedString, Style};
use crate::message::MessageDetail;

pub trait FormattedStringAppendable {
    fn append(&mut self, s: FormattedString) -> &mut Self;

    fn append_styled<S: ToString>(&mut self, s: S, style: Style) -> &mut Self {
        self.append(FormattedString::styled(s, style));
        self
    }

    fn append_plain<S: ToString>(&mut self, s: S) -> &mut Self {
        self.append(FormattedString::plain(s));
        self
    }
}

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

    pub fn raw(&mut self, raw: String) -> &mut Self {
        self.raw = raw;
        self
    }

    pub fn section<F, S>(&mut self, name: S, apply: F) -> &mut Self
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

    pub fn text_block<F>(&mut self, apply: F) -> &mut Self
        where F: FnOnce(&mut TextBlockBuilder) {
        let mut text_block = TextBlockBuilder::default();
        apply(&mut text_block);
        self.contents.push(text_block.build());
        self
    }

    pub fn build(self) -> MessageDetail {
        MessageDetail::Formatted(FormattedMessageDetail::new(self.raw, self.contents))
    }
}

#[derive(Default)]
pub struct TextBlockBuilder {
    contents: Vec<FormattedString>,
}

impl TextBlockBuilder {
    pub fn build(self) -> FormattedMessageComponent {
        FormattedMessageComponent::Text(self.contents)
    }

    pub fn build_vec(self) -> Vec<FormattedString> {
        self.contents
    }
}

impl FormattedStringAppendable for TextBlockBuilder {
    fn append(&mut self, s: FormattedString) -> &mut Self {
        self.contents.push(s);
        self
    }
}

pub struct SectionBuilder {
    name: String,
    contents: Vec<FormattedString>,
}

impl SectionBuilder {
    pub fn build(self) -> FormattedMessageComponent {
        FormattedMessageComponent::Section(self.name, self.contents)
    }
}

impl FormattedStringAppendable for SectionBuilder {
    fn append(&mut self, s: FormattedString) -> &mut Self {
        self.contents.push(s);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::message::formatted_detail::{FormattedMessageComponent, FormattedMessageDetail, FormattedString, Style};
    use crate::message::detail_builder::{FormattedStringAppendable, MessageDetailBuilder};
    use crate::message::MessageDetail;

    #[test]
    fn test_detail_builder() {
        let mut builder = MessageDetailBuilder::new();
        builder.text_block(|block| {
                block.append_plain("Base Description");
            })
            .section("New section", |section| {
                section.append_styled("hello", Style::Monospace);
            });

        let built = builder.build();

        let test = MessageDetail::Formatted(FormattedMessageDetail::new(
            "Raw not available".to_string(),
            vec![
                FormattedMessageComponent::Text(vec![FormattedString::plain("Base Description".to_owned())]),
                FormattedMessageComponent::Section("New section".to_owned(),
                                                    vec![FormattedString::styled("hello", Style::Monospace)])]
        ));

        assert_eq!(built, test);
    }
}