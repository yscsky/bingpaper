use std::{io, path::Path};

use widestring::U16CString;
use winapi::um::{
    winnt::PVOID,
    winuser::{
        SystemParametersInfoW, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
    },
};

pub fn list_screens() -> Vec<String> {
    vec!["screen".to_string()]
}

pub fn set_screen_paper(_: &str, picture: &str) -> bool {
    let path = U16CString::from_os_str(Path::new(picture).as_os_str()).unwrap();
    let res = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as PVOID,
            SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
        )
    };
    match res {
        1 => true,
        v => {
            println!("set paper err: {}", io::Error::from_raw_os_error(v));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_screen_paper() {
        assert!(set_screen_paper(
            "",
            "D:/Pictures/WallPapers\\阿尼瓦角的灯塔，俄罗斯萨哈林岛.jpg"
        ));
    }
}
