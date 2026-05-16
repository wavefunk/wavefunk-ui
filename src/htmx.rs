use serde::Serialize;
use serde_json::{Map, Value};

pub const HX_TRIGGER_HEADER: &str = "HX-Trigger";

#[derive(Clone, Debug, Serialize)]
pub struct MessageEvent<'a> {
    pub kind: &'a str,
    pub msg: &'a str,
}

#[derive(Clone, Debug)]
pub enum Trigger<'a> {
    Toast(MessageEvent<'a>),
    Echo(MessageEvent<'a>),
    Custom { name: &'a str, value: Value },
}

impl<'a> Trigger<'a> {
    pub fn toast(kind: &'a str, msg: &'a str) -> Self {
        Self::Toast(MessageEvent { kind, msg })
    }

    pub fn echo(kind: &'a str, msg: &'a str) -> Self {
        Self::Echo(MessageEvent { kind, msg })
    }

    pub fn custom(name: &'a str, value: Value) -> Self {
        Self::Custom { name, value }
    }
}

pub fn trigger_header(triggers: &[Trigger<'_>]) -> serde_json::Result<String> {
    let mut map = Map::new();

    for trigger in triggers {
        match trigger {
            Trigger::Toast(event) => {
                map.insert("wfToast".to_owned(), serde_json::to_value(event)?);
            }
            Trigger::Echo(event) => {
                map.insert("wfEcho".to_owned(), serde_json::to_value(event)?);
            }
            Trigger::Custom { name, value } => {
                map.insert((*name).to_owned(), value.clone());
            }
        }
    }

    Ok(Value::Object(map).to_string())
}

pub fn trigger_header_pair(triggers: &[Trigger<'_>]) -> serde_json::Result<(&'static str, String)> {
    trigger_header(triggers).map(|value| (HX_TRIGGER_HEADER, value))
}

pub fn toast_header(kind: &str, msg: &str) -> String {
    trigger_header(&[Trigger::toast(kind, msg)])
        .expect("serializing a Wave Funk toast trigger should not fail")
}

pub fn echo_header(kind: &str, msg: &str) -> String {
    trigger_header(&[Trigger::echo(kind, msg)])
        .expect("serializing a Wave Funk echo trigger should not fail")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_hx_trigger_payload() {
        let header = trigger_header(&[Trigger::toast("ok", "Saved.")]).unwrap();
        assert_eq!(header, r#"{"wfToast":{"kind":"ok","msg":"Saved."}}"#);
    }

    #[test]
    fn builds_hx_trigger_response_header_pair() {
        let header = trigger_header_pair(&[
            Trigger::toast("ok", "Saved."),
            Trigger::echo("info", "Queued."),
        ])
        .unwrap();

        assert_eq!(header.0, HX_TRIGGER_HEADER);
        let value: Value = serde_json::from_str(&header.1).unwrap();
        assert_eq!(value["wfToast"]["kind"], "ok");
        assert_eq!(value["wfToast"]["msg"], "Saved.");
        assert_eq!(value["wfEcho"]["kind"], "info");
        assert_eq!(value["wfEcho"]["msg"], "Queued.");
    }
}
