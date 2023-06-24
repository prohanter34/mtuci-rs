use std::io::ErrorKind;

use search::{choosing_file, open_read_file, enter_request, search_in_text, quit_or_continue};



fn main() {

    loop {

        let file_text;
        let request;

        'gh: loop {
            let path = choosing_file();
            if let Result::Ok(path) = path {
                match open_read_file(path) {
                    Ok(text) => {
                        file_text = text.to_lowercase();
                        break 'gh;
                    },
                    Err(e) => {
                        if let ErrorKind::NotFound = e.kind() {
                            println!("Файл не найден");
                            continue 'gh;
                        } else {
                            panic!();
                        }
                    }

                }
            } else {
                println!("Ошибка ввода");
            }
        }

        if let Result::Ok(req) = enter_request() {
            request = req.to_lowercase();
        } else {
            println!("Ошибка ввода");
            panic!();
        }

        let answer = search_in_text(&request, file_text);

        for i in answer {
            println!("{}", i);
        }

        if quit_or_continue() {
            continue;
        } else {
            break;
        }

    }
    
}