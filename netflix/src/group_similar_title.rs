// First, we need to figure out a way to individually group all the character combinations of each title.
// Suppose the content library contains the following titles: "duel", "dule", "speed", "spede", "deul", "cars".
// How would you efficiently implement a functionality so that if a user misspells speed as spede,
// they are shown the correct title?
// We want to split the list of titles into sets of words so that all words in a set are anagrams.
// In the above list, there are three sets: {"duel", "dule", "deul"}, {"speed", "spede"}, and {"cars"}.
// Search results should comprise all members of the set that the search string is found in.
// We should pre-compute these sets instead of forming them when the user searches a title.

use std::collections::HashMap;

pub fn group_similar_title(titles: Vec<String>) -> Vec<Vec<String>> {
    let mut dictionary: HashMap<String, Vec<String>> = HashMap::new();
    let mut groups: Vec<Vec<String>> = Vec::new();

    for word in titles.into_iter() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        let sorted_word = String::from_iter(chars);

        dictionary
            .entry(sorted_word)
            .or_insert(Vec::new())
            .push(word.to_string());
    }

    for (_, words) in &dictionary {
        groups.push(words.to_owned());
    }

    println!("Similar titles are grouped, {groups:?}");

    groups
}

pub fn driver() {
    let words = vec![
        "duel".to_string(),
        "dule".to_string(),
        "speed".to_string(),
        "spede".to_string(),
        "deul".to_string(),
        "cars".to_string(),
    ];
    group_similar_title(words);
}
