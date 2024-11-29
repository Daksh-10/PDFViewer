use lopdf::Document;
use std::fs;

fn read_pdf(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load(file_path)?;
    for (page_num, page) in doc.get_pages().iter() {
        let content = doc.extract_text(&[(*page_num)])?;
        println!("Page {}:\n{}", page_num, content);
    }
    Ok(())
}


fn transfer_file(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::copy(src, dest)?;
    println!("File transferred from {} to {}", src, dest);
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "abc.pdf"; //Input file name
    let dest = "./transferred/abc.pdf"; //This is the destination
    transfer_file(file_path, dest)?;
    read_pdf(file_path)
}
