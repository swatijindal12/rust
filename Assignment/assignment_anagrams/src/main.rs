fn is_anagram(target: &str, candidate: &str) -> bool {
    let mut target_chars: Vec<char> = target.chars().collect();
    let mut candidate_chars: Vec<char> = candidate.chars().collect();

    target_chars.sort_unstable();
    candidate_chars.sort_unstable();

    target_chars == candidate_chars && target.to_lowercase() != candidate.to_lowercase()
}


fn find_anagram<'a>(target_word: &str, candidates_word: &'a [&'a str]) -> Vec<&'a str> {
    let mut anagrams: Vec<&str> = Vec::new();

    for &candidate in candidates_word {
        if is_anagram(target_word, candidate) {
            anagrams.push(candidate);
        }
    }

    anagrams
}

fn main() {

    let target_word = "stone";
    let candidate_word = ["stone", "tones", "tons", "banana", "SeTon", "noset"];

    let anagram_word = find_anagram(target_word, &candidate_word);
    println!("anagram word is: {:?}", anagram_word);
}
