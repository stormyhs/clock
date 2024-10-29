use toml_edit;
use std::fs;
use std::process;
use dirs::home_dir;

static FILE_NAME: &str = "clock.toml";

pub fn read_data() -> toml_edit::DocumentMut {
    let data_path = home_dir()
        .expect("Could not find home directory")
        .join(FILE_NAME);

    if fs::metadata(&data_path).is_err() {
        match fs::File::create(&data_path) {
            Ok(f) => {
                println!("✔️ Created 'clock.toml'");
                f
            },
            Err(e) => {
                println!("❌ Could not create 'clock.toml'");
                println!("{e}");
                process::exit(1);
            }
        };
    }

    let data = match fs::read_to_string(&data_path) {
        Ok(d) => d,
        Err(e) => {
            println!("❌ Could not read 'clock.toml'");
            println!("{e}");
            process::exit(1);
        }
    };

    match data.parse() {
        Ok(d) => d,
        Err(e) => {
            println!("❌ Could not parse 'clock.toml'");
            println!("{e}");
            process::exit(1);
        }
    }
}

pub fn write_data(doc: toml_edit::DocumentMut) {
    let data_path = home_dir()
        .expect("Could not find home directory")
        .join(FILE_NAME);

    match fs::write(&data_path, doc.to_string()) {
        Ok(_) => println!("✔️ Updated 'clock.toml'"),
        Err(e) => {
            println!("❌ Could not update 'clock.toml'");
            println!("{e}");
            process::exit(1);
        }
    }
}
