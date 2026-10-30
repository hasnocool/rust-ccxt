pub fn score_text(text: &str) -> f64 {
    let pos_words = ["up","rise","gain","bull","buy","moon","breakout","record","high"];
    let neg_words = ["down","fall","loss","bear","sell","crash","dump","drop","hack","scam"];
    let words: Vec<String> = text.to_lowercase().split(|c: char| !c.is_alphabetic()).map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
    let pos = words.iter().filter(|w| pos_words.contains(&w.as_str())).count() as f64;
    let neg = words.iter().filter(|w| neg_words.contains(&w.as_str())).count() as f64;
    if pos + neg == 0.0 { 0.0 } else { (pos - neg) / (pos + neg) }
}
