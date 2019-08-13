use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{BufWriter, Write};

pub mod reduce;

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let (name, suff) = args[i].split_at(args[i].rfind('.').unwrap());
        let mut file = BufReader::new(File::open(&args[i]).unwrap());
        let mut base = String::new();
        let mut mini = String::new();
        if file.read_to_string(&mut base).is_ok() {
            match suff {
                _ if suff == ".css" => mini.push_str(&reduce::css(&base).unwrap()),
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
                                    mini.push_str(&reduce::css(&data).unwrap());
                                    data = String::new();
                                }
                                data.push_str(&tags);
                                tags = String::new();
                            } else if tags != "<"
                                && tags != "</"
                                && tags != "</s"
                                && tags != "</st"
                                && tags != "</sty"
                                && tags != "</styl"
                                && tags != "</style"
                            {
                                data.push_str(&tags);
                                tags = String::new();
                            }
                        } else if mode == 7 {
                            tags.push(face);
                            if tags == "</script>" {
                                mode = 0;
                                if data != "" {
                                    mini.push_str(&reduce::js(&data).unwrap());
                                    data = String::new();
                                }
                                data.push_str(&tags);
                                tags = String::new();
                            } else if tags != "<"
                                && tags != "</"
                                && tags != "</s"
                                && tags != "</sc"
                                && tags != "</scr"
                                && tags != "</scri"
                                && tags != "</scrip"
                                && tags != "</script"
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
                                    mini.push_str(&reduce::html(&data).unwrap());
                                    data = String::new();
                                } else if tags == "<script" {
                                    mode = 7;
                                    mini.push_str(&reduce::html(&data).unwrap());
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
                                    mini.push_str(&reduce::html(&data).unwrap());
                                    data = String::new();
                                } else if tags == "<script" {
                                    mode = 7;
                                    mini.push_str(&reduce::html(&data).unwrap());
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
                    mini.push_str(&reduce::html(&data).unwrap());
                }
                _ if suff == ".js" => mini.push_str(&reduce::js(&base).unwrap()),
                _ => {}
            }
            let minifile = File::create(format!("{}{}{}", name, ".mini", suff))
                .expect("Unable to create file");
            let mut minifile = BufWriter::new(minifile);
            minifile
                .write_all(mini.as_bytes())
                .expect("Unable to write data");
        }
    }
}
