use core::fmt;

use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    #[serde(rename(deserialize = "host"))]
    pub domain: String,
    pub path: String,
    #[serde(rename(deserialize = "expiry"))]
    pub expires: Option<i64>,
    #[serde(rename(deserialize = "httponly"))]
    pub http_only: Option<bool>,
    pub secure: Option<bool>,
    #[serde(
        rename(deserialize = "sameSite"),
        deserialize_with = "boolean",
        default
    )]
    pub same_site: Option<bool>,
}

/// Deserialize 1 as true besides the normal boolean values
fn boolean<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<bool>, D::Error> {
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        Value::Bool(b) => Some(b),
        Value::Number(num) => Some(num.as_i64().ok_or(de::Error::custom("Invalid number"))? != 0),
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MozSession {
    pub cookies: Vec<Cookie>,
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}={}; Domain={}; Path={}",
            self.name, self.value, self.domain, self.path
        )?;

        if let Some(expires) = self.expires {
            write!(f, "; Expires={}", expires)?;
        }

        if let Some(http_only) = self.http_only {
            if http_only {
                write!(f, "; HttpOnly")?;
            }
        }

        if let Some(secure) = self.secure {
            if secure {
                write!(f, "; Secure")?;
            }
        }

        if let Some(same_site) = self.same_site {
            match same_site {
                true => write!(f, "; SameSite=Strict")?,
                false => write!(f, "; SameSite=None")?,
            }
        }

        Ok(())
    }
}

impl Cookie {
    pub fn to_curl_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.domain,
            self.same_site
                .map_or("FALSE".to_string(), |v| v.to_string()),
            self.path,
            self.secure.map_or("FALSE".to_string(), |v| v.to_string()),
            self.expires.map_or("0".to_string(), |v| v.to_string()),
            self.name,
            self.value
        ));

        result
    }
}
