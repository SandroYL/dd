use std::collections::HashMap;
use std::io::{self, Write};
use std::{
    collections::VecDeque,
    fs::{self, File},
};

pub struct dirs {
    file_filepath: HashMap<String, Vec<String>>, //every file with its filepath
}

impl dirs {
    pub fn new() -> Self {
        dirs {
            file_filepath: HashMap::new(),
        }
    }

    pub fn mkdir(&mut self,path: String) 
    {
        let mut problems: VecDeque<String> = VecDeque::new();
        problems.push_back(path);
        while !problems.is_empty() 
        {
            let curpath = problems.pop_back().unwrap();
            let insider = match fs::read_dir(curpath) 
            {
                Ok(ins) => ins,
                Err(_) => continue,
            };
            for sonpath in insider 
            {
                if let Ok(intopath) = sonpath 
                {
                    let filepath = intopath.path().to_str().unwrap().to_string();
                    //get the last part of the filepath and the other part of the filepath
                    let mut filepath_vec: Vec<&str> = filepath.split("\\").collect();
                    let file_name = filepath_vec.pop().unwrap().to_string();
                    //check file_name is a file or a directory
                    if let Ok(file_type) = fs::metadata(filepath.clone()) 
                    {
                        if file_type.is_dir() 
                        {
                            problems.push_back(filepath);
                        }
                        else 
                        {
                            self.file_filepath.entry(file_name).or_insert(Vec::new()).push(filepath_vec.join("\\"));
                        }
                    }
                }
            }
        }
    }

    //print the file_filepath
    pub fn print(&self) 
    {
        for (key, value) in &self.file_filepath 
        {
            print!("key:{}", key);
            for v in value 
            {
                println!("\tvalue:{}", v);
            }
        }
    }

    //serialize the file_filepath to a file
     //return filename if Ok
    //else return Error
    pub fn serialize(&self,filepath:String)->Result<String,io::Error>
    {
        let mut file=match File::create(filepath.clone()) {
            Ok(file)=>file,
            Err(e)=>return Err(e),
        };
        for (key,val) in self.file_filepath.iter()
        {
            let mut input_str=String::new();
            input_str.push_str(key);
            input_str.push_str("->");
            for files in val
            {
                input_str.push_str(files);
                input_str.push_str(",");
            }
            input_str.push_str("\n");
            file.write_all(input_str.as_bytes())?;
            file.flush()?;
        }

        Ok(filepath)
    }

}
