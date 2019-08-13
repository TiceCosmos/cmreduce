// WEB代码压缩
// pub fn web(data: &str) -> Result<String, &str> {
// }
// CSS代码压缩
pub fn css(data: &str) -> Result<String, &str> {
    /* mode值说明：
     * “true”表示在“/*...*/”内
     */
    let mut mode = false;
    let mut mini = String::new();
    let mut last = ' ';
    for face in data.chars() {
        if mode {
            if last == '*' && face == '/' {
                mode = false;
                last = ' ';
            } else {
                last = face;
            }
        } else if last == '/' && face == '*' {
            mode = true;
            last = ' ';
        } else if face.is_whitespace() {
            if (!last.is_ascii_punctuation()) && last != ' ' {
                mini.push(last);
                last = ' ';
            }
        } else {
            if (!face.is_ascii_punctuation()) || face == '#' || face == '.' || last != ' ' {
                mini.push(last);
            }
            last = face;
        }
    }
    if last != ' ' {
        mini.push(last);
    }
    if mini.chars().next() == Some(' ') {
        mini.remove(0);
    }
    Ok(mini)
}

// HTML代码压缩
pub fn html(data: &str) -> Result<String, &str> {
    /* mode值说明：
     * “1”表示在“'...'”内
     * “2”表示在“"..."”内
     */
    let mut mode = 0;
    let mut mini = String::new();
    let mut last = ' ';
    for face in data.chars() {
        if mode == 1 {
            if last != '\\' && face == '\'' {
                mode = 0;
            }
            mini.push(last);
            last = face;
        } else if mode == 2 {
            if last != '\\' && face == '"' {
                mode = 0;
            }
            mini.push(last);
            last = face;
        } else if face == '\'' {
            mode = 1;
            if last != ' ' {
                mini.push(last);
            }
            last = face;
        } else if face == '"' {
            mode = 2;
            if last != ' ' {
                mini.push(last);
            }
            last = face;
        } else if face.is_whitespace() {
            if ((!last.is_ascii_punctuation()) || last == '\'' || last == '"') && last != ' ' {
                mini.push(last);
                last = ' ';
            }
        } else {
            if (!face.is_ascii_punctuation()) || face == '\'' || face == '"' || last != ' ' {
                mini.push(last);
            }
            last = face;
        }
    }
    if last != ' ' {
        mini.push(last);
    }
    if mini.chars().next() == Some(' ') {
        mini.remove(0);
    }
    Ok(mini)
}

// JS代码压缩
pub fn js(data: &str) -> Result<String, &str> {
    /* mode值说明：
     * “1”表示在“'...'”内
     * “2”表示在“"..."”内
     * “3”表示在“/*...*/”内
     * “4”表示在“//...\n”内
     */
    let mut mode = 0;
    let mut mini = String::new();
    let mut last = ' ';
    for face in data.chars() {
        if mode == 1 {
            if last != '\\' && face == '\'' {
                mode = 0;
            }
            mini.push(last);
            last = face;
        } else if mode == 2 {
            if last != '\\' && face == '"' {
                mode = 0;
            }
            mini.push(last);
            last = face;
        } else if mode == 3 {
            if face == '\n' {
                mode = 0;
                let temp = mini.pop();
                if temp.is_some() {
                    last = temp.unwrap();
                } else {
                    last = ' ';
                }
            }
        } else if mode == 4 {
            if last == '*' && face == '/' {
                mode = 0;
                let temp = mini.pop();
                if temp.is_some() {
                    last = temp.unwrap();
                } else {
                    last = ' ';
                }
            } else {
                last = face;
            }
        } else if face == '\'' {
            mode = 1;
            if last != ' ' {
                mini.push(last);
            }
            last = face;
        } else if face == '"' {
            mode = 2;
            if last != ' ' {
                mini.push(last);
            }
            last = face;
        } else if last == '/' && face == '/' {
            mode = 3;
        } else if last == '/' && face == '*' {
            mode = 4;
        } else if face.is_whitespace() {
            if (!last.is_ascii_punctuation()) && last != ' ' {
                mini.push(last);
                last = ' ';
            }
        } else {
            if (!face.is_ascii_punctuation()) || last != ' ' {
                mini.push(last);
            }
            last = face;
        }
    }
    if last != ' ' {
        mini.push(last);
    }
    if mini.chars().next() == Some(' ') {
        mini.remove(0);
    }
    Ok(mini)
}
