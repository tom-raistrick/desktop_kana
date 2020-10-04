use std::io;
use std::io::prelude::*;
use std::str;
use std::vec::Vec;
use std::fs;
use std::thread::sleep;
use std::time;
use termion::clear;
use text_io::read;
use regex::Regex;
use rand::Rng;

// Function called to clear terminal screen
fn cls() {
    println!("{}", clear::All);
}

// Function called to pull selected characters from kana lists, along with their romanisations
fn select_chars<'a>(selection: &str, sc_kana: &Vec<&'a str>, sc_romanised: &Vec<&'a str>)
                    -> (Vec<&'a str>, Vec<&'a str>, String, bool) {

    let mut input_valid = true;

    let (mut char_list, mut romanised_char_list) = (vec![], vec![]);

    let mut select_vector: Vec<String>
        = selection.split(",").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    select_vector.sort_unstable();
    select_vector.dedup();

    let unique_columns = select_vector.join(", ");

    for column in &select_vector {
        if column.parse::<usize>().unwrap() < 29 && column.parse::<usize>().unwrap() > 0 {
            char_list.extend(sc_kana[(column.parse::<usize>().unwrap()) - 1].split(","));
            romanised_char_list.extend(sc_romanised[(column.parse::<usize>().unwrap()) - 1]
                .split(","));
        }
        else {
            input_valid = false;
        }
    }

    (char_list, romanised_char_list, unique_columns, input_valid)
}


