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
/// Zero-inclusive domain; only all-zero data needs a synthetic unit span.
#[cfg(any(feature = "native", test))]
pub(crate) fn chart_domain(points: &[ChartPoint]) -> Option<(f64, f64)> {
    if !valid_points(points) { return None; }
    let min = points.iter().map(|p| p.value).fold(0.0, f64::min);
    let max = points.iter().map(|p| p.value).fold(0.0, f64::max);
    Some((min, if min == max { 1.0 } else { max }))
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
    fn chart_domain_preserves_fractional_and_tiny_values() {
        let points = |values: &[f64]| values.iter().enumerate().map(|(i, value)| ChartPoint {
            id: format!("point-{i}"), label: format!("Measured {i}"), value: *value,
        }).collect::<Vec<_>>();
        for (values, expected) in [
            ([0.1, 0.2], (0.0, 0.2)), ([-0.1, -0.2], (-0.2, 0.0)),
            ([-0.1, 0.2], (-0.1, 0.2)), ([0.0, 0.0], (0.0, 1.0)),
            ([1e-12, 2e-12], (0.0, 2e-12)), ([-1e12, 1e12], (-1e12, 1e12)),
            ([f64::from_bits(1), f64::from_bits(2)], (0.0, f64::from_bits(2))),
        ] { assert_eq!(chart_domain(&points(&values)), Some(expected)); }
        for values in [vec![], vec![f64::NAN], vec![f64::INFINITY], vec![1e13]] {
            assert_eq!(chart_domain(&points(&values)), None);
        }
    }
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

pub const IMAGE_LIMIT: usize = 256 * 1024;
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320 & (0u32.wrapping_sub(crc & 1)));
        }
    }
    !crc
}
/// Inspect a complete bounded PNG before a native decoder allocates pixels.
pub fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), String> {
    if bytes.len() < 57 || bytes.len() > IMAGE_LIMIT || bytes[..8] != *b"\x89PNG\r\n\x1a\n" {
        return Err("A bounded PNG image is required".into());
    }
    let mut offset = 8;
    let mut dimensions = None;
    let mut data = false;
    let mut chunks = 0;
    while offset + 12 <= bytes.len() {
        chunks += 1;
        if chunks > 4096 {
            break;
        }
        let len = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        let end = offset
            .checked_add(12)
            .and_then(|v| v.checked_add(len))
            .filter(|end| *end <= bytes.len())
            .ok_or("Truncated PNG chunk")?;
        let kind = &bytes[offset + 4..offset + 8];
        let body = &bytes[offset + 8..end - 4];
        if crc32(&bytes[offset + 4..end - 4])
            != u32::from_be_bytes(bytes[end - 4..end].try_into().unwrap())
        {
            return Err("Invalid PNG checksum".into());
        }
        if dimensions.is_none() && kind != b"IHDR" {
            return Err("PNG header must be first".into());
        }
        match kind {
            b"IHDR" => {
                if dimensions.is_some() || len != 13 {
                    return Err("Invalid PNG header".into());
                }
                let width = u32::from_be_bytes(body[..4].try_into().unwrap());
                let height = u32::from_be_bytes(body[4..8].try_into().unwrap());
                if width == 0
                    || height == 0
                    || width > 4096
                    || height > 4096
                    || u64::from(width) * u64::from(height) > 4 * 1024 * 1024
                    || ![0, 2, 3, 4, 6].contains(&body[9])
                    || ![1, 2, 4, 8, 16].contains(&body[8])
                    || body[10] != 0
                    || body[11] != 0
                    || body[12] > 1
                {
                    return Err("Unsupported PNG dimensions or format".into());
                }
                dimensions = Some((width, height));
            }
            b"IDAT" => data = true,
            b"IEND" if len == 0 && end == bytes.len() && data => return Ok(dimensions.unwrap()),
            b"PLTE" => {}
            _ if kind[0] & 0x20 != 0 => {}
            _ => return Err("Unsupported PNG chunk".into()),
        }
        offset = end;
    }
    Err("Incomplete PNG image".into())
}
pub fn image_hex(value: &str) -> Result<Vec<u8>, String> {
    if value.len() > 2 * IMAGE_LIMIT
        || value.len() % 2 != 0
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("Invalid bounded image encoding".into());
    }
    let bytes = (0..value.len())
        .step_by(2)
        .map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap())
        .collect::<Vec<_>>();
    png_dimensions(&bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod image_tests {
    use super::*;
    #[test]
    fn images_are_complete_crc_checked_and_bounded_before_decoding() {
        let bytes = include_bytes!("../tests/pixel.png");
        assert_eq!(png_dimensions(bytes).unwrap(), (1, 1));
        let mut corrupt = bytes.to_vec();
        corrupt[20] ^= 1;
        assert!(png_dimensions(&corrupt).is_err());
        assert!(png_dimensions(&bytes[..bytes.len() - 1]).is_err());
        assert!(image_hex("ffx").is_err());
        assert!(image_hex(&"00".repeat(IMAGE_LIMIT + 1)).is_err());
        let hex = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(image_hex(&hex).unwrap(), bytes);
    }
}

/// Literal replacement preflights UTF-8 byte growth before allocating the result.
pub fn replace_literal(
    text: &str,
    query: &str,
    replacement: &str,
) -> Result<(String, usize), String> {
    if text.len() > 65536
        || query.is_empty()
        || query.len() > 256
        || replacement.len() > 256
        || [text, query, replacement]
            .iter()
            .any(|value| value.contains('\0'))
    {
        return Err(
            "Use a document up to 64 KiB and literal search/replacement up to 256 bytes".into(),
        );
    }
    let count = text.match_indices(query).count();
    let length = text.len() - count * query.len() + count * replacement.len();
    if length > 65536 {
        return Err("Replacement would exceed 64 KiB".into());
    }
    Ok((text.replace(query, replacement), count))
}
#[cfg(test)]
mod editor_tests {
    use super::*;
    #[test]
    fn replacement_is_literal_unicode_exact_and_bounded_before_allocation() {
        assert_eq!(
            replace_literal("日本語 λ 日本語", "日本語", "café").unwrap(),
            ("café λ café".into(), 2)
        );
        assert_eq!(
            replace_literal("a.*a", ".*", "$1").unwrap(),
            ("a$1a".into(), 1)
        );
        assert!(replace_literal(&"x".repeat(65536), "x", &"y".repeat(256)).is_err());
        assert!(replace_literal("text", "", "x").is_err());
    }
}
