use std::{io::stdin, fs, path::Path, env::{home_dir, current_dir}};

struct Arguments {
    path: String,
    extension: String
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    while input == "\n" {
        input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input != "\n" {
            break;
        } else {
            println!("Harap masukkan ekstensi!");
            continue;
        }
    }

    let mut input = input.trim().split_whitespace();
    let p = input.next().unwrap();

    let mut args = Arguments {
        path: String::from(current_dir().unwrap().display().to_string()),
        extension: String::from(p)
    };

    if !p.contains("/") {
        args.extension = String::from(p);
    } else {
        args.path = String::from(p);
        args.extension = String::from(input.next().unwrap_or_else(|| {
            println!("Tidak dapat membaca ekstensi."); 
            "-"
        }));
    }

    if Path::new(format!("{}/{}", home_dir().unwrap().display(), args.path).as_str()).is_dir() {
        let mut entries = fs::read_dir(format!("{}/{}", home_dir().unwrap().display(), args.path)).unwrap().flat_map(|res| res.map(|d| d.path())).collect::<Vec<_>>();
        entries.sort_by_key(|f| f.file_name().unwrap().to_ascii_lowercase());
        entries.sort_by_key(|f| f.extension().unwrap_or_default().to_str().unwrap_or_default() != args.extension.as_str());

        for i in entries {
            println!("{}", i.display());
        }
    } else {
        let mut entries = fs::read_dir(current_dir().unwrap()).unwrap().flat_map(|res| res.map(|d| d.path())).collect::<Vec<_>>();
        entries.sort_by_key(|f| f.file_name().unwrap().to_ascii_lowercase());
        entries.sort_by_key(|f| f.extension().unwrap_or_default().to_str().unwrap_or_default() != args.extension.as_str());

        for i in entries {
            println!("{}", i.display());
        }
    }
}
