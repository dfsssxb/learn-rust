use std::fs::{File, OpenOptions};
use std::io::{Seek, Read};
use std::io::Write;
// 实现数据的增删查

pub struct Record {
    pub id: i32,
    pub content: String,
}

impl Record {
    pub fn parse(line: &str) -> Self {
        let fields: Vec<&str> = line.split(",").collect();
        if fields.len() == 1 {
            // 空行，或者不满足"id,content"格式
            Record {
                id: 0,
                content: "".to_string(),
            }
        } else {
            let id = fields[0].parse::<i32>().unwrap();
            let content = fields[1..].join(",");
            Record {
                id,
                content,
            }
        }
        
    }
}
pub struct DataBase {
    pub file: File,
}


impl DataBase {
    pub fn open(filename: &str) -> Self {
        // 文件应该能同时读和写, 标准库中的File::Open, Create 分别能只读和只写
        let file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(filename)
                .unwrap();
        DataBase {file}
    }

    pub fn add_record(&mut self, record: &Record) -> Result<(), &'static str> {
        let line = format!("\n{},{}", record.id, record.content);
        if let Err(_) = write!(self.file, "{}", line) {
            return Err("追加记录失败");
            //return Err("{e:?}");
        }
        Ok(())
    }

    pub fn remove_recode(&mut self, id: i32) -> Result<String, &'static str> {
        // 读取数据，并找到对应的行
        let mut buf  = String::new();    
        let _ = self.file.read_to_string(&mut buf);
        let lines = buf.split("\n").collect::<Vec<_>>();

        match lines.clone().into_iter()
                    .enumerate()
                    .find(|(_, line)| {
                        let r = Record::parse(line);
                        r.id == id
        }) {
            Some((i, line)) => {
                // 删除数据
                let new_contents = lines.into_iter()
                                                .enumerate()
                                                .filter(|(j, _)| *j != i)
                                                .map(|(_, line)|line)
                                                .filter(|line| !line.is_empty())
                                                .collect::<Vec<_>>()
                                                .join("\n");
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();               // seek到文件开头，相当于重新写入
                self.file.write_all(new_contents.as_bytes()).unwrap();                              // 写内容
                self.file.set_len(new_contents.len() as u64).unwrap();          // 文件结束


                Ok(String::from(line))    
            },
            _ => {Err("can't find this id") },
        }
    }

    pub fn read_all_records(&mut self) -> Vec<Record> {
        self.file.seek(std::io::SeekFrom::Start(0)).unwrap();               // seek到文件开头
        let mut buf  = String::new();    
        let _ = self.file.read_to_string(&mut buf);
        buf.clone().split("\n").collect::<Vec<&str>>()
                    .into_iter()
                    .filter(|line| !line.is_empty())
                    .map(|line|Record::parse(line))
                    .collect()
    }
}
