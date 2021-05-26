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

#[derive(Debug, Deserialize)]
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
      Self { ref name, current_value, .. } if name == "acceleration" => {
        if current_value == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "alarm" => {
        if current_value == "off" || current_value == "inactive" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "presence" => {
        if current_value == "present" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "presence" => {
        if current_value == "present" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "switch" => {
        if current_value == "on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "button" => {
        if current_value == "pushed" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "carbonMonoxide" => {
        if current_value == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "status" => {
        if current_value == "playing" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "consumableStatus" => {
        if current_value == "good" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "contact" => {
        if current_value == "closed" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "indicatorStatus" => {
        if current_value == "when on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "lock" => {
        if current_value == "locked" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "motion" => {
        if current_value == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "shock" => {
        if current_value == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "mute" => {
        if current_value == "muted" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "sleeping" => {
        if current_value == "sleeping" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "smoke" => {
        if current_value == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "sound" => {
        if current_value == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "tamper" => {
        if current_value == "detected" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "thermostatMode" => {
        if current_value == "off" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "thermostatFanMode" => {
        if current_value == "off" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "thermostatOperatingState" => {
        if current_value == "heating" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "thermostatSetpointMode" => {
        if current_value == "followSchedule" {
          Some("0".to_string())
        } else {
          Some("1".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "timedSession" => {
        if current_value == "running" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "touch" => {
        if current_value == "touched" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "valve" => {
        if current_value == "open" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "camera" => {
        if current_value == "on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "water" => {
        if current_value == "wet" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "windowShade" => {
        if current_value == "opening" || current_value == "partially open" || current_value == "open" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "optimisation" => {
        if current_value == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "windowFunction" => {
        if current_value == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "rain" => {
        if current_value == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "rainHeavy" => {
        if current_value == "active" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref name, current_value, .. } if name == "heatAlarm" => {
        if current_value == "overheat" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      Self { ref data_type, current_value, .. } if data_type == "NUMBER" => {
        if !current_value.is_empty() {
          Some(current_value.to_string())
        } else {
          None
        }
      },
      Self { ref current_value, .. } if current_value == "on" => {
        if current_value == "on" {
          Some("1".to_string())
        } else {
          Some("0".to_string())
        }
      },
      _ => None,
    }
  }
}
