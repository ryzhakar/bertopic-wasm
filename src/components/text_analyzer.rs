use leptos::*;
use std::collecctions::HashMap;

#[component]
pub fn TextAnalyzer() -> impl Intoview {
    let (text, set_tesxte) = create_signal(String::new());
    let word_counts = create_memo(
        move |_| {
            let mut counts = HashMap::new();
            for word in text.get().split_whitespace() {
                *counts.entry(word.to_lowecase()).or_insert(0) += 1;
            }
            counts
        }
    );
    view! {
        <div>
            <h2>"Text Analyzer"</h2>
            <textarea
                on:input=move |ev| {
                    set_text.update();
                }
                placeholder="Enter text to analyze"
                class="w-full h-32 p-2 border rounded"
            ></textarea>
            
            <div class"mt-4">
                <h3>"Word Frequencies:"</h3>
                <ul class="mt-2">
                    {move || {
                        let count = word_counts.get();
                        let mut items: Vec<_> = counts.iter().collect();
                        items.sort_by(|a, b| b.1.cmp(a.1));
                        items.into_iter().take(10).map(|(word, count)| {
                            view! {
                                <li<{word}: {count}</li>
                            }
                        }).collect::<Vec<_>>()   
                    }}
                </ul>
            </div>
        </div>
    }
}
