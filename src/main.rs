use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::io::BufWriter;


//HTML、CSS、js匹配规则
fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let file = File::open(&args[i]).unwrap();
        let mut data = BufReader::new(file);
        let mut mini = String::new();
        let mut line = String::new();
        let mut buff = [ ' ', ' ', ' ' ];
        while data.read_line(&mut line).unwrap()>0 {
            for x in line.chars() {
                match x {
                    //删除“/*...*/”注释段
                    x if buff[0]=='/' && buff[1]=='*' => {
                        if x=='/' && buff[2]=='*' {
                            let temp = mini.pop();
                            if temp.is_some() {
                                buff[2]=temp.unwrap();
                                let temp = mini.pop();
                                if temp.is_some() {
                                    buff[1]=temp.unwrap();
                                }else{
                                    buff[1]=' ';
                                }
                            }else{
                                buff[2]=' ';
                                buff[1]=' ';
                            }
                            buff[0]=' ';
                        } else if x=='*' {
                            buff[2]='*';
                        } else {
                            buff[2]=' ';
                        }
                    },
                    //删除“<!--...-->”注释段
                    x if buff[0]=='<' && buff[1]=='!' => {
                        if x=='>' && buff[2]=='-' {
                            let temp = mini.pop();
                            if temp.is_some() {
                                buff[2]=temp.unwrap();
                                let temp = mini.pop();
                                if temp.is_some() {
                                    buff[1]=temp.unwrap();
                                }else{
                                    buff[1]=' ';
                                }
                            }else{
                                buff[2]=' ';
                                buff[1]=' ';
                            }
                            buff[0]=' ';
                        } else if x=='-' {
                            buff[2]='-';
                        } else {
                            buff[2]=' ';
                        }
                    },
                    //匹配“/*...*/”注释段
                    x if x=='*' && buff[2]=='/' => {
                        if buff[1]!=' ' {
                            mini.push(buff[1]);
                        }
                        buff[0]='/';
                        buff[1]='*';
                        buff[2]=' ';
                    },
                    //匹配“<!--...-->”注释段
                    x if x=='!' && buff[2]=='<' => {
                        if buff[1]!=' ' {
                            mini.push(buff[1]);
                        }
                        buff[0]='<';
                        buff[1]='!';
                        buff[2]=' ';
                    },
                    //删除以“//”开头的注释
                    x if x=='/' && buff[2]=='/' => {
                        buff[2]=buff[1];
                        buff[1]=buff[0];
                        buff[0]=' ';
                        mini.pop();
                        break;
                    },
                    //删除ASCII符号右边空白
                    x if x.is_whitespace() => {
                        if buff[2]!=' ' && ( (!buff[2].is_ascii_punctuation()) || buff[2]=='\'' || buff[2]=='"' ) {
                            if buff[0]!=' ' || buff[1]!=' ' {
                                buff[0]=buff[1];
                                mini.push(buff[0]);
                            }
                            buff[1]=buff[2];
                            buff[2]=' ';
                        }
                    },
                    //删除ASCII符号左边空白
                    x if x.is_ascii_punctuation() && x!='\'' && x!='"' && buff[2]==' ' => buff[2]=x,
                    //其余情况
                    _ => {
                        if buff[1]!=' ' || buff[0]!=' ' {
                            buff[0]=buff[1];
                            mini.push(buff[0]);
                        }
                        buff[1]=buff[2];
                        buff[2]=x;
                    }
                }
            }
            line = String::new();
        }
        if buff[1]!=' '{
            mini.push(buff[1]);
        }
        if buff[2]!=' '{
            mini.push(buff[2]);
        }
        println!("{}", mini);
    }
}
