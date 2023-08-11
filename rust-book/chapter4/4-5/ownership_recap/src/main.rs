fn main() {
    let words = vec!["Hello".to_string()];
    let d = new_document(words);

    let words_copy = get_word(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    assert!(!get_word(&d).contains(&"world".into()));

    let mut a_num = 0;
    inner(&mut a_num);

    let s = String::from("abcdefg");
    let _s_slice = &s[2..5];
}

type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_word(this: &Document) -> &[String] {
    this.as_slice()
}

fn inner(x: &mut i32) {
    let another_num = 1;
    let _a_stack_ref = &another_num;

    let a_box = Box::new(2);
    let _a_box_stack_ref = &a_box;
    let _a_box_heap_ref = &*a_box;

    *x += 5;
}
