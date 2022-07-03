use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Write,
};

use crate::network::{TopicManifest, TopicManifests};
use crate::parser::list_installed;
use crate::pk::{create_transaction, find_stable_version_of, get_updated_packages, refresh_cache};
use anyhow::Result;
use dbus::blocking::{Connection, Proxy};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string};

const SOURCE_HEADER: &[u8] = b"# Generated by AOSC Topic Manager. DO NOT EDIT THIS FILE!\n";
const SOURCE_PATH: &str = "/etc/apt/sources.list.d/atm.list";
const STATE_PATH: &str = "/var/lib/atm/state";
const STATE_DIR: &str = "/var/lib/atm/";
const DPKG_STATE: &str = "/var/lib/dpkg/status";
const APT_GEN_LIST_STATUS: &str = "/var/lib/apt/gen/status.json";
const DEFAULT_REPO_URL: &str = "https://repo.aosc.io";

#[derive(Deserialize, Debug)]
struct AptGenListStatus {
    mirror: IndexMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Mirror {
    url: String,
}

pub fn get_mirror_url() -> Result<String> {
    let status_data = fs::read(APT_GEN_LIST_STATUS)?;
    let status_data: AptGenListStatus = serde_json::from_slice(&status_data)?;

    for (_, url) in status_data.mirror {
        let url = if url.ends_with('/') {
            url.clone()
        } else {
            format!("{}/", url)
        };

        if get(&url).is_ok() {
            return Ok(url);
        }
    }

    Ok(DEFAULT_REPO_URL.to_string())
}

lazy_static! {
    pub static ref MIRROR_URL: String =
        get_mirror_url().unwrap_or_else(|_| "https://repo.aosc.io/".to_string());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreviousTopic {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub date: i64,
    pub packages: Vec<String>,
}

type PreviousTopics = Vec<PreviousTopic>;

/// Returns the packages need to be reinstalled
pub fn close_topics(topics: &[TopicManifest]) -> Result<Vec<String>> {
    let state_file = fs::read(DPKG_STATE)?;
    let installed = list_installed(&state_file)?;
    let mut remove = Vec::new();

    for topic in topics {
        for package in topic.packages.iter() {
            if installed.contains(package) {
                remove.push(package.clone());
            }
        }
    }

    Ok(remove)
}

/// Returns the list of enrolled topics
fn get_previous_topics() -> Result<PreviousTopics> {
    let f = fs::read(STATE_PATH)?;

    Ok(from_slice(&f)?)
}

pub fn get_display_listing(current: TopicManifests) -> TopicManifests {
    let prev = get_previous_topics().unwrap_or_default();
    let mut lookup: HashMap<String, TopicManifest> = HashMap::new();
    let current_len = current.len();

    for topic in current.into_iter() {
        lookup.insert(topic.name.clone(), topic);
    }

    let mut concatenated = Vec::new();
    concatenated.reserve(prev.len() + current_len);
    for topic in prev {
        if let Some(topic) = lookup.get_mut(&topic.name) {
            topic.enabled = true;
            continue;
        }
        concatenated.push(TopicManifest {
            enabled: false,
            closed: true,
            name: topic.name.clone(),
            description: topic.description.clone(),
            date: topic.date,
            arch: HashSet::new(),
            packages: topic.packages.clone(),
        });
    }
    // consume the lookup table and append all the elements to the concatenated list
    for topic in lookup.into_iter() {
        concatenated.push(topic.1);
    }

    concatenated
}

fn save_as_previous_topics(current: &[&TopicManifest]) -> Result<String> {
    let mut previous_topics = Vec::new();
    for topic in current {
        if !topic.enabled {
            continue;
        }
        previous_topics.push(PreviousTopic {
            name: topic.name.clone(),
            description: topic.description.clone(),
            date: topic.date,
            packages: topic.packages.clone(),
        });
    }

    Ok(to_string(&previous_topics)?)
}

fn make_topic_list(topics: &[&TopicManifest]) -> String {
    let mut output = String::new();
    output.reserve(1024);

    for topic in topics {
        output.push_str(&format!(
            "# Topic `{}`\ndeb {} {} main\n",
            topic.name,
            format!("{}{}", MIRROR_URL.to_string(), "debs"),
            topic.name
        ));
    }

    output
}

pub fn write_source_list(topics: &[&TopicManifest]) -> Result<()> {
    let mut f = fs::File::create(SOURCE_PATH)?;
    f.write_all(SOURCE_HEADER)?;
    f.write_all(make_topic_list(topics).as_bytes())?;

    fs::create_dir_all(STATE_DIR)?;
    let mut f = fs::File::create(STATE_PATH)?;
    f.write_all(save_as_previous_topics(topics)?.as_bytes())?;

    Ok(())
}

pub fn switch_topics(
    proxy: &Proxy<&Connection>,
    closed: &[TopicManifest],
) -> Result<(Vec<String>, Vec<String>)> {
    let tx_proxy = create_transaction(&proxy)?;
    refresh_cache(&tx_proxy)?;
    let removed = close_topics(closed)?;
    let removed = removed.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    let tx_proxy = create_transaction(&proxy)?;
    let (not_found, tasks) = find_stable_version_of(&tx_proxy, &removed)?;
    let tx_proxy = create_transaction(&proxy)?;
    let updated = get_updated_packages(&tx_proxy)?;
    let mut updated = updated
        .into_iter()
        .map(|x| x.package_id)
        .collect::<Vec<_>>();
    updated.extend(tasks);

    Ok((not_found, updated))
}
