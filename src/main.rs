use std::io;

use bingpaper::{get_bing_paper, get_home, list_pictures, list_screens, set_screen_paper};
use clap::Parser;
use rand::Rng;

/// Set wallpaper using bing daily picture
#[derive(Parser, Debug)]
struct Args {
    /// download last bing wallpapar
    #[arg(short, long)]
    new: bool,
    /// list all pictures
    #[arg(short, long)]
    list: bool,
    /// index pictures
    #[arg(short, long, value_name = "NUMBER", default_value_t = 0)]
    index: usize,
    /// download global last bing wallpapar
    #[arg(short, long)]
    global: bool,
    /// select screen
    #[arg(short, long, value_name = "NUMBER", default_value_t = 0)]
    screen: usize,
}

fn main() {
    let args = Args::parse();

    let home = match get_home() {
        Ok(v) => v,
        Err(e) => {
            println!("Get Home fail, err:{}", e);
            println!("You should export BING_PAPER_HOME environment");
            return;
        }
    };

    if args.new {
        new_picture(&home, args.index, args.screen);
    }

    if args.global {}

    if args.list {
        display_pictures(&home);
        return;
    }

    if args.index > 0 {
        select_picture(&home, args.index, args.screen).expect("select picture fail");
        return;
    }

    // random pick a picture to set wallpaper
    random_picture(&home, args.screen).expect("random picture fail");
}

fn new_picture(home: &str, index: usize, screen_index: usize) {
    let path = match get_bing_paper(&home, index) {
        Ok(p) => p,
        Err(e) => {
            println!("get bing paper err: {e}");
            return;
        }
    };
    let screens = list_screens();
    if screen_index >= screens.len() {
        println!("screen index large than screen count");
        return;
    }
    if set_screen_paper(&screens[screen_index], &path) {
        println!("screen{}: {}", &screens[screen_index], &path);
    } else {
        println!("set screen paper fail");
    }
}

fn display_pictures(home: &str) {
    let pictures = list_pictures(&home).expect("list picture fail");
    for (i, p) in pictures.iter().enumerate() {
        let no = i + 1;
        println!("{no}: {p}");
    }
}

#[test]
fn test_display_pictures() {
    display_pictures("/home/ysc/Pictures/WallPapers");
}

fn select_picture(home: &str, index: usize, screen_index: usize) -> io::Result<()> {
    let pictures = list_pictures(home)?;
    let index = index - 1;
    if index >= pictures.len() {
        println!("index large than picture count");
        return Ok(());
    }
    let screens = list_screens();
    if screen_index >= screens.len() {
        println!("screen index large than screen count");
        return Ok(());
    }
    if set_screen_paper(&screens[screen_index], &pictures[index]) {
        println!("screen{}: {}", &screens[screen_index], &pictures[index]);
    } else {
        println!("set screen paper fail");
    }
    Ok(())
}

#[test]
fn test_select_picture() {
    let _ = select_picture("/home/ysc/Pictures/WallPapers", 10, 0);
    let _ = select_picture("/home/ysc/Pictures/WallPapers", 20, 1);
}

fn random_picture(home: &str, screen_index: usize) -> io::Result<()> {
    let pictures = list_pictures(home)?;
    let rand_idx = rand::thread_rng().gen_range(0..pictures.len());
    let screens = list_screens();
    if screen_index >= screens.len() {
        println!("screen index large than screen count");
        return Ok(());
    }
    if set_screen_paper(&screens[screen_index], &pictures[rand_idx]) {
        println!("screen{}: {}", &screens[screen_index], &pictures[rand_idx]);
    } else {
        println!("set screen paper fail");
    }
    Ok(())
}

#[test]
fn test_random_picture() {
    let _ = random_picture("/home/ysc/Pictures/WallPapers", 0);
    let _ = random_picture("/home/ysc/Pictures/WallPapers", 1);
}
