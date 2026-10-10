//! Versioned native component schemas; no host authority or application state.
use serde::Serialize;
use std::collections::BTreeMap;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyKind {
    String,
    Boolean,
    Number,
}
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum PropertyType {
    Kind(PropertyKind),
    Choices(Vec<&'static str>),
}
#[derive(Clone, Debug, Serialize)]
pub struct Component {
    pub props: BTreeMap<&'static str, PropertyType>,
    pub required: Vec<&'static str>,
    pub bindable: Vec<&'static str>,
    pub events: Vec<&'static str>,
    pub slots: BTreeMap<&'static str, [usize; 2]>,
    pub role: &'static str,
    pub minimum_size: [u32; 2],
}
fn component(
    role: &'static str,
    props: Vec<(&'static str, PropertyType)>,
    required: Vec<&'static str>,
) -> Component {
    Component {
        props: props.into_iter().collect(),
        required,
        bindable: vec![],
        events: vec![],
        slots: BTreeMap::new(),
        role,
        minimum_size: [80, 32],
    }
}
pub fn components() -> BTreeMap<&'static str, Component> {
    use PropertyKind::*;
    use PropertyType::*;
    let mut stack = component(
        "group",
        vec![("spacing", Choices(vec!["compact", "normal", "relaxed"]))],
        vec![],
    );
    stack.slots.insert("children", [0, 2048]);
    let text = component(
        "text",
        vec![
            ("text", Kind(String)),
            ("role", Choices(vec!["text", "heading"])),
        ],
        vec!["text"],
    );
    let mut status = component("status", vec![("value", Kind(String))], vec!["value"]);
    status.bindable = vec!["value"];
    let mut button = component(
        "button",
        vec![("label", Kind(String)), ("disabled", Kind(Boolean))],
        vec!["label"],
    );
    button.events = vec!["activate"];
    let mut link = component(
        "link",
        vec![("label", Kind(String)), ("url", Kind(String))],
        vec!["label", "url"],
    );
    link.events = vec!["activate"];
    let mut field = component(
        "textbox",
        vec![
            ("label", Kind(String)),
            ("placeholder", Kind(String)),
            ("value", Kind(String)),
            ("multiline", Kind(Boolean)),
        ],
        vec!["label"],
    );
    field.events = vec!["submit", "change"];
    field.minimum_size = [160, 40];
    let mut progress = component(
        "progressbar",
        vec![("label", Kind(String)), ("value", Kind(Number))],
        vec!["label", "value"],
    );
    progress.bindable = vec!["value"];
    BTreeMap::from([
        ("Stack@1", stack.clone()),
        ("Row@1", stack),
        ("Text@1", text),
        ("Status@1", status),
        ("Button@1", button),
        ("Link@1", link),
        ("TextField@1", field),
        ("Progress@1", progress),
    ])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metadata_is_available_without_a_renderer() {
        let catalog = components();
        assert_eq!(catalog.len(), 8);
        for schema in catalog.values() {
            for property in schema.required.iter().chain(&schema.bindable) {
                assert!(schema.props.contains_key(property));
            }
            assert!(schema.minimum_size[0] > 0 && schema.minimum_size[1] > 0);
        }
    }
}
