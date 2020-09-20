use std::io;
use std::io::BufReader;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::thread;

use std::collections::HashMap;

use console::Term;
use console::style;

mod settings;
use settings::Settings;

//use reqwest;
use walkdir::WalkDir;
use glob::glob;
use fs_extra;
use serde_json::Value;

// 8 bit colors: https://jonasjacek.github.io/colors/
// TODO 20.09.2020: Download the appids.json instead of manually adding it.
fn main() {
    if let Err(_e) = run() {
        println!("Something went wrong!");
        std::process::exit(1);
    }
    else {
        std::process::exit(0);
    }    
}

fn run() -> io::Result<()> {
    let exe = std::env::current_exe().unwrap();
    let app_dir = exe.parent().unwrap();

    let appid_path = &app_dir.join("appids.json");
    let dir_settings = &app_dir.join("settings.toml");

    let term = Term::stdout();
    term.hide_cursor()?;

    term.set_title("Steam Screenshot Backup | Rust Edition");

    //term.write_line(format!("{}", style("Found game 'TEST' with AppID '42000'").color256(242)).as_str())?;
    //term.write_line(format!("{}", style("C:\\Users\\wolji\\Pictures\\Screenshots\\Screenshot (471).png").color256(248)).as_str())?;  

    //finish(&term);
    
    // SETTINGS - START
    if !dir_settings.exists() {
        Settings::save(dir_settings.as_path(), &Settings::new());
        term.write_line("Settings file created!\nPlease edit and press any key to continue!")?;
        term.show_cursor()?;
        drop(term.read_key());
    }

    let m_settings: Settings = Settings::load(dir_settings.as_path());
    // SETTINGS - END


    // ARG PROCESSING - START
    let mut noinput: bool = false;
    for x in std::env::args() {
        match x.as_str() {
            "-noinput" => {
                noinput = true;
            },
            _ => {}
        }
    }
    // ARG PROCESSING - END

    // APP ID LIST - START

    // Note: Fairly memory intensive in the beginning peaking at around 80mb but drops down to around 10mb when finished.
    let appid_map: HashMap<i32, String> = {
        let appids: Value = {

            let file_appids = File::open(appid_path).unwrap();
            let appids_reader = BufReader::new(file_appids);

            serde_json::from_reader(appids_reader).unwrap()
        };

         let mut _map: HashMap<i32, String> = HashMap::new();
         let appid_length = appids["applist"]["apps"].as_array().unwrap().len();
    
        for i in 0..appid_length {
            let _appid = appids["applist"]["apps"][i]["appid"].as_i64().unwrap().clone() as i32;
            let _name = appids["applist"]["apps"][i]["name"].to_string();
            
            _map.insert(_appid, _name);
        }

        _map.insert(0, "Empty".to_string());

        _map
    };
    // APP ID LIST - END

    for entry in WalkDir::new(&m_settings.steam_folder).follow_links(false).into_iter() {
        let e = &entry.unwrap();
        if e.file_type().is_dir() && e.file_name().to_string_lossy() == "screenshots" {
            let folder_id: i32 = e.clone().path().parent().unwrap().file_name().unwrap().to_string_lossy().trim().parse().unwrap_or(0);

            if appid_map.contains_key(&folder_id) {
                let mut retreived_app_name = appid_map.get(&folder_id).unwrap().clone();
                retreived_app_name.retain(|c| !r#"[\/?:*""><|]+"#.contains(c)); // FILTER
                retreived_app_name = retreived_app_name.trim().to_string();

                term.write_line(format!("{}", style(format!("Found game '{0}' with AppID '{1}'", &retreived_app_name, &folder_id).as_str()).color256(244)).as_str())?;
            
                if folder_id > 0 {
                    let target_path = &Path::new(&m_settings.target_folder).join(retreived_app_name);
                    if !target_path.exists() {
                        fs::create_dir_all(target_path).unwrap();
                    }
                    
                    let options = fs_extra::dir::CopyOptions::new();

                    for entry_img in glob(e.path().join("*.jpg").to_str().unwrap()).unwrap() {
                        if let Ok(img) = entry_img {

                            let target_file = target_path.join(img.file_name().unwrap());
                            
                            let mut from_paths = Vec::new();
                            from_paths.push(img);

                            if !target_file.exists() {

                                term.write_line(format!("{}", style(target_file.to_str().unwrap()).color256(250)).as_str())?;

                                let copy = fs_extra::copy_items(&from_paths, &target_path, &options);
                                let copy = match copy {
                                    Ok(_) => {}
                                    Err(err) => println!("ERROR: {}", err),
                                };

                                thread::sleep(std::time::Duration::from_millis(30));
                            }  
                            
                            drop(from_paths);
                        }
                    }
                }
            }       
        }
    }
 
    //let client = reqwest::Client::new();
    /*
    for x in 0..255 {
        term.write_str(format!("{0}", style("X").color256(x)).as_str())?;
    }
    */
    
    //term.write_line(format!("I have a {0}, haha!", style("lightsaber").color256(243)).as_str())?;
    //term.write_line(format!("Steam Folder: '{}'", m_settings.steam_folder).as_str())?;
    //term.write_line("Done!")?;

    if !noinput {
        finish(&term);
    }
    Ok(())
}

// Just a test function i made to test our borrowing.
fn finish(term: &Term) {
    term.write_line("Done! Press ANY KEY to exit!").unwrap();
    term.show_cursor().unwrap();
    //drop(term.read_key());
    drop(term.read_line());
}