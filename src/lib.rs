use std::collections::HashMap;

use exports::edgee::components::consent_management::{Consent, Dict, Guest};

wit_bindgen::generate!({
    path: ".edgee/wit",
    world: "consent-management",
    generate_all,
});

struct Component;
export!(Component);

impl Guest for Component {
    fn map(cookies: Dict, settings: Dict) -> Option<Consent> {
        let settings = match Settings::try_from(settings) {
            Ok(settings) => settings,
            Err(err) => {
                eprintln!("Could not get settings: {err}");
                return Some(Consent::Pending);
            }
        };
        let cookies = match Cookies::from_dict(cookies, &settings.cookie_name) {
            Ok(cookies) => cookies,
            Err(err) => {
                eprintln!("Could not get cookies: {err}");
                return Some(Consent::Pending);
            }
        };

        if matches!(
            cookies,
            Cookies {
                vendors: None,
                purposes: None,
                vendors_li: None,
                purposes_li: None,
            }
        ) {
            return Some(Consent::Pending);
        }

        let entries = [
            cookies.vendors,
            cookies.purposes,
            cookies.vendors_li,
            cookies.purposes_li,
        ];
        let disabled = entries
            .into_iter()
            .flatten()
            .flat_map(|entry| entry.disabled)
            .collect::<Vec<_>>();
        if !disabled.is_empty() {
            return Some(Consent::Denied);
        }

        Some(Consent::Granted)
    }
}

struct Settings {
    cookie_name: String,
}

impl TryFrom<Dict> for Settings {
    type Error = String;

    fn try_from(value: Dict) -> Result<Self, Self::Error> {
        let dict: HashMap<_, _> = value.into_iter().collect();

        Ok(Self {
            cookie_name: dict
                .get("cookie_name")
                .map(|v| v.as_str())
                .unwrap_or("didomi_token")
                .to_string(),
        })
    }
}

#[derive(serde::Deserialize)]
struct Cookies {
    #[serde(default)]
    vendors: Option<CookieEntry>,
    #[serde(default)]
    purposes: Option<CookieEntry>,
    #[serde(default)]
    vendors_li: Option<CookieEntry>,
    #[serde(default)]
    purposes_li: Option<CookieEntry>,
}

#[derive(serde::Deserialize)]
struct CookieEntry {
    #[serde(default)]
    disabled: Vec<serde_json::Value>,
}

impl Cookies {
    fn from_dict(cookies: Dict, cookie_name: &str) -> Result<Self, String> {
        use base64::prelude::*;

        let cookies: HashMap<_, _> = cookies.into_iter().collect();

        let value = cookies
            .get(cookie_name)
            .ok_or_else(|| format!("Cookie not found: {cookie_name}"))?;
        let value =
            urlencoding::decode(value).map_err(|err| format!("Invalid cookie value: {err}"))?;
        let value = BASE64_STANDARD
            .decode(value.as_ref())
            .map_err(|err| format!("Invalid cookie value: {err}"))?;

        serde_json::from_slice(&value).map_err(|err| format!("Invalid cookie value: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    macro_rules! dict {
        {
            $($key:literal: $value:expr),*$(,)?
        } => {
            vec![
                $(($key.to_string(), $value.to_string()),)*
            ]
        };
    }

    fn make_cookie(value: serde_json::Value) -> String {
        use base64::prelude::*;

        let value = serde_json::to_vec(&value).unwrap();
        let value = BASE64_STANDARD.encode(value);
        let value = urlencoding::encode(&value);

        value.to_string()
    }

    #[test]
    fn test_consent_pending() {
        let cookies = dict! {};

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Pending)
        );
    }

    #[test]
    fn test_consent_granted() {
        let cookies = dict! {
            "didomi_token": make_cookie(serde_json::json!({
                "purposes": {
                    "disabled": [],
                },
                "purposes_li": {
                    "disabled": [],
                },
                "vendors": {
                    "disabled": [],
                },
                "vendors_li": {
                    "disabled": [],
                },
            })),
        };

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Granted)
        );
    }

    #[test]
    fn test_incomplete_cookie() {
        let cookies = dict! {
            "didomi_token": make_cookie(serde_json::json!({
                "purposes": {
                    "disabled": [],
                },
            })),
        };

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Granted)
        );
    }

    #[test]
    fn test_consent_denied() {
        let cookies = dict! {
            "didomi_token": make_cookie(serde_json::json!({
                "purposes": {
                    "disabled": ["test"],
                },
            })),
        };

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Denied)
        );
    }

    #[test]
    fn test_custom_cookie_name() {
        let settings = dict! {
            "cookie_name": "edgee_cookie",
        };
        let cookies = dict! {
            "edgee_cookie": make_cookie(serde_json::json!({
                "purposes": {
                    "disabled": [],
                },
            })),
        };

        assert_eq!(Component::map(cookies, settings), Some(Consent::Granted));
    }
}
