//! Bounded inert reading literals, usable without GTK.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RichRun {
    pub text: String,
    pub style: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChartPoint {
    pub id: String,
    pub label: String,
    pub value: f64,
}
pub fn valid_runs(runs: &[RichRun]) -> bool {
    !runs.is_empty()
        && runs.len() <= 256
        && runs.iter().map(|run| run.text.len()).sum::<usize>() <= 65536
        && runs.iter().all(|run| {
            !run.text.contains('\0')
                && ["plain", "strong", "emphasis", "code"].contains(&run.style.as_str())
        })
}
pub fn valid_points(points: &[ChartPoint]) -> bool {
    let mut ids = std::collections::BTreeSet::new();
    !points.is_empty()
        && points.len() <= 128
        && points.iter().all(|point| {
            !point.id.is_empty()
                && point.id.len() <= 128
                && point
                    .id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-_:".contains(&b))
                && ids.insert(&point.id)
                && !point.label.trim().is_empty()
                && point.label.len() <= 256
                && !point.label.contains('\0')
                && point.value.is_finite()
                && point.value.abs() <= 1e12
        })
}
pub fn valid_literal(kind: &str, value: &serde_json::Value) -> bool {
    match kind {
        "richruns" => serde_json::from_value::<Vec<RichRun>>(value.clone())
            .is_ok_and(|runs| valid_runs(&runs)),
        "chartpoints" => serde_json::from_value::<Vec<ChartPoint>>(value.clone())
            .is_ok_and(|points| valid_points(&points)),
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn reading_literals_bound_keys_values_styles_and_unknown_fields() {
        assert!(valid_literal(
            "richruns",
            &json!([{"text":"<script>日本語","style":"strong"}])
        ));
        assert!(!valid_literal(
            "richruns",
            &json!([{"text":"x","style":"markup"}])
        ));
        assert!(!valid_literal(
            "richruns",
            &json!([{"text":"x","style":"plain","url":"file:///secret"}])
        ));
        assert!(!valid_literal(
            "richruns",
            &json!([{"text":"x".repeat(65537),"style":"plain"}])
        ));
        assert!(valid_literal(
            "chartpoints",
            &json!([{"id":"a","label":"Measured","value":-12.5}])
        ));
        for value in [
            json!([]),
            json!([{"id":"a","label":"A","value":1},{"id":"a","label":"B","value":2}]),
            json!([{"id":"a","label":" ","value":1}]),
            json!([{"id":"a","label":"A","value":1e13}]),
        ] {
            assert!(!valid_literal("chartpoints", &value));
        }
    }
}
