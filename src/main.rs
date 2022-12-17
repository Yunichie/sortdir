use std::{io::stdin, fs, path::Path, env::{home_dir, current_dir}};

struct Arguments {
    path: String,
    extension: String
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // While input is empty (not the best way, I know. I just want to keep it simple).
    while input == "\n" {
        println!("Harap masukkan ekstensi!");
        input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input != "\n" {
            break;
        } else {
            continue;
        }
    }

    let mut input = input.trim().split_whitespace();
    let p = input.next().unwrap();                      // next() consumes an element from `input`.

    let mut args = Arguments {
        path: String::from(current_dir().unwrap().display().to_string()),   // If no (valid) directory provided, default to current directory.
        extension: String::from(p)                                          // Input is given immediately as extension if no directory input is provided.
    };

    // Check whether or not the input looks like a valid directory.
    if !p.contains("/") {
        args.extension = String::from(p);
    } else {
        args.path = String::from(p);
        args.extension = String::from(input.next().unwrap_or_else(|| {
            // If None, `args.extension` will default to "-".
            // Doesn't really matter if it is not a valid extension as it will not be read/executed.
            println!("Tidak dapat membaca ekstensi."); 
            "-"
        }));
    }

    // Check whether or not `args.path` is a valid directory.
    if Path::new(format!("{}/{}", home_dir().unwrap().display(), args.path).as_str()).is_dir() {
        // Collect all the files and directories in `home_dir/args.path`
        // Sort by alphabetical order.
        // Sort by extension.
        let mut entries = fs::read_dir(format!("{}/{}", home_dir().unwrap().display(), args.path)).unwrap().flat_map(|res| res.map(|d| d.path())).collect::<Vec<_>>();
        entries.sort_by_key(|f| f.file_name().unwrap().to_ascii_lowercase());
        entries.sort_by_key(|f| f.extension().unwrap_or_default().to_str().unwrap_or_default() != args.extension.as_str());

        // Print the sorted directory to screen.
        for i in entries {
            println!("{}", i.display());
        }
    } else {
        // Collect all the files and directories in current directory.
        // Sort by alphabetical order.
        // Sort by extension.
        let mut entries = fs::read_dir(current_dir().unwrap()).unwrap().flat_map(|res| res.map(|d| d.path())).collect::<Vec<_>>();
        entries.sort_by_key(|f| f.file_name().unwrap().to_ascii_lowercase());
        entries.sort_by_key(|f| f.extension().unwrap_or_default().to_str().unwrap_or_default() != args.extension.as_str());

        // Print the sorted directory to screen.
        for i in entries {
            println!("{}", i.display());
        }
    }
}
