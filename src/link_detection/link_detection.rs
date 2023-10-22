use std::collections::{HashMap, HashSet};

pub fn is_url_bad(char_map: &HashMap<char, char>, tokens: &HashSet<String>, url: &String) -> bool {
    let url = tokenize(char_map, url);
    let url_comps: Vec<&str> = url.split(" ").collect();
    let url_comps = &url_comps[0..url_comps.len() - 1];

    for tk in tokens {
        for &c in url_comps {
            // println!("Comparing '{c}' with '{tk}'");
            let d = levenshtein::levenshtein(tk, c);
            if d < 6 {
                // println!("'{c}' Matched '{tk}' with distance {d}");
                return true;
            }
        }
    }

    return false;
}

fn make_index(words: &Vec<String>, char_map: &HashMap<char, char>) -> HashSet<String> {
    let mut w_idx = HashSet::new();

    for w in words {
        let tk = tokenize(char_map, w);
        w_idx.insert(tk);
    }

    let mut two_tks = HashSet::<String>::new();
    for tk1 in &w_idx {
        for tk2 in &w_idx {
            let mut s = String::with_capacity(tk1.len() + tk2.len());
            s.push_str(tk1);
            s.push_str(tk2);
            two_tks.insert(s);
        }
    }
    for tk in w_idx {
        two_tks.insert(tk);
    }

    two_tks
}

fn tokenize(char_map: &HashMap<char, char>, w: &String) -> String {
    let mut new_w = String::new();
    let mut prev_c = '\0';

    for c in w.chars() {
        if char_map.contains_key(&c) && c != prev_c {
            new_w.push(*char_map.get(&c).unwrap());
        } else if c != prev_c {
            new_w.push(c);
        }
        prev_c = c;
    }

    new_w
}

macro_rules! char_map {
    ($($key:expr => $value:expr),*) => {
        {
            // can this be generalized?
            let mut map = HashMap::<char, char>::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

pub fn setup() -> (HashSet<std::string::String>, HashMap<char, char>) {
    let words = vec![
        "discord".to_string(),
        "steam".to_string(),
        "steamcommunity".to_string(),
        "steampowered".to_string(),
        "free".to_string(),
        "gift".to_string(),
        "cs".to_string(),
        "csgo".to_string(),
        "game".to_string(),
        "twitch".to_string(),
        "academy".to_string(),
        "reward".to_string(),
        "captcha".to_string(),
        "tech".to_string(),
        "account".to_string(),
        "nitro".to_string(),
        "pubg".to_string(),
        "coin".to_string(),
        "event".to_string(),
        "wallet".to_string(),
        "roblox".to_string(),
        "hypixel".to_string(),
    ];

    let char_map = char_map! {
        'a' => 'a',
        'o' => 'a',
        '0' => 'a',
        'e' => 'a',
        'i' => 'i',
        'l' => 'i',
        'j' => 'i',
        '1' => 'i',
        'u' => 'u',
        'v' => 'u',
        'w' => 'u',
        'm' => 'm',
        'n' => 'm',
        'd' => 'd',
        'b' => 'd',
        't' => 'r',
        '-' => ' ',
        '_' => ' ',
        '.' => ' ',
        '2' => ' ',
        '3' => ' ',
        '4' => ' ',
        '5' => ' ',
        '6' => ' ',
        '7' => ' ',
        '8' => ' ',
        '9' => ' '
    };

    return (make_index(&words, &char_map), char_map);
}
