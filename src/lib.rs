use std::collections::HashMap;

pub mod competition;
pub mod search;

//哈希表，滑动窗口
//lc:3
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left =0;
    let mut max = 0;
    let mut map:HashMap<char, usize> = HashMap::new();
    for (index, value) in s.chars().enumerate() {
        if map.contains_key(&value) {
            left = left.max(map[&value] + 1);
        }
        map.insert(value, index);
        max = max.max(index - left + 1);
    }
    max as i32
}

//lc:30
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let m = words[0].len();
    let n = words.len();
    let wind_width = m * n;
    let mut res:Vec<i32> = vec![];
    let mut map1: HashMap<&str,usize> = HashMap::new();
    let mut map2: HashMap<&str,usize> = HashMap::new();
    if s.len() < wind_width {
        return res;
    }
    for word in words.iter() {
        *map1.entry(word).or_insert(0) += 1;
    }
    for i in (0..=s.len() - wind_width).step_by(m) {
        for j in 0..n {
            let word = &s[(i+j*m)..(i+(j+1)*m)];
            *map2.entry(word).or_insert(0) += 1;
        }
        if map1 == map2 {
            res.push(i as i32);
        }

        map2.clear();
    }


    res

}

pub fn find_substring1(s: String, words: Vec<String>) -> Vec<i32> {
    let word_len = words[0].len();
    let word_num = words.len();
    let mut word_set = HashMap::<&str, usize>::new();
    words.iter().for_each(|wd| *word_set.entry(wd).or_insert(0) += 1);
    let mut result = Vec::new();

    for offset in 0..word_len {
        let mut cover = HashMap::new();
        let mut ready = 0;

        let mut i = offset;
        let mut j = offset;
        while j + word_len <= s.len() {
            let wd = &s[j..(j + word_len)];
            j += word_len;
            match word_set.get(wd) {
                Some(target) => {
                    *cover.entry(wd).or_insert(0) += 1;
                    ready += 1;
                    while *cover.get(wd).unwrap() > *target {
                        *cover.get_mut(&s[i..(i + word_len)]).unwrap() -= 1;
                        ready -= 1;
                        i += word_len;
                    }

                    if ready == word_num {
                        result.push(i as i32);
                        *cover.get_mut(&s[i..(i + word_len)]).unwrap() -= 1;
                        ready -= 1;
                        i += word_len;
                    }
                },
                None => {
                    cover.clear();
                    ready = 0;
                    i = j;
                },
            }
        }
    }
    result
}

