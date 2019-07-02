use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Write, BufWriter};

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let (name, suff) = args[i].split_at(args[i].rfind('.').unwrap());
        let mut file = BufReader::new(File::open(&args[i]).unwrap());
        let mut base = String::new();
        let mut mini = String::new();
        if file.read_to_string(&mut base).is_ok() {
            match suff {
                //CSS文件
                _ if suff == ".css" => mini.push_str(&reduce_css(&base)),
                _ if suff == ".html" => {
                    /* mode值说明：
                     * “1”表示在“'...'”内
                     * “2”表示在“"..."”内
                     * “3”表示在“<!--...-->”中
                     * “4”表示在“<...>”内，判断中
                     * “5”表示在“<...>”内，完成中
                     * “6”表示在“<style>...</style>”内
                     * “7”表示在“<script>...</script>”内
                     */
                    let mut mode = 0;
                    let mut data = String::new();
                    let mut tags = String::new();
                    let mut last = ' ';
                    let mut olds = 0;
                    for face in base.chars() {
                        if mode == 1 {
                            if last != '\\' && face == '\'' {
                                mode = olds;
                            }
                            data.push(face);
                            last = face;
                        } else if mode == 2 {
                            if last != '\\' && face == '"' {
                                mode = olds;
                            }
                            data.push(face);
                            last = face;
                        } else if mode == 3 {
                            tags.push(face);
                            if tags == "-->" {
                                mode = 0;
                                tags = String::new();
                            } else if tags != "-" && tags != "--" {
                                tags = String::new();
                                tags.push(face);
                            }
                        } else if tags == "<!--" {
                            mode = 3;
                            tags = String::new();
                            tags.push(face);
                        } else if face == '\'' {
                            olds = mode;
                            mode = 1;
                            data.push(face);
                            last = face;
                        } else if face == '"' {
                            olds = mode;
                            mode = 2;
                            data.push(face);
                            last = face;
                        } else if mode == 6 {
                            tags.push(face);
                            if tags == "</style>" {
                                mode = 0;
                                if data != "" {
                                    mini.push_str(&reduce_css(&data));
                                    data = String::new();
                                }
                                data.push_str(&tags);
                                tags = String::new();
                            } else if tags != "<" && tags != "</" && tags != "</s" &&
                                       tags != "</st" &&
                                       tags != "</sty" &&
                                       tags != "</styl" &&
                                       tags != "</style"
                            {
                                data.push_str(&tags);
                                tags = String::new();
                            }
                        } else if mode == 7 {
                            tags.push(face);
                            if tags == "</script>" {
                                mode = 0;
                                if data != "" {
                                    mini.push_str(&reduce_js(&data));
                                    data = String::new();
                                }
                                data.push_str(&tags);
                                tags = String::new();
                            } else if tags != "<" && tags != "</" && tags != "</s" &&
                                       tags != "</sc" &&
                                       tags != "</scr" &&
                                       tags != "</scri" &&
                                       tags != "</scrip" &&
                                       tags != "</script"
                            {
                                data.push_str(&tags);
                                tags = String::new();
                            }
                        } else if face == '<' {
                            mode = 4;
                            data.push_str(&tags);
                            tags = String::new();
                            tags.push(face);
                        } else if mode == 4 {
                            if face == ' ' || face == '/' {
                                mode = 5;
                                data.push_str(&tags);
                                data.push(face);
                            } else if face == '>' {
                                data.push_str(&tags);
                                data.push(face);
                                if tags == "<style" {
                                    mode = 6;
                                    mini.push_str(&reduce_html(&data));
                                    data = String::new();
                                } else if tags == "<script" {
                                    mode = 7;
                                    mini.push_str(&reduce_html(&data));
                                    data = String::new();
                                } else {
                                    mode = 0;
                                }
                                tags = String::new();
                            } else {
                                tags.push(face);
                            }
                        } else if mode == 5 {
                            data.push(face);
                            if face == '>' {
                                if tags == "<style" {
                                    mode = 6;
                                    mini.push_str(&reduce_html(&data));
                                    data = String::new();
                                } else if tags == "<script" {
                                    mode = 7;
                                    mini.push_str(&reduce_html(&data));
                                    data = String::new();
                                } else {
                                    mode = 0;
                                }
                                tags = String::new();
                            }
                        } else {
                            data.push(face);
                        }
                    }
                    mini.push_str(&reduce_html(&data));
                }
                _ if suff == ".js" => mini.push_str(&reduce_js(&base)),
                _ => {}
            }
            let minifile = File::create(format!("{}{}{}", name, ".mini", suff))
                .expect("Unable to create file");
            let mut minifile = BufWriter::new(minifile);
            minifile.write_all(mini.as_bytes()).expect(
                "Unable to write data",
            );
        }
    }
}
//CSS代码压缩
fn reduce_css(data: &str) -> String {
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
    mini
}
//HTML代码压缩
fn reduce_html(data: &str) -> String {
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
    mini
}
//JS代码压缩
fn reduce_js(data: &str) -> String {
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
    mini
}
