use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let path = "idioms.txt"; // 成语库文件路径
    let idioms = Arc::new(load_idioms(path));

    let idioms_for_check = idioms.clone();

    let handle = thread::spawn(move || {
        let mut score = 0;
        let mut max_consecutive = 0;
        let mut current_consecutive = 0;
        let mut try_error_count = 0;
        let mut last_char = None;
        let mut recent_idioms = VecDeque::new();
        let mut all_idioms = HashSet::new();
        let mut duplicate_penalty = 0.0;

        loop {
            let mut input = String::new();
            println!("请输入成语（输入exit退出）:");
            io::stdin().read_line(&mut input).expect("读取输入失败");
            let input = input.trim();
            if input == "exit" {
                let final_score = score as f64 + max_consecutive as f64 * 0.7 - try_error_count as f64 * 0.2 - duplicate_penalty;
                println!("游戏结束");
                println!("连续结对最长词语数: {}", max_consecutive);
                println!("试错成语数: {}", try_error_count);
                println!("重复成语扣分: {:.2}", duplicate_penalty);
                println!("最终得分: {:.2}", final_score);
                break;
            }

            if let Some(last) = last_char {
                if !input.starts_with(last) {
                    println!("成语需要以'{}'开始，请重新输入", last);
                    try_error_count += 1;
                    continue;
                }
            }

            if !idioms_for_check.lock().unwrap().contains(input) {
                println!("成语不存在或输入错误，请重新输入");
                try_error_count += 1;
                continue;
            }

            if recent_idioms.contains(&input.to_string()) || all_idioms.contains(&input.to_string()) {
                println!("相同成语不能在10个连续接龙中重复出现，或超过10个成语后重复出现将扣分，请重新输入");
                duplicate_penalty += 0.1;
                try_error_count += 1;
                continue;
            }

            score += 1;
            current_consecutive += 1;
            max_consecutive = max_consecutive.max(current_consecutive);
            if recent_idioms.len() == 10 {
                let _ = recent_idioms.pop_front();
            }
            recent_idioms.push_back(input.to_string());
            all_idioms.insert(input.to_string());

            last_char = input.chars().last();
            println!("接龙成功，请继续输入");
        }
    });

    handle.join().unwrap();
}

fn load_idioms(path: &str) -> Mutex<HashSet<String>> {
    let file = File::open(path).expect("无法打开文件");
    let reader = BufReader::new(file);
    let mut idioms = HashSet::new();
    for line in reader.lines() {
        if let Ok(idiom) = line {
            idioms.insert(idiom);
        }
    }
    Mutex::new(idioms)
}

