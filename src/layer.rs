use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub id: String,
    pub r#type: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<Paint>,
}

impl Layer {
    pub fn new(
        id: impl Into<String>,
        r#type: impl Into<String>,
        source: impl Into<String>,
    ) -> Layer {
        Layer {
            id: id.into(),
            r#type: r#type.into(),
            source: source.into(),
            layout: None,
            paint: None,
        }
    }
}

/// Layout property can be either value in String or Number e.g. 0.25
/// or tuple of 2 elements, ("get", "icon").
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LayoutProperty {
    String(String),
    Number(f64),
    FeatureProperty((String, String)),
}

impl From<&str> for LayoutProperty {
    fn from(value: &str) -> Self {
        LayoutProperty::String(value.into())
    }
}

impl From<String> for LayoutProperty {
    fn from(value: String) -> Self {
        LayoutProperty::String(value)
    }
}

impl From<f64> for LayoutProperty {
    fn from(value: f64) -> Self {
        LayoutProperty::Number(value)
    }
}

impl From<(String, String)> for LayoutProperty {
    fn from((verb, name): (String, String)) -> Self {
        LayoutProperty::FeatureProperty((verb, name))
    }
}

impl From<(&str, &str)> for LayoutProperty {
    fn from((verb, name): (&str, &str)) -> Self {
        LayoutProperty::FeatureProperty((verb.into(), name.into()))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Layout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_join: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_cap: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_image: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_size: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_anchor: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_offset: Option<(i32, i32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<LayoutProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_allow_overlap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_allow_overlap: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Paint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_halo_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_halo_color: Option<String>,
}
