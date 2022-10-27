mod hub;

use std::{collections::HashMap, io::Write};

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, ArgMatches, Command};

extern crate tiny_http;
use tiny_http::{Response, Server};

use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use reqwest::blocking::RequestBuilder;

fn main() {
  let app = Command::new("").version(env!("CARGO_PKG_VERSION"))
                            .author(env!("CARGO_PKG_AUTHORS"))
                            .about(env!("CARGO_PKG_DESCRIPTION"))
                            .arg(Arg::new("listener").long("listener").env("LISTENER").default_value("0.0.0.0:8000").num_args(1))
                            .arg(Arg::new("he_ip").short('i').long("hubitat_ip").env("HE_IP").help("Hubitat Hub IP").required(true).num_args(1))
                            .arg(Arg::new("he_app_id").short('a').long("hubitat_app_id").env("HE_APP_ID").help("Hubitat APP ID").required(true).num_args(1))
                            .arg(Arg::new("he_api_token").short('t').long("hubitat_api_access_token").env("HE_API_TOKEN").help("Hubitat API TOKEN").required(true).num_args(1))
                            .arg(Arg::new("he_dd").short('d').long("hubitat_device_details").env("HE_DD").help("Add extra detailed labels").action(clap::ArgAction::SetTrue).default_missing_value("false").required(false))
                            .arg(Arg::new("he_auth_usr").short('u').long("hubitat_auth_usr").env("HE_AUTH_USR").help("Hubitat Hub Username").required(false).num_args(1))
                            .arg(Arg::new("he_auth_pwd").short('p').long("hubitat_auth_pwd").env("HE_AUTH_PWD").help("Hubitat Hub Password").requires("he_auth_usr").required(false).num_args(1))
                            .arg(Arg::new("v").short('v').action(clap::ArgAction::Count).required(false).help("Log verbosity (-v, -vv, -vvv...)"))
                            .get_matches();

  match app.get_one::<u8>("v").unwrap() {
    0 => std::env::set_var("RUST_LOG", "error"),
    1 => std::env::set_var("RUST_LOG", "warn"),
    2 => std::env::set_var("RUST_LOG", "info"),
    3 => std::env::set_var("RUST_LOG", "debug"),
    4 => std::env::set_var("RUST_LOG", "trace"),
    _ => std::env::set_var("RUST_LOG", "trace"),
  }

  env_logger::Builder::from_default_env().format(|buf, record| writeln!(buf, "{} {} {}:{} [{}] - {}", chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"), record.module_path().unwrap_or("unknown"), record.file().unwrap_or("unknown"), record.line().unwrap_or(0), record.level(), record.args())).init();

  let mut he = hub::HubInfo { ip: app.get_one::<String>("he_ip").map(|s| s.as_str()), auth_usr: None, auth_pwd: None, api_id: app.get_one::<String>("he_app_id").map(|s| s.as_str()), api_access_token: app.get_one::<String>("he_api_token").map(|s| s.as_str()), client: None };

  let listener = app.get_one::<String>("listener").unwrap();
  if let Ok(server) = Server::http(listener) {
    info!("started on http://{}", listener);

    if app.get_flag("he_dd") {
      info!("detailed mode is turned on");
      get_log(&mut he, &app);
    }

    for request in server.incoming_requests() {
      let dev_inv = get_device_inventory(&mut he, &app);
      trace!("dev_inv:{:#?}", dev_inv);

      let devs = get_device_details(&he, get_device_ids(&he));
      trace!("devs:{:#?}", devs);

      let m = build_metrics(devs, &dev_inv);
      let response = Response::from_string(m);
      let _ = request.respond(response);
    }
  } else {
    error!("Starting web server with listener {:?}", listener);
  }
}

fn build_metrics(devs: Result<Vec<hub::Device>, anyhow::Error>, dev_inv: &Option<HashMap<String, hub::DeviceInventory>>) -> String {
  let mut metrics: String = "".to_owned();

  if let Ok(dev_details) = devs {
    for i in dev_details.iter() {
      if let Some(d) = dev_inv {
        match d.get(&i.id) {
          Some(d) => {
            // Detailed mode without the Device Inventory
            for a in i.attributes.iter() {
              if let Some(v) = a.get_numeric_value() {
                let m = &format!("{metric}{{hub_name=\"{hub_name}\",hub_location_name=\"{hub_location_name}\",device_network_id=\"{device_network_id}\",device_driver_type=\"{device_driver_type}\",device_driver=\"{device_driver}\",device_name=\"{device_name}\",device_label=\"{device_label}\"}} {val}\n", metric = a.name.to_case(Case::Snake), hub_name = d.hub_name, hub_location_name = d.location_name, device_network_id = d.device_network_id, device_driver_type = i.r#type, device_driver = d.device_type_name, device_name = i.name, device_label = i.label, val = v);
                metrics.push_str(m);
              }
            }
          },
          _ => {
            if !&i.id.is_empty() {
              warn!("Device ID: {:?} not found", &i.id);
            }
          },
        }
      } else {
        // Simple mode without the Device Inventory
        for a in i.attributes.iter() {
          if let Some(v) = a.get_numeric_value() {
            let m = &format!("{metric}{{device_name=\"{device_name}\",device_label=\"{device_label}\",device_driver_type=\"{device_driver_type}\"}} {val}\n", metric = a.name.to_case(Case::Snake), device_name = i.name, device_label = i.label, device_driver_type = i.r#type, val = v);
            metrics.push_str(m);
          }
        }
      }
    }
  }

  metrics
}

fn get_log(he: &mut hub::HubInfo, app: &ArgMatches) {
  if let Ok(c) = reqwest::blocking::Client::builder().user_agent(env!("CARGO_PKG_NAME")).cookie_store(true).danger_accept_invalid_certs(true).connection_verbose(true).build() {
    let req_url = format!("http://{he_ip}/login", he_ip = he.ip.unwrap());

    let req: RequestBuilder = if app.get_one::<String>("he_auth_usr").is_some() && app.get_one::<String>("he_auth_pwd").is_some() {
      debug!("Auth on {:?}/{:?}", app.get_one::<String>("he_auth_usr").unwrap(), app.get_one::<String>("he_auth_pwd").unwrap());

      let mut params = HashMap::new();
      params.insert("username", app.get_one::<String>("he_auth_usr").unwrap());
      params.insert("password", app.get_one::<String>("he_auth_pwd").unwrap());
      c.post(req_url).form(&params)
    } else {
      c.post(req_url)
    };

    match req.send() {
      Ok(r) => {
        debug!("resp:{:#?}", r);

        if r.status().is_success() {
          he.client = Some(c)
        } else {
          he.client = None;
          error!("request post failed: {:?}", r);
        }
      },
      Err(e) => {
        he.client = None;
        error!("request post failed: {:?}", e);
      },
    }
  }
}

fn get_device_inventory(he: &mut hub::HubInfo, app: &ArgMatches) -> Option<HashMap<String, hub::DeviceInventory>> {
  if let Some(c) = &he.client {
    let req_url = format!("http://{he_ip}/device/list/all/data", he_ip = he.ip.unwrap());

    match c.get(req_url).send() {
      Ok(r) => {
        if r.status().is_success() {
          debug!("resp:{:#?}", r);
          if let Some(_h) = r.headers().get(reqwest::header::X_FRAME_OPTIONS) {
            get_log(he, app);
            error!("loging failed");
            return None;
          }
          match r.json::<Vec<hub::DeviceInventory>>() {
            Ok(dev) => {
              let inv = dev.into_iter().map(|item| (item.id.clone(), item)).collect();
              Some(inv)
            },
            Err(e) => {
              error!("json parsing failed: {:?}", e);
              None
            },
          }
        } else {
          error!("request get failed: {:?}", r);
          None
        }
      },
      Err(e) => {
        error!("request get failed: {:?}", e);
        None
      },
    }
  } else {
    None
  }
}

fn get_device_ids(he: &hub::HubInfo) -> Result<Vec<u32>, anyhow::Error> {
  let req_url = format!("http://{he_ip}/apps/api/{he_api_id}/devices?access_token={he_api_token}", he_ip = he.ip.unwrap(), he_api_id = he.api_id.unwrap(), he_api_token = he.api_access_token.unwrap());
  let client = reqwest::blocking::Client::builder().user_agent(env!("CARGO_PKG_NAME")).danger_accept_invalid_certs(true).connection_verbose(true).build().expect("Error building client");

  match client.get(req_url).send() {
    Ok(r) => {
      debug!("resp:{:#?}", r);
      if r.status().is_success() {
        match r.json::<Vec<hub::DeviceIDs>>() {
          Ok(d) => {
            let ids: Vec<u32> = d.iter().map(|i| i.id.trim().parse::<u32>()).filter_map(Result::ok).collect();
            Ok(ids)
          },
          Err(e) => Err(anyhow!("json parsing failed: {:?}", e)),
        }
      } else {
        Err(anyhow!("request get failed: {:?}", r))
      }
    },
    Err(e) => Err(anyhow!("request get failed: {:?}", e)),
  }
}

fn get_device_details(he: &hub::HubInfo, ids: Result<Vec<u32>, anyhow::Error>) -> Result<Vec<hub::Device>, anyhow::Error> {
  let mut devs: Vec<hub::Device> = vec![hub::Device::default()];

  if let Ok(dev_ids) = ids {
    for i in dev_ids.iter() {
      let req_url = format!("http://{he_ip}/apps/api/{he_api_id}/devices/{dev_id}?access_token={he_api_token}", he_ip = he.ip.unwrap(), he_api_id = he.api_id.unwrap(), he_api_token = he.api_access_token.unwrap(), dev_id = i);
      let client = reqwest::blocking::Client::builder().user_agent(env!("CARGO_PKG_NAME")).danger_accept_invalid_certs(true).connection_verbose(true).build().expect("Error building client");

      match client.get(req_url).send() {
        Ok(r) => {
          debug!("resp:{:#?}", r);
          if r.status().is_success() {
            match r.json::<hub::Device>() {
              Ok(d) => devs.push(d),
              Err(e) => {
                error!("json parsing failed: {:?}", e);
              },
            }
          } else {
            return Err(anyhow!("request get failed: {:?}", r));
          }
        },
        Err(e) => {
          return Err(anyhow!("request get failed: {:?}", e));
        },
      }
    }
  }

  Ok(devs)
}
