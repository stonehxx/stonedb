use super::command::Command;
use super::path::PathType;
use super::command::Value;

pub fn read_line(input: impl AsRef<str>) -> Result<Command, String> {
    let input = input.as_ref();
    let input = equal_split(input);
    match input.len() {
        0 => Ok(Command::None),
        1 => {
            let input = &input[0];
            if input.is_empty() {
                return Ok(Command::None);
            }
            read_get(input)
        }
        2 => {
            let (path, put) = (&input[0], &input[1]);
            if path.is_empty() && put.is_empty() {
                return Err(format!("??? = ???"));
            }
            if path.is_empty() {
                return Err(format!("??? = {put}"));
            }
            if put.is_empty() {
                return Err(format!("{path} = ???"));
            }
            read_put(path, put)
        }
        _ => Err(format!("too many =")),
    }
}

fn read_get(path: impl AsRef<str>) -> Result<Command, String> {
    match read_path(path) {
        Ok(data) => Ok(Command::Get(data)),
        Err(e) => Err(e),
    }
}

fn read_put(path: impl AsRef<str>, put: impl AsRef<str>) -> Result<Command, String> {
    let path = match read_path(path) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };
    let data = match read_data(put) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };
    Ok(Command::Set(path, data))
}

fn read_path(path: impl AsRef<str>) -> Result<Vec<PathType>, String> {
    let mut result = Vec::new();
    let mut chars: Vec<char> = path.as_ref().chars().collect();
    let mut only = true;
    for i in 0..chars.len() {
        if chars[i].is_ascii_punctuation() {
            let database_name: String = chars[0..i].iter().collect();
            result.push(PathType::Database(database_name));
            chars = chars[i..].to_vec();
            only = false;
            break;
        }
    }
    if only {
        return Ok(vec![PathType::Database(chars.iter().collect())]);
    }
    loop {
        if chars.len() == 0 {
            println!("1");
            break;
        }
        match chars[0] {
            '[' => {
                let (data, end) = match bounds_split(chars, '[', ']') {
                    Some(data) => data,
                    None => return Err(format!("[ has no end")),
                };
                chars = end;
                if data.iter().all(|x| x.is_ascii_digit()) {
                    result.push(PathType::Num(data.iter().collect()));
                } else {
                    result.push(PathType::Field(data.iter().collect()));
                }
            }
            '(' => {
                let (data, end) = match bounds_split(chars, '(', ')') {
                    Some(data) => data,
                    None => return Err(format!("( has no end")),
                };
                chars = end;
                result.push(PathType::Fn(data.iter().collect()));
            }
            '{' => {
                let (data, end) = match bounds_split(chars, '{', '}') {
                    Some(data) => data,
                    None => return Err(format!("{{ has no end")),
                };
                chars = end;
                result.push(PathType::Object(data.iter().collect()));
            }
            '.' => {
                if chars.len() > 0 {
                    chars = chars[1..].to_vec()
                } else {
                    return Err(format!(".???"));
                }
                let mut only = true;
                let mut content = String::new();
                for i in 0..chars.len() {
                    if chars[i].is_ascii_punctuation() {
                        content = chars[0..i].iter().collect();
                        chars = chars[i..].to_vec();
                        only = false;
                        break;
                    }
                }
                if only {
                    if chars.iter().all(|x| x.is_ascii_digit()) {
                        result.push(PathType::Num(chars.iter().collect()));
                    } else {
                        result.push(PathType::Field(chars.iter().collect()));
                    }
                    return Ok(result);
                }
                match chars[0] {
                    '(' => {
                        let (data, end) = match bounds_split(chars, '(', ')') {
                            Some(data) => data,
                            None => return Err(format!("( has no end")),
                        };
                        chars = end;
                        result.push(PathType::Call(content, data.iter().collect()));
                    }
                    _ => return Err(format!("unknow {}", chars[0])),
                }
            }
            _ => return Err(format!("unknow {}", chars[0])),
        }
    }
    Ok(result)
}

fn read_data(data: impl AsRef<str>) -> Result<Value, String> {
    let data = data.as_ref();

    Ok(Value::PathType(read_path(data)?))
}

fn read_new(data: &str) {}

fn read_input() {}

fn bounds_split(input: Vec<char>, start: char, end: char) -> Option<(Vec<char>, Vec<char>)> {
    let mut depth = 0;
    for i in 0..input.len() {
        if input[i] == start {
            depth += 1
        } else if input[i] == end {
            depth -= 1;
            if depth == 0 {
                if i + 1 == input.len() {
                    return Some((input[1..i].to_vec(), Vec::new()));
                } else {
                    return Some((input[1..i].to_vec(), input[i + 1..].to_vec()));
                }
            }
        }
    }
    None
}

fn equal_split(input: impl AsRef<str>) -> Vec<String> {
    let chars: Vec<char> = input.as_ref().chars().collect();
    let mut result = Vec::new();
    let mut start = 0;
    for i in 0..chars.len() {
        if chars[i] == '=' {
            let left_not_equal = i == 0 || chars[i - 1] != '=';
            let right_not_equal = i == chars.len() - 1 || chars[i + 1] != '=';
            if left_not_equal && right_not_equal {
                let left: String = chars[start..i].iter().collect();
                result.push(left.trim().to_string());
                start = i + 1;
            }
        }
    }
    if start < chars.len() {
        let last: String = chars[start..].iter().collect();
        let last = last.trim().to_string();
        result.push(last);
    } else {
        result.push(String::new());
    }
    result
}
