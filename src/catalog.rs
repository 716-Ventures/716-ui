//! Versioned native component schemas; no host authority or application state.
use serde::Serialize;
use std::collections::BTreeMap;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyKind {
    String,
    Boolean,
    Number,
    StringList,
    KeyedRows,
    RichRuns,
    ChartPoints,
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
    let mut table = component(
        "table",
        vec![
            ("label", Kind(String)),
            ("columns", Kind(StringList)),
            ("rows", Kind(KeyedRows)),
        ],
        vec!["label", "columns", "rows"],
    );
    table.minimum_size = [240, 200];
    let mut list = component(
        "list",
        vec![("label", Kind(String)), ("rows", Kind(KeyedRows))],
        vec!["label", "rows"],
    );
    list.minimum_size = [160, 200];
    let mut section = component(
        "group",
        vec![
            ("label", Kind(String)),
            ("spacing", Choices(vec!["compact", "normal", "relaxed"])),
        ],
        vec!["label"],
    );
    section.slots.insert("children", [0, 2048]);
    let mut scroll = section.clone();
    scroll.minimum_size = [160, 160];
    let mut split = component(
        "group",
        vec![
            ("label", Kind(String)),
            ("axis", Choices(vec!["horizontal", "vertical"])),
        ],
        vec!["label"],
    );
    split.slots.insert("children", [2, 2]);
    split.minimum_size = [240, 160];
    let mut tabs = component(
        "tablist",
        vec![("label", Kind(String)), ("labels", Kind(StringList))],
        vec!["label", "labels"],
    );
    tabs.slots.insert("children", [1, 32]);
    tabs.minimum_size = [160, 120];
    let choice = component(
        "combobox",
        vec![
            ("label", Kind(String)),
            ("options", Kind(StringList)),
            ("value", Kind(String)),
        ],
        vec!["label", "options", "value"],
    );
    let toggle = component(
        "checkbox",
        vec![("label", Kind(String)), ("value", Kind(Boolean))],
        vec!["label", "value"],
    );
    let rich = component("text", vec![("runs", Kind(RichRuns))], vec!["runs"]);
    let mut chart = component(
        "figure",
        vec![("label", Kind(String)), ("points", Kind(ChartPoints))],
        vec!["label", "points"],
    );
    chart.minimum_size = [240, 240];
    let mut image = component(
        "img",
        vec![("label", Kind(String)), ("reference", Kind(String))],
        vec!["label", "reference"],
    );
    image.minimum_size = [160, 240];
    let terminal = component(
        "terminal",
        vec![("label", Kind(String)), ("source", Kind(String))],
        vec!["label", "source"],
    );
    let mut outcome = component(
        "status",
        vec![("label", Kind(String)), ("value", Kind(String))],
        vec!["label", "value"],
    );
    outcome.bindable = vec!["value"];
    let mut failure = component(
        "alert",
        vec![
            ("label", Kind(String)),
            ("message", Kind(String)),
            ("recovery_label", Kind(String)),
        ],
        vec!["label", "message"],
    );
    failure.bindable = vec!["message"];
    failure.events = vec!["recover"];
    let reference = component(
        "button",
        vec![("label", Kind(String)), ("target", Kind(String))],
        vec!["label", "target"],
    );
    BTreeMap::from([
        ("PtySession@1", terminal),
        ("Result@1", outcome),
        ("Error@1", failure),
        ("DocumentReference@1", reference.clone()),
        ("ApplicationReference@1", reference),
        ("Image@1", image),
        ("RichText@1", rich),
        ("Chart@1", chart),
        ("Choice@1", choice),
        ("Toggle@1", toggle),
        ("Split@1", split),
        ("Tabs@1", tabs),
        ("Section@1", section),
        ("Scroll@1", scroll),
        ("List@1", list.clone()),
        ("KeyValue@1", list),
        ("Table@1", table),
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
/// Validate collection literals before a renderer allocates model objects.
pub fn valid_collection(kind: &str, value: &serde_json::Value) -> bool {
    match kind {
        "stringlist" => value.as_array().is_some_and(|values| {
            !values.is_empty()
                && values.len() <= 32
                && values.iter().all(|value| {
                    value
                        .as_str()
                        .is_some_and(|s| !s.is_empty() && s.len() <= 256 && !s.contains('\0'))
                })
        }),
        "keyedrows" => {
            let mut keys = std::collections::BTreeSet::new();
            value.as_array().is_some_and(|rows| {
                rows.len() <= 10000
                    && rows.iter().all(|row| {
                        row.as_object().is_some_and(|object| {
                            object.len() == 2
                                && row["id"].as_str().is_some_and(|id| {
                                    !id.is_empty()
                                        && id.len() <= 128
                                        && id.bytes().all(|b| {
                                            b.is_ascii_alphanumeric() || b"-_:".contains(&b)
                                        })
                                        && keys.insert(id.to_string())
                                })
                                && row["cells"].as_array().is_some_and(|cells| {
                                    !cells.is_empty()
                                        && cells.len() <= 32
                                        && cells.iter().all(|cell| {
                                            cell.as_str().is_some_and(|s| {
                                                s.len() <= 4096 && !s.contains('\0')
                                            })
                                        })
                                })
                        })
                    })
            })
        }
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn table_literals_have_unique_keys_bounded_cells_and_no_extra_fields() {
        use serde_json::json;
        assert!(valid_collection("stringlist", &json!(["Name", "Status"])));
        assert!(valid_collection(
            "keyedrows",
            &json!([{"id":"row-a","cells":["日本語","Ready"]}])
        ));
        for value in [
            json!([{"id":"a","cells":["Ready"]},{"id":"a","cells":["Other"]}]),
            json!([{"id":"a","cells":[1]}]),
            json!([{"id":"a","cells":["Ready"],"code":"run"}]),
        ] {
            assert!(!valid_collection("keyedrows", &value));
        }
        assert!(!valid_collection("stringlist", &json!([])));
    }
    #[test]
    fn metadata_is_available_without_a_renderer() {
        let catalog = components();
        assert_eq!(catalog.len(), 25);
        for schema in catalog.values() {
            for property in schema.required.iter().chain(&schema.bindable) {
                assert!(schema.props.contains_key(property));
            }
            assert!(schema.minimum_size[0] > 0 && schema.minimum_size[1] > 0);
        }
    }
}
