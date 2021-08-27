use colored::*;
use curl::easy::{Easy, List};

fn main() {
    let red = "██".red();
    let black = "██".black();
    let green = "██".green();
    let yellow = "██".yellow();
    let blue = "██".blue();
    let magenta = "██".magenta();
    let cyan = "██".cyan();
    let white = "██".red();
    for i in 0.. 5302 {
        let mut newnum = "".to_string();
        for x in 0..5-i.to_string().len() {
            newnum.push_str("0");
        }
        newnum.push_str(i.to_string().as_str());
        let mut dst = Vec::new();
        let mut easy = Easy::new();
        easy.url(vec!["https://github.com/AMTitan/RickRoller/blob/master/Images/terminal/", newnum.as_str(), ".jpg"].join("").as_str()).unwrap();

        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
        drop(transfer);

        println!("{}", i);
    }
}
