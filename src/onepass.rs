use std::process::{Command, Stdio};
use regex::Regex;
use serde_derive::{Serialize, Deserialize};
use serde_json::{Value, from_str, from_value, to_vec};

pub struct OnePassClient {
    token: (String, String),
    vault: String,
}

impl OnePassClient {
    pub fn new(vault: Option<&str>) -> Result<OnePassClient, String> {
        // todo: add separate error for when user account doesn't exist
        let re = Regex::new("export ([\\w_]+)=\"(.*)\"").unwrap();
        let child = Command::new("op").args(&["signin"]).stdin(Stdio::inherit()).stdout(Stdio::piped()).spawn().expect("failed to execute 'op signin'");

        let output = child.wait_with_output().expect("failed to get output");

        let stdout = String::from_utf8(output.stdout).unwrap();

        // todo: make this an error
        assert!(re.is_match(&stdout));

        for cap in re.captures_iter(&stdout) {
            return Ok(OnePassClient {
                token: (cap[1].to_owned(), cap[2].to_owned()),
                vault: vault.map(|s| s.to_string()).unwrap_or("Enigma".to_string()),
            });
        }

        // todo: make this an error
        return Err("could not parse 1Password session token".to_owned());
    }

    pub fn get_variable(&self, name: &str) -> Option<(String, String)> {
        match self.get_item(name) {
            Some(item) => {
                let mut variable = None;
                let mut value = None;

                for field in item.details.fields {
                    if field.name == "variable" {
                        variable = Some(field.value);
                    } else if field.name == "value" {
                        value = Some(field.value);
                    }
                }

                if variable.is_some() && value.is_some() {
                    Some((variable.unwrap(), value.unwrap()))
                } else {
                    None
                }
            },
            None => None,
        }
    }

    pub fn set_variable(&self, name: &str, variable: &str, value: &str) -> Result<(), String> {
        let mut old_uuid = None;

        let new_item = OnePassLogin::for_variable(variable, value);

        if let Some(item) = self.get_item(name) {
            old_uuid = Some(item.uuid);
        }

        self.create_item(name, &new_item)?;

        if let Some(uuid) = old_uuid {
            self.delete_item(&uuid).expect("could not delete old item");
        }

        Ok(())
    }

    fn create_item(&self, name: &str, item: &OnePassLogin) -> Result<(), String> {
        use base64::encode;

        let vault = format!("--vault={}", &self.vault);
        let title = format!("--title={}", name);
        let encoded = encode(&to_vec(&item).unwrap());

        self.command(&["create", "item", "Login", &encoded, &title, &vault])?;

        Ok(())
    }

    pub fn list_items(&self) -> Vec<OnePassItem> {
        let vault = format!("--vault={}", &self.vault);
        let output = self.command(&["list", "items", &vault]).unwrap();
        let raw_items: Vec<Value> = from_str(&output).expect("could not parse items");
        raw_items.into_iter().filter_map(|i| from_value(i).ok()).collect()
    }

    pub fn get_item(&self, name: &str) -> Option<OnePassDetailItem> {
        let vault = format!("--vault={}", &self.vault);
        let output = self.command(&["get", "item", name, &vault]);

        if let Ok(o) = output {
            from_str(&o).ok()
        } else {
            None
        }
    }

    fn delete_item(&self, name: &str) -> Result<(), String> {
        let vault = format!("--vault={}", &self.vault);
        self.command(&["delete", "item", name, &vault])?;
        Ok(())
    }

    fn command(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("op").env(&self.token.0, &self.token.1).args(args).output().map_err(|_| "failed to execute 'op'".to_owned())?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout).map_err(|_| "failed to parse stdout".to_owned())?)
        } else {
            Err(String::from_utf8(output.stderr).map_err(|_| "failed to parse stderr".to_owned())?)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnePassItemOverview {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnePassItem {
    pub uuid: String,
    pub overview: OnePassItemOverview,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnePassField {
    pub name: String,
    pub designation: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnePassDetailItem {
    pub uuid: String,
    pub overview: OnePassItemOverview,
    pub details: OnePassLogin,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnePassLogin {
    pub fields: Vec<OnePassField>,
}

impl OnePassLogin {
    pub fn for_login(username: &str, password: &str) -> OnePassLogin {
        OnePassLogin {
            fields: vec![
                OnePassField {
                    name: "username".to_string(),
                    designation: "username".to_string(),
                    ty: "T".to_string(),
                    value: username.to_string(),
                },
                OnePassField {
                    name: "password".to_string(),
                    designation: "password".to_string(),
                    ty: "P".to_string(),
                    value: password.to_string(),
                },
            ],
        }
    }

    pub fn for_variable(variable: &str, value: &str) -> OnePassLogin {
        OnePassLogin {
            fields: vec![
                OnePassField {
                    name: "variable".to_string(),
                    designation: "username".to_string(),
                    ty: "T".to_string(),
                    value: variable.to_string(),
                },
                OnePassField {
                    name: "value".to_string(),
                    designation: "password".to_string(),
                    ty: "P".to_string(),
                    value: value.to_string(),
                },
            ],
        }
    }
}
