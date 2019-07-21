extern crate stem;
extern crate reqwest;
use std::collections::BTreeMap;

fn word_counts(s: String) -> BTreeMap<String, isize> {
    let mut counts: BTreeMap<String, isize> = BTreeMap::new();
    let words: Vec<&str> = s.split(" ").collect();

    for word in &words {
        if word.len() > 1 {
            let s = stem::get(word);
            match s {
               Ok(stemmed) => {
                   *counts.entry(stemmed.to_string()).or_insert(0) += 1;
               },
               Err(_e) => {},
            }
        }
    }
    return counts;
}

fn top_tfidf_words(tf: BTreeMap<String, isize>, df: BTreeMap<String, isize>, num_words: usize, min_document_use: i32, max_document_use: i32) ->  Vec<String> {

    let mut tfidf = BTreeMap::new();
    for (word, term_freq) in tf.iter() {
        let doc_freq = df[word] as i32;
        if doc_freq >= min_document_use && doc_freq <= max_document_use {
            let term_freq = *term_freq as f64;
            let doc_freq = doc_freq as f64;
            let v: f64 = term_freq/doc_freq;
            tfidf.insert(word.to_string(), v);
        }
    }

    let mut weights: Vec<f64> = tfidf.values().cloned().collect();
    weights.sort_by(|b, a| a.partial_cmp(b).unwrap());

    let mut prev: f64 = 0.0;
    let mut best_words : Vec<String> = Vec::new();
    for w in weights {
        if w == prev {
            continue
        }
        prev = w;
        for (word, weight) in tfidf.iter(){
            if *weight == w {
                best_words.push(word.to_string());
                if best_words.len() >= num_words {
                    return best_words;
                }
            }
        }
    }
    return best_words;

//    return words;
}

fn keywords(book: String, chapter: String, chapter_number: u32, min_document_use: i32, max_document_use: i32) {
    let tf = word_counts(chapter);
    let df = word_counts(book);
    let words = top_tfidf_words(tf, df, 10, min_document_use, max_document_use);
    println!("chapter {:?}: {}", chapter_number, words.join(", "));
}


fn main() {
    let url = "http://www.gutenberg.org/files/11/11-0.txt";
    let mut book = reqwest::get(url).expect("failed to make get request").text().expect("failed to get text").to_lowercase();
    for bad_char in ["\r", "\n", ",", ".", "!", ":"].iter() {
        book = book.replace(bad_char, " ")
    }
    let chapters: Vec<&str> = book.split("chapter").collect();

    let mut i = 0;
    for chapter in &chapters {
        keywords(book.to_string(), chapter.to_string(), i+1, 5, 100);
        i += 1;
    }
}