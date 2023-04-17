use wallpaper;
use chrono;
use reqwest;
use serde_json;

fn parse_get_url() -> String {
    let resp = reqwest::blocking::get("https://cn.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1").unwrap();
    let json: serde_json::Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
    let images = json["images"].as_array().unwrap();
    let image = images[0].as_object().unwrap();
    let url = image["urlbase"].as_str().unwrap();
    let url = format!("https://www.bing.com{}_UHD.jpg&rf=LaDigue_UHD.jpg&pid=hp&rs=1&c=4&qlt=100&uhd=1&uhdwidth=3840&uhdheight=2160", url);
    return url;
}

#[test]
fn test_parse_get_url() {
    let url = parse_get_url();
    println!("{}", url);
    assert!(url.starts_with("https://www.bing.com"));
    assert!(url.ends_with("_UHD.jpg&rf=LaDigue_UHD.jpg&pid=hp&rs=1&c=4&qlt=100&uhd=1&uhdwidth=3840&uhdheight=2160"));
}

fn main() {
    let mut _url: String = "".to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            println!("Usage: download and set wallpaper from a url (or `default-server`), 
       rm *.set to download again.
       pass nothing to auto parse url from cn.bing.com");
            return;
        } else {
            let content = &args[1];
            if content.eq_ignore_ascii_case("default-server") {
                _url = "https://go.mazhangjing.com/bing-today-image".to_string();
            } else {
                _url = content.to_string();
            }
        }
    }
    if _url.is_empty() {
        _url = parse_get_url();
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
    wallpaper::set_from_url(_url.as_str()).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    std::fs::File::create(format!("{}.set", date)).unwrap();
}
