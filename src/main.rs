use dialoguer::{Input, Select};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::Path;

fn main() {
    println!("Searching files...");

    loop {
        println!("実行したい操作を教えて下さい。");
        let menu = &["シフト作成", "設定"];
        let choice = Select::new().items(menu).default(0).interact().unwrap();

        match choice {
            0 => {
                println!("シフト作成ですね。");
                break;
            }
            1 =>{
                println!("設定ですね。");
                break;
            }
            _ => {
                println!("エラー");
            }
        }
    }
}
