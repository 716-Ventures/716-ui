//! Bounded button declarations shared by catalog validators and native hosts.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonVariant {
    #[default]
    #[serde(rename = "default")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "outline")]
    Outline,
    #[serde(rename = "ghost")]
    Ghost,
    #[serde(rename = "destructive")]
    Destructive,
    #[serde(rename = "link")]
    Link,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonSize {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "xs")]
    ExtraSmall,
    #[serde(rename = "sm")]
    Small,
    #[serde(rename = "lg")]
    Large,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "arrow-right")]
    ArrowRight,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconPosition {
    Leading,
    Trailing,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonIconPosition {
    Leading,
    Trailing,
    Only,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonContent {
    Text,
    IconOnly(Icon),
    WithIcon(Icon, IconPosition),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ButtonProps {
    pub label: String,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub variant: ButtonVariant,
    #[serde(default)]
    pub size: ButtonSize,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present_choice"
    )]
    pub icon: Option<Icon>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present_choice"
    )]
    pub icon_position: Option<ButtonIconPosition>,
}
// Missing choices use their default; an explicit null is not a catalog choice.
fn present_choice<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
impl ButtonProps {
    pub fn from_value(value: serde_json::Value) -> Result<Self, String> {
        let props: Self = serde_json::from_value(value).map_err(|error| error.to_string())?;
        props.validate().map_err(str::to_owned)?;
        Ok(props)
    }
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.label.trim().is_empty() || self.label.len() > 256 || self.label.contains('\0') {
            return Err("A nonempty button label of at most 256 UTF-8 bytes is required");
        }
        if self.icon_position.is_some() && self.icon.is_none() {
            return Err("An icon position requires an icon");
        }
        if self.icon.is_some() && self.label.chars().any(char::is_control) {
            return Err(
                "An icon button requires a single-line accessible label without control characters",
            );
        }
        Ok(())
    }
    pub fn content(&self) -> ButtonContent {
        match (
            self.icon,
            self.icon_position.unwrap_or(ButtonIconPosition::Leading),
        ) {
            (None, _) => ButtonContent::Text,
            (Some(icon), ButtonIconPosition::Only) => ButtonContent::IconOnly(icon),
            (Some(icon), ButtonIconPosition::Leading) => {
                ButtonContent::WithIcon(icon, IconPosition::Leading)
            }
            (Some(icon), ButtonIconPosition::Trailing) => {
                ButtonContent::WithIcon(icon, IconPosition::Trailing)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn default_declaration_retains_text_action_and_schema_choices_parse() {
        let default = ButtonProps::from_value(json!({"label":"Continue λ"})).unwrap();
        assert_eq!(default.variant, ButtonVariant::Primary);
        assert_eq!(default.size, ButtonSize::Default);
        assert_eq!(default.content(), ButtonContent::Text);
        assert!(!default.disabled);
        let serialized = serde_json::to_value(&default).unwrap();
        assert!(serialized.get("icon").is_none());
        assert!(serialized.get("icon_position").is_none());
        assert_eq!(ButtonProps::from_value(serialized).unwrap(), default);
        let catalog = crate::catalog::components();
        for field in ["variant", "size", "icon", "icon_position"] {
            let crate::catalog::PropertyType::Choices(choices) = &catalog["Button@1"].props[field]
            else {
                panic!("Button choice schema missing")
            };
            for choice in choices {
                let mut props = json!({"label":"Continue λ","icon":"plus"});
                props[field] = json!(choice);
                ButtonProps::from_value(props).unwrap();
            }
        }
        assert_eq!(
            ButtonProps::from_value(
                json!({"label":"Next","icon":"arrow-right","icon_position":"only"})
            )
            .unwrap()
            .content(),
            ButtonContent::IconOnly(Icon::ArrowRight)
        );
    }
    #[test]
    fn invalid_names_enum_values_and_dependent_properties_are_rejected() {
        for props in [
            json!({}),
            json!({"label":"Next","icon":null}),
            json!({"label":"Next","icon":"plus","icon_position":null}),
            json!({"label":" "}),
            json!({"label":"x".repeat(257)}),
            json!({"label":"x\0"}),
            json!({"label":"Next","variant":"#ff0000"}),
            json!({"label":"Next","size":"enormous"}),
            json!({"label":"Next","icon":"host-icon"}),
            json!({"label":"Next","icon_position":"only"}),
            json!({"label":"Next\nline","icon":"plus"}),
            json!({"label":"Next","command":"execute"}),
            json!({"label":"Next","disabled":"false"}),
        ] {
            assert!(ButtonProps::from_value(props.clone()).is_err(), "{props}");
        }
        assert!(ButtonProps::from_value(json!({"label":"λ".repeat(128)})).is_ok());
        assert!(ButtonProps::from_value(json!({"label":"λ".repeat(129)})).is_err());
        assert!(ButtonProps::from_value(json!({"label":"Two\nlines"})).is_ok());
    }
}
