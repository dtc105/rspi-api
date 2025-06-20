use std::collections::HashSet;

pub fn similarity(string1: &String, string2: &String) -> f32 {
    let string1 = string1.to_lowercase();
    let string2 = string2.to_lowercase();

    if string1 == string2 {
        return 1.0;
    }

    let mut iter1 = string1.chars();
    let mut iter2 = string2.chars();

    let mut bigrams1: HashSet<String> = HashSet::new();
    let mut bigrams2: HashSet<String> = HashSet::new();

    let mut last_char1 = match iter1.next() {
        Some(char) => char,
        None => return 0f32,
    };

    let mut last_char2 = match iter2.next() {
        Some(char) => char,
        None => return 0f32,
    };

    let mut count: usize = 0;

    while let Some(curr_char) = iter1.next() {
        let inserted = bigrams1.insert(last_char1.to_string() + &curr_char.to_string());
        last_char1 = curr_char;
        if inserted {
            count += 1;
        }
    }

    let mut intersection: usize = 0;
    while let Some(curr_char) = iter2.next() {
        let bigram = last_char2.to_string() + &curr_char.to_string();
        let is_seen = bigrams1.contains(&bigram);
        let inserted = bigrams2.insert(bigram);

        if is_seen {
            intersection += 1;
        }

        last_char2 = curr_char;

        if inserted {
            count += 1;
        }
    }

    return 2f32 * (intersection as f32) / (count as f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similarity_test() {
        let result1 = similarity(&String::from("Hello"), &String::from("World"));
        let result2 = similarity(&String::from("Aditya Godbole"), &String::from("Kodi"));
        let result3 = similarity(&String::from("Hello"), &String::from("hello"));
        let result4 = similarity(&String::from("Onomatopoeia"), &String::from("Onomanapoeia"));
        assert_eq!(result1, 0.0);
        assert_eq!(result2, 0.25);
        assert_eq!(result3, 1.0);
        assert_eq!(result4, 8.0 / 11.0);
    }
}
