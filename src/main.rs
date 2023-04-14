use wallpaper;
use chrono;

fn main() {
    let mut _url = "https://go.mazhangjing.com/bing-today-image";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            println!("Usage: download and set wallpaper from a url (or default), rm *.set to download again");
            return;
        } else {
            _url = &args[1];
        }
    }
    println!("setting wallpaper from {}", _url);
    //get date of now, format as 20200101
    let now = chrono::Local::now();
    let date = now.format("%Y%m%d").to_string();
    //if exist 20200101.set, return
    if std::path::Path::new(&format!("{}.set", date)).exists() {
        println!("{} already set", date);
        return;
    } 
    //set wallpaper
    wallpaper::set_from_url(_url).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    std::fs::File::create(format!("{}.set", date)).unwrap();
}
