mod MakeDir;





fn main()
{
    let mut mk=MakeDir::mkdir::dirs::new();
    let path="E:\\7Zip\\7-Zip".to_string();
    mk.mkdir(path);
    mk.print();
    match mk.serialize("datas.txt".to_string()) 
    {
        Ok(filename)=>println!("{}",filename),
        Err(_)=>panic!("Error happen"),    
    }
}

