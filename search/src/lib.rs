use std::{io::{self, Error, Read}, fs::File};




pub fn choosing_file() -> Result<String, Error> {
    println!("Ведите путь к файлу");
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim().to_string();
    return Ok(path);
}

pub fn open_read_file(path: String) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut file_text = String::new();
    f.read_to_string(&mut file_text)?;
    return Ok(file_text);
}

pub fn enter_request() -> Result<String, Error> {
    println!("Введите тоо что хотите найти");
    let mut request = String::new();
    io::stdin().read_line(&mut request)?;
    let request = request.trim().to_string();
    return Ok(request);
}

pub fn search_in_text(arg: &String, text: String) -> Vec<String> {
    let mut answer = Vec::with_capacity(5);
    for line in text.lines() {
        if line.contains(arg) {
            answer.push(line.to_string());
        }
    }
    return answer;
}

pub fn quit_or_continue() -> bool {
    loop {
        println!("Продолжить? (y/n)");
        let mut yes_no = String::new();
        if let Result::Ok(_) = io::stdin().read_line(&mut yes_no) {
            yes_no = yes_no.trim().to_string();
            if yes_no == "y".to_string() {
                return true;
            } else if yes_no == "n".to_string() {
                return false;
            } else {
                continue;
            }
        }
    }
}
