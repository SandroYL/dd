use std::{ io::{self, Read}, collections::HashMap, fs::File, thread::panicking};

pub struct FilePaths
{
    file_paths: HashMap<String, Vec<String>>,
}

impl FilePaths
{
    pub fn new() -> Self
    {
        FilePaths
        {
            file_paths: HashMap::new(),
        }
    }
    pub fn deserialize(&mut self,file:String) ->Result<(),io::Error>
    {
        let mut file=File::open(file)?;
        let mut contents=String::new();
        file.read_to_string(&mut contents)?;
        let contents=contents.split("\n").collect::<Vec<&str>>();
        for line in contents
        {
            if line.len()<=1
            {break;}
            let mut line_vec=line.split("->").collect::<Vec<&str>>();
            let val=line_vec.pop().unwrap().trim_matches(',').split(",").collect::<Vec<&str>>(); //remove the end of ','
            let key=line_vec.pop().unwrap();

            
            for str in val
            {
                self.file_paths.entry(key.to_string()).or_insert(Vec::new()).push(str.to_string());
            }
        }

        Ok(())
    }

    pub fn find(&self,input:String)->Option<Vec<String>>
    {
        if self.file_paths.is_empty()
        {
            return None;
        }
        if let Some(rets)=self.file_paths.get(&input)
        {
            return Some(rets.clone());
        }
        else{return None;}

    }
}
