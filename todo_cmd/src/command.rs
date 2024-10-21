use crate::database::{DataBase, Record};
pub fn add(db: &mut DataBase, content: Option<String>) -> Result<(), &'static str>{
    if let Some(content) = content {
        let id = DataBase::read_all_records(db).last().map(|r|r.id+1).unwrap_or(1);
        let record = Record{id, content: content.clone()};
        let _ = db.add_record(&record);
        println!("item added:{}", content);
        Ok(())
    } else {
        Err("content is err!")
    }
}

pub fn list(db: &mut DataBase) -> Result<(), &'static str> {
    let records = DataBase::read_all_records(db);
    for r in records.into_iter() {
        println!("{},{}", r.id, r.content);
    }
    Ok(())
}

pub fn remove(db: &mut DataBase, id: Option<String>) -> Result<(), &'static str> {
    if let Some(id) = id {
        
        match id.parse::<i32>(){
            Ok(id) => {
                match db.remove_recode(id) {
                    Ok(line) => {
                        println!("item delete succeed:{}", line);
                        Ok(())
                    },
                    _ => Err("this id is not exist!"),
                }
                
            },
            Err(e) => {
                println!("{}", e);
                Err( "parse error")
                //return Err( &errors[..]);
            },
        }   
    } else {
        Err("miss parameters on remove command\n")
    }
}