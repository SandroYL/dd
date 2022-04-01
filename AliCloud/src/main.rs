pub mod MakeDir;

fn main()
{
    let mut mkdir=MakeDir::mkdir::dirs::new();
    let mut indexer=MakeDir::findIndex::FilePaths::new();
    let mut input_path=String::new();
    let mut input_find=String::new();
    std::io::stdin().read_line(&mut input_path).unwrap();
    std::io::stdin().read_line(&mut input_find).unwrap();
    input_path.pop();input_path.pop();  //remove the \n and 
    input_find.pop();input_find.pop();
    mkdir.mkdir(input_path);
    match mkdir.serialize("data.txt".to_string())
    {
        Ok(str) => println!("{} is succeed!",&str),
        Err(_) => panic!("Something wrong happen when serialize"),
    }
    if let Err(_)=indexer.deserialize("data.txt".to_string())
    {
        panic!("Something wrong happen when deserialize");
    }
    if let Some(res)=indexer.find("main.rs".to_string())
    {
        for str in res
        {
            println!("{}",str+"\\"+&input_find);
        }
    }
    else{println!("Not Found!");}

}