fn main() {

    let (mut h_chars_valid, mut k_chars_valid) = (false, false);

    let mut input_valid;

    let (mut h_chars, mut romanised_h_chars, mut k_chars, mut romanised_k_chars)
        : (Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>) = (vec![], vec![], vec![], vec![]);

    let mut select_char_result: (Vec<&str>, Vec<&str>, String, bool);

    let (mut h_unique_columns, mut k_unique_columns): (String, String) = (String::from(""), String::from(""));

    let hiragana = vec![
        "あ,い,う,え,お",
        "か,き,く,け,こ",
        "さ,し,す,せ,そ",
        "た,ち,つ,て,と",
        "な,に,ぬ,ね,の",
        "は,ひ,ふ,へ,ほ",
        "ま,み,む,め,も",
        "や,ゆ,よ",
        "ら,り,る,れ,ろ",
        "わ,を",
        "ん",
        "が,ぎ,ぐ,げ,ご",
        "ざ,じ,ず,ぜ,ぞ",
        "だ,ぢ,づ,で,ど",
        "ば,び,ぶ,べ,ぼ",
        "ぱ,ぴ,ぷ,ぺ,ぽ",
        "きゃ,きゅ,きょ",
        "しゃ,しゅ,しょ",
        "ちゃ,ちゅ,ちょ",
        "にゃ,にゅ,にょ",
        "ひゃ,ひゅ,ひょ",
        "みゃ,みゅ,みょ",
        "りゃ,りゅ,りょ",
        "ぎゃ,ぎゅ,ぎょ",
        "じゃ,じゅ,じょ",
        "ぢゃ,ぢゅ,ぢょ",
        "びゃ,びゅ,びょ",
        "ぴゃ,ぴゅ,ぴょ"
    ];

    let katakana = vec![
        "ア,イ,ウ,エ,オ",
        "カ,キ,ク,ケ,コ",
        "サ,シ,ス,セ,ソ",
        "タ,チ,ツ,テ,ト",
        "ナ,ニ,ヌ,ネ,ノ",
        "ハ,ヒ,フ,ヘ,ホ",
        "マ,ミ,ム,メ,モ",
        "ヤ,ユ,ヨ",
        "ラ,リ,ル,レ,ロ",
        "ワ,ヲ",
        "ン",
        "ガ,ギ,グ,ゲ,ゴ",
        "ザ,ジ,ズ,ゼ,ゾ",
        "ダ,ヂ,ヅ,デ,ド",
        "バ,ビ,ブ,ベ,ボ",
        "パ,ピ,プ,ペ,ポ",
        "キャ,キュ,キョ",
        "シャ,シュ,ショ",
        "チャ,チュ,チョ",
        "ニャ,ニュ,ニョ",
        "ヒャ,ヒュ,ヒョ",
        "ミャ,ミュ,ミョ",
        "リャ,リュ,リョ",
        "ギャ,ギュ,ギョ",
        "ジャ,ジュ,ジョ",
        "ヂャ,ヂュ,ヂョ",
        "ビャ,ビュ,ビョ",
        "ピャ,ピュ,ピョ"
    ];

    let romanised = vec![
        "a,i,u,e,o",
        "ka,ki,ku,ke,ko",
        "sa,shi,su,se,so",
        "ta,chi,tsu,te,to",
        "na,ni,nu,ne,no",
        "ha,hi,fu,he,ho",
        "ma,mi,mu,me,mo",
        "ya,yu,yo",
        "ra,ri,ru,re,ro",
        "wa,wo",
        "n",
        "ga,gi,gu,ge,go",
        "za,ji,zu,ze,zo",
        "da,ji;dji;jyi;di,dzu;zu;du,de,do",
        "ba,bi,bu,be,bo",
        "pa,pi,pu,pe,po",
        "kya,kyu,kyo",
        "sha,shu,sho",
        "cha,chu,cho",
        "nya,nyu,nyo",
        "hya,hyu,hyo",
        "mya,myu,myo",
        "rya,ryu,ryo",
        "gya,gyu,gyo",
        "ja,ju,jo",
        "ja,ju,jo",
        "bya,byu,byo",
        "pya,pyu,pyo"
    ];

    let (mut h_selection, mut k_selection): (String, String) = (String::from(""), String::from(""));

    while h_chars_valid == false || k_chars_valid == false {
        cls();
        while h_chars_valid == false {
            cls();
            println!("Printing hiragana list.");

            let hiragana_full = fs::read_to_string("resources/hiragana_full");

            for line in hiragana_full {
                println!("{}", line);
            }

            println!("Select hiragana characters to test by typing below.  Enter column numbers \
            separated by commas (e.g. '1,2,4,10,26') to\nselect the kana in those columns.  You \
            can also type 'all' or 'none' to make your selection.\n\n");
            print!("Selection: ");
            io::stdout().flush().unwrap();
            h_selection = read!("{}\n");
            h_selection.retain(|c| !c.is_whitespace());

            if h_selection == "all" {

                for element in &hiragana {
                    for char in element.split(",") {
                        h_chars.push(char);
                    }
                }

                for element in &romanised {
                    for char in element.split(",") {
                        romanised_h_chars.push(char);
                    }
                }

                println!("Selected all hiragana characters.");
                h_chars_valid = true;
            }

            else if h_selection == "none" {
                h_chars_valid = true;
            }

            else {

                let re = Regex::new("^[0-9,]*$").unwrap();

                if re.is_match(&h_selection) {

                    select_char_result = select_chars(&h_selection, &hiragana, &romanised);

                    input_valid = select_char_result.3;

                    if input_valid == true {
                        h_chars = select_char_result.0;
                        romanised_h_chars = select_char_result.1;
                        h_unique_columns = select_char_result.2;
                        h_chars_valid = true;
                    }

                    else {
                        println!("\nYou entered a number outside the accepted range.  Press Enter to try again.");

                        let _pause: String = read!("{}\n");

                    }
                }

                else {
                    println!("\nInvalid Selection.  Press Enter to try again.");

                    let _pause: String = read!("{}\n");
                }
            }
        }

        while k_chars_valid == false {
            cls();
            println!("Printing katakana list.");

            let katakana_full = fs::read_to_string("resources/katakana_full");

            for line in katakana_full {
                println!("{}", line);
            }

            println!("Select katakana characters to test by typing below.  Enter column numbers \
            separated by commas (e.g. '1,2,4,10,26') to\nselect the kana in those columns.  You \
            can also type 'all' or 'none' to make your selection.\n\n");
            print!("Selection: ");
            io::stdout().flush().unwrap();
            k_selection = read!("{}\n");
            k_selection.retain(|c| !c.is_whitespace());

            if k_selection == "all" {

                for element in &katakana {
                    for char in element.split(",") {
                        k_chars.push(char);
                    }
                }

                for element in &romanised {
                    for char in element.split(",") {
                        romanised_k_chars.push(char);
                    }
                }

                println!("Selected all katakana characters.");
                k_chars_valid = true;
            }

            else if k_selection == "none" {
                k_chars_valid = true;
            }

            else {

                let re = Regex::new("^[0-9,]*$").unwrap();

                if re.is_match(&k_selection) {


                    let select_char_result = select_chars(&k_selection, &katakana, &romanised);

                    input_valid = select_char_result.3;

                    if input_valid == true {
                        k_chars = select_char_result.0;
                        romanised_k_chars = select_char_result.1;
                        k_unique_columns = select_char_result.2;
                        k_chars_valid = true;
                    }

                    else {
                        println!("\nYou entered a number outside the accepted range.  Press Enter to try again.");

                        let _pause: String = read!("{}\n");

                    }

                }

                else {
                    println!("\nInvalid Selection.  Press Enter to try again.");

                    let _pause: String = read!("{}\n");

                }
            }
        }
        if h_unique_columns == "" && k_unique_columns == ""
            && h_selection != "all" && k_selection != "all" {
            h_chars_valid = false;
            k_chars_valid = false;
            println!("\nNo kana selected.  Restarting.\n");
            sleep(time::Duration::from_millis(2000));
            continue;
        }
    }

    cls();

    if h_unique_columns != "" {
        println!("Selected hiragana characters in the following column{}: {}.", if h_unique_columns.len() > 1 {"s"} else {""}, h_unique_columns);
    }

    if k_unique_columns != "" {
        println!("Selected katakana characters in the following column{}: {}.", if k_unique_columns.len() > 1 {"s"} else {""}, k_unique_columns);
    }

    if k_unique_columns != "" || h_unique_columns != "" {
        println!("\n");
    }

    let (run, mut new_prompt, mut vowel) = (true, true, false);

    let (mut kana_num, mut previous_num): (usize, isize) = (0, -1);

    let (mut prompt_num, mut correct_num) = (0, 0);

    let mut kana_list: Vec<&str> = vec![];

    kana_list.extend(h_chars);
    kana_list.extend(k_chars);

    let mut romanised_list: Vec<&str> = vec![];

    romanised_list.extend(romanised_h_chars);
    romanised_list.extend(romanised_k_chars);

    while run {

        let mut rng = rand::thread_rng();

        if new_prompt {
            kana_num = rng.gen_range(0, kana_list.len());

            if kana_num == previous_num as usize {
                continue;
            }
        }

        if prompt_num > 0 {
            println!("{} of {} answers correct ({}%)",
                     correct_num, prompt_num, (correct_num as f32/prompt_num as f32)*100.0);
        }
        println!("Prompt {}:", prompt_num + 1);
        println!("Kana: {}", kana_list[kana_num]);
        print!("Rōmaji: ");
        io::stdout().flush().unwrap();

        if "aiueo".contains(romanised_list[kana_num]) {
            vowel = true;
        }

        let response: String = read!("{}\n");

        if response == "skip" {
            println!("Answer{}: '{}'.\n",
                     if romanised_list[kana_num].contains(";") {"s"} else {""},
                     romanised_list[kana_num].replace(";", "', '"));
            new_prompt = true;
        }

        else if vowel == false && romanised_list[kana_num].contains(&response) {
            println!("Correct!\n");
            correct_num += 1;
            new_prompt = true;
        }

        else if vowel == true && response == romanised_list[kana_num] {
            println!("Correct!\n");
            correct_num += 1;
            new_prompt = true;
        }

        else {
            println!("Incorrect.  Please try again.\n");
            new_prompt = false;
        }

        prompt_num += 1;

        previous_num = kana_num as isize;
    }
}