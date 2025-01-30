use clap::Parser;

use rusqlite::Connection;

use std::{
    fs,
    path::{Path, PathBuf},
};
use types::{Cookie, MozSession};

use crate::cli::OutputFormat;

mod cli;
mod types;

#[cfg(windows)]
const PROFILES_DIR: &str = "~/AppData/Roaming/Mozilla/Firefox/Profiles";
#[cfg(unix)]
const PROFILES_DIR: &str = "~/.mozilla/firefox";

fn main() {
    let cli = cli::Cli::parse();

    let profiles_dir: PathBuf = match cli.profile_dir {
        Some(p) => PathBuf::from(shellexpand::tilde(&p.to_string_lossy()).to_string()),
        None => PathBuf::from(shellexpand::tilde(PROFILES_DIR).to_string()),
    };

    if !profiles_dir.exists() {
        eprintln!(
            "Path to firefox profiles doesn't exist: `{}`",
            profiles_dir.display()
        )
    }

    let profile_dir = get_profile_dir(&profiles_dir, cli.profile.as_deref());

    let mut cookies = get_sqlite_cookies(&profile_dir.join("cookies.sqlite"));

    let session_cookies_path = profile_dir
        .join("sessionstore-backups")
        .join("recovery.jsonlz4");

    if session_cookies_path.exists() {
        cookies.extend(get_session_cookies(&session_cookies_path))
    }

    if let Some(domain) = cli.domain {
        cookies.retain(|c| c.domain == domain || c.domain == format!(".{domain}"));
    }

    match cli.output_format {
        OutputFormat::Javascript => {
            println!(
                "{}",
                cookies
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
        OutputFormat::Netscape => {
            print!(
                "{}",
                cookies
                    .iter()
                    .map(|c| c.to_curl_string())
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string(&cookies).expect("Not serializable to json")
            )
        }
    }
}

fn get_profile_dir(base_dir: &Path, profile: Option<&str>) -> PathBuf {
    for path in fs::read_dir(base_dir).unwrap() {
        let path = path.unwrap();
        if path.file_type().unwrap().is_dir() {
            if path
                .path()
                .to_str()
                .unwrap()
                .ends_with(profile.unwrap_or("default"))
            {
                return path.path();
            }
        }
    }

    for path in fs::read_dir(base_dir).unwrap() {
        let path = path.unwrap();
        if path.file_type().unwrap().is_dir() {
            if path
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(profile.unwrap_or("default"))
            {
                return path.path();
            }
        }
    }

    unreachable!()
}

fn get_sqlite_cookies(db_path: &Path) -> Vec<Cookie> {
    // let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    // let db_data = fs::read(db_path).unwrap();
    // tmpfile.write_all(&db_data).unwrap();

    // let conn = Connection::open(tmpfile).unwrap();
    let conn = Connection::open(db_path).unwrap();
    let mut stmt = conn.prepare(
        "SELECT name, value, host, path, expiry, isHttpOnly, isSecure, sameSite FROM moz_cookies"
    ).unwrap();

    let cookie_iter = stmt
        .query_map([], |row| {
            Ok(Cookie {
                name: row.get(0).unwrap(),
                value: row.get(1).unwrap(),
                domain: row.get(2).unwrap(),
                path: row.get(3).unwrap(),
                expires: row.get(4).ok(),
                http_only: row.get(5).ok(),
                secure: row.get(6).ok(),
                same_site: row.get(7).ok(),
            })
        })
        .unwrap();

    cookie_iter.map(|c| c.expect("Row Error")).collect()
}

fn get_session_cookies(file_path: &Path) -> Vec<Cookie> {
    // Remove mozilla non standard header
    let bytes = &fs::read(file_path).unwrap()[8..];

    let json: MozSession =
        serde_json::from_slice(&lz4::block::decompress(bytes, None).unwrap()).unwrap();

    json.cookies
}
