use std::process::Command;

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
}
