use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub struct FormattedMessageDetail {
    raw: String,
    components: Vec<FormattedMessageComponent>,
}

impl FormattedMessageDetail {
    pub fn new(raw: String, components: Vec<FormattedMessageComponent>) -> Self {
        Self {
            raw,
            components,
        }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn components(&self) -> &Vec<FormattedMessageComponent> {
        &self.components
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormattedMessageComponent {
    Section(String, Vec<FormattedString>),
    Text(Vec<FormattedString>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormattedString {
    styles: Vec<Style>,
    s: String,
}

impl FormattedString {
    pub fn new(s: String, styles: Vec<Style>) -> Self {
        Self {
            s,
            styles
        }
    }

    pub fn plain(s: String) -> Self {
        Self {
            s,
            styles: vec![],
        }
    }

    pub fn get_styles(&self) -> &Vec<Style> {
        &self.styles
    }

    pub fn get_string(&self) -> &str {
        &self.s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Style {
    Bold,
    Italics,
    Monospace,
}

pub fn parse_raw_to_formatted(s: &str) -> FormattedMessageDetail {
    let mut components = vec![];

    let mut section_title = None;
    let mut section_text = vec![];

    fn push_section(section_title: &mut Option<String>, section_text: &mut Vec<FormattedString>, components: &mut Vec<FormattedMessageComponent>) {
        if section_title.is_some() || !section_text.is_empty() {
            let old = mem::replace(section_text, vec![]);
            let component = if let Some(title) = section_title.take() {
                FormattedMessageComponent::Section(title, old)
            } else {
                FormattedMessageComponent::Text(old)
            };
            components.push(component);
        }
    }

    for line in s.lines() {
        // #<section text>#
        if line.len() > 4 && line.starts_with("#<") && line.ends_with(">#") {
            push_section(&mut section_title, &mut section_text, &mut components);
            section_title = Some(line[2..line.len() - 2].to_owned());
        } else {
            section_text.extend_from_slice(&parse_section_text(&format!("{}\n", line)));
        }
    }
    push_section(&mut section_title, &mut section_text, &mut components);
    FormattedMessageDetail::new(s.to_owned(), components)
}

fn parse_section_text(s: &str) -> Vec<FormattedString> {
    vec![FormattedString::new(s.to_owned(), vec![])]
}