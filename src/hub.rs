use reqwest::blocking::Client;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Default)]
pub struct HubInfo<'a> {
  pub ip:               Option<&'a str>,
  pub auth_usr:         Option<&'a str>,
  pub auth_pwd:         Option<&'a str>,
  pub api_id:           Option<&'a str>,
  pub api_access_token: Option<&'a str>,
  pub client:           Option<Client>,
}

fn de_strings<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
  Ok(match Value::deserialize(deserializer)? {
    Value::String(s) => s,
    Value::Number(num) => num.to_string(),
    Value::Bool(b) => b.to_string(),
    Value::Null => "".to_string(),
    _ => return Err(de::Error::custom("wrong type")),
  })
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInventory {
  pub location_name:      String,
  pub hub_name:           String,
  #[serde(deserialize_with = "de_strings")]
  pub is_component:       String,
  #[serde(deserialize_with = "de_strings")]
  pub zigbee_id:          String,
  pub device_type_name:   String,
  #[serde(deserialize_with = "de_strings")]
  pub location_id:        String,
  #[serde(deserialize_with = "de_strings")]
  pub id:                 String,
  #[serde(deserialize_with = "de_strings")]
  pub lan_id:             String,
  #[serde(deserialize_with = "de_strings")]
  pub hub_id:             String,
  #[serde(deserialize_with = "de_strings")]
  pub display_as_child:   String,
  #[serde(deserialize_with = "de_strings")]
  pub mesh_enabled:       String,
  pub name:               String,
  pub r#type:             String,
  #[serde(deserialize_with = "de_strings")]
  pub disabled:           String,
  pub status:             String,
  #[serde(deserialize_with = "de_strings")]
  pub linked_device:      String,
  #[serde(deserialize_with = "de_strings")]
  pub label:              String,
  #[serde(deserialize_with = "de_strings")]
  pub device_network_id:  String,
  #[serde(deserialize_with = "de_strings")]
  pub device_type_id:     String,
  #[serde(deserialize_with = "de_strings")]
  pub last_activity_time: String,
  #[serde(deserialize_with = "de_strings")]
  pub parent_device_id:   String,
  #[serde(deserialize_with = "de_strings")]
  pub display_name:       String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DeviceIDs {
  pub id: String,
  name:   String,
  label:  String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device {
  pub id:           String,
  pub name:         String,
  pub label:        String,
  pub r#type:       String,
  pub attributes:   Vec<DeviceAttribute>,
  pub capabilities: Vec<serde_json::Value>,
  pub commands:     Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAttribute {
  pub name:          String,
  #[serde(deserialize_with = "de_strings")]
  pub current_value: String,
  pub data_type:     String,
  #[serde(default)]
  pub values:        Vec<String>,
}

impl DeviceAttribute {
  pub fn get_numeric_value(&self) -> Option<String> {
    match self {
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "acceleration" =>
      {
        if current_value.to_ascii_lowercase() == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "alarm" =>
      {
        if current_value.to_ascii_lowercase() == "off" || current_value.to_ascii_lowercase() == "inactive" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "presence" =>
      {
        if current_value.to_ascii_lowercase() == "present" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "presence" =>
      {
        if current_value.to_ascii_lowercase() == "present" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "switch" =>
      {
        if current_value.to_ascii_lowercase() == "on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "button" =>
      {
        if current_value.to_ascii_lowercase() == "pushed" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "carbonMonoxide" =>
      {
        if current_value.to_ascii_lowercase() == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "status" =>
      {
        if current_value.to_ascii_lowercase() == "playing" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "consumableStatus" =>
      {
        if current_value.to_ascii_lowercase() == "good" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "contact" =>
      {
        if current_value.to_ascii_lowercase() == "closed" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "indicatorStatus" =>
      {
        if current_value.to_ascii_lowercase() == "when on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "lock" =>
      {
        if current_value.to_ascii_lowercase() == "locked" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "motion" =>
      {
        if current_value.to_ascii_lowercase() == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "shock" =>
      {
        if current_value.to_ascii_lowercase() == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "mute" =>
      {
        if current_value.to_ascii_lowercase() == "muted" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "sleeping" =>
      {
        if current_value.to_ascii_lowercase() == "sleeping" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "smoke" =>
      {
        if current_value.to_ascii_lowercase() == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "sound" =>
      {
        if current_value.to_ascii_lowercase() == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "tamper" =>
      {
        if current_value.to_ascii_lowercase() == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "door" =>
      {
        if current_value.to_ascii_lowercase() == "closed" {
          Some("0".to_string())
        } else if current_value.to_ascii_lowercase() == "closing" {
          Some("1".to_string())
        } else if current_value.to_ascii_lowercase() == "open" {
          Some("2".to_string())
        } else if current_value.to_ascii_lowercase() == "opening" {
          Some("3".to_string())
        } else if current_value.to_ascii_lowercase() == "unknown" {
          Some("4".to_string())
        } else {
          None
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "thermostatMode" =>
      {
        if current_value.to_ascii_lowercase() == "off" {
          Some("0".to_string())
        } else if current_value.to_ascii_lowercase() == "auto" {
          Some("1".to_string())
        } else if current_value.to_ascii_lowercase() == "heat" {
          Some("2".to_string())
        } else if current_value.to_ascii_lowercase() == "cool" {
          Some("3".to_string())
        } else if current_value.to_ascii_lowercase() == "emergency heat" {
          Some("4".to_string())
        } else {
          None
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "thermostatFanMode" =>
      {
        if current_value.to_ascii_lowercase() == "off" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "thermostatOperatingState" =>
      {
        if current_value.to_ascii_lowercase() == "idle" {
          Some("0".to_string())
        } else if current_value.to_ascii_lowercase() == "heating" {
          Some("1".to_string())
        } else if current_value.to_ascii_lowercase() == "cooling" {
          Some("2".to_string())
        } else if current_value.to_ascii_lowercase() == "pending heat" {
          Some("3".to_string())
        } else if current_value.to_ascii_lowercase() == "pending cool" {
          Some("4".to_string())
        } else if current_value.to_ascii_lowercase() == "vent economizer" {
          Some("5".to_string())
        } else if current_value.to_ascii_lowercase() == "fan only" {
          Some("6".to_string())
        } else {
          None
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "thermostatSetpointMode" =>
      {
        if current_value.to_ascii_lowercase() == "followSchedule" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "timedSession" =>
      {
        if current_value.to_ascii_lowercase() == "running" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "touch" =>
      {
        if current_value.to_ascii_lowercase() == "touched" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "valve" =>
      {
        if current_value.to_ascii_lowercase() == "open" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "camera" =>
      {
        if current_value.to_ascii_lowercase() == "on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "water" =>
      {
        if current_value.to_ascii_lowercase() == "wet" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "windowShade" =>
      {
        if current_value.to_ascii_lowercase() == "opening" || current_value.to_ascii_lowercase() == "partially open" || current_value.to_ascii_lowercase() == "open" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "optimisation" =>
      {
        if current_value.to_ascii_lowercase() == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "windowFunction" =>
      {
        if current_value.to_ascii_lowercase() == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "rain" =>
      {
        if current_value.to_ascii_lowercase() == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "rainHeavy" =>
      {
        if current_value.to_ascii_lowercase() == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name,
             current_value,
             .. }
        if name.to_ascii_lowercase() == "heatAlarm" =>
      {
        if current_value.to_ascii_lowercase() == "overheat" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref data_type,
             current_value,
             .. }
        if data_type == "NUMBER" =>
      {
        if !current_value.is_empty() {
          Some(current_value.to_string())
        } else {
          None
        }
      },
      Self { ref current_value, .. } if current_value.to_ascii_lowercase() == "on" => {
        if current_value.to_ascii_lowercase() == "on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      _ => None,
    }
  }
}
