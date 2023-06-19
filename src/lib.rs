use std::{
    env::{self, VarError},
    error::Error,
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

use chrono::Utc;
use serde::Deserialize;

pub fn list_screens() -> Vec<String> {
    let out = Command::new("xfconf-query")
        .arg("-c")
        .arg("xfce4-desktop")
        .arg("-l")
        .output()
        .expect("xfconf query list fail");
    let list = String::from_utf8(out.stdout).unwrap();
    list.lines()
        .into_iter()
        .filter(|s| {
            (s.contains("DisplayPort") || s.contains("HDMI"))
                && s.ends_with("workspace0/last-image")
        })
        .map(|s| s.to_string())
        .collect()
}

pub fn get_screen_paper(screen: String) -> String {
    let out = Command::new("xfconf-query")
        .arg("-c")
        .arg("xfce4-desktop")
        .arg("-p")
        .arg(screen)
        .output()
        .expect("xfconf query property fail");
    String::from_utf8(out.stdout).unwrap()
}

pub fn set_screen_paper(screen: &str, picture: &str) -> bool {
    let status = Command::new("xfconf-query")
        .arg("-c")
        .arg("xfce4-desktop")
        .arg("-p")
        .arg(screen)
        .arg("-s")
        .arg(picture)
        .status()
        .expect("xfconf query set fail");
    status.success()
}

pub fn get_home() -> Result<String, VarError> {
    const HOME: &str = "BING_PAPER_HOME";
    env::var(HOME)
}

pub fn list_pictures(home: &str) -> io::Result<Vec<String>> {
    let paths = fs::read_dir(home)?;
    let mut list = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        list.push(String::from(path.to_str().unwrap()));
    }
    Ok(list)
}

#[derive(Debug, Deserialize)]
struct ImageResp {
    images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
struct Image {
    url: String,
    copyright: String,
}

pub fn get_bing_paper(home: &str, index: usize) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://cn.bing.com/HPImageArchive.aspx?format=js&idx={}&n=1&nc={}&pid=hp",
        index,
        Utc::now().timestamp_millis(),
    );
    download_bing_paper(url, home)
}

pub fn get_global_bing_paper(home: &str, index: usize) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://cn.bing.com/HPImageArchive.aspx?format=js&idx={}&n=1&nc={}&pid=hp&ensearch=1",
        index,
        Utc::now().timestamp_millis(),
    );
    download_bing_paper(url, home)
}

fn download_bing_paper(url: String, home: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?.json::<ImageResp>()?;
    if resp.images.len() == 0 {
        return Err("no images")?;
    }
    let path = format!("{}/{}", home, convert_name(&resp.images[0].copyright));
    if Path::new(&path).exists() {
        println!("picture exists: {}", path);
        return Ok(path);
    }
    let content =
        reqwest::blocking::get(format!("https://cn.bing.com/{}", resp.images[0].url))?.bytes()?;
    fs::File::create(&path)?.write_all(&content)?;
    Ok(path)
}

fn convert_name(copyright: &str) -> String {
    let l = copyright
        .find('(')
        .or_else(|| copyright.find('©'))
        .unwrap_or(copyright.len());
    let mut name: String = String::from(copyright).drain(..l - 1).collect();
    name = name.replace("/", "_");
    name.push_str(".jpg");
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_screens() {
        let screens = list_screens();
        println!("screens: {:?}", screens);
    }

    #[test]
    fn test_get_screen_paper() {
        let pic = get_screen_paper(
            "/backdrop/screen0/monitorDisplayPort-0/workspace0/last-image".to_string(),
        );
        println!("pic:{pic}");
    }

    #[test]
    fn test_set_screen_paper() {
        assert!(set_screen_paper(
            "/backdrop/screen0/monitorHDMI-A-0/workspace0/last-image",
            "/home/ysc/Pictures/WallPapers/山上的日出，河北蔚县.jpg"
        ));
    }

    #[test]
    fn test_list_pictures() {
        let pics = list_pictures("/home/ysc/Pictures/WallPapers").unwrap();
        println!("pics:{:?}", pics);
    }

    #[test]
    fn test_get_bing_paper() {
        get_bing_paper("/home/ysc/Pictures/WallPapers", 0).unwrap();
        get_global_bing_paper("/home/ysc/Pictures/WallPapers", 0).unwrap();
    }

    #[test]
    fn test_convert_name() {
        let copyright = "游泳的鹰嘴海龟， 冲/绳，日/本 (© Robert Mallon/Getty Images)";
        assert_eq!(convert_name(copyright), "游泳的鹰嘴海龟， 冲_绳，日_本.jpg");
    }
}
