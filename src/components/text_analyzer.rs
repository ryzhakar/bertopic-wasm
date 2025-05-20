use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn TextAnalyzer() -> impl IntoView {
    let (text_reader, text_writer) = signal(String::new());
    let word_counts = Memo::new(
        move |_| {
            let mut counts = HashMap::new();
            for word in text_reader.get().split_whitespace() {
                *counts.entry(word.to_lowercase()).or_insert(0) += 1;
            }
            counts
        }
    );
    view! {
        <div>
            <h2>"Text Analyzer"</h2>
            <textarea
                on:input=move |event| {
                    text_writer.set(event_target_value(&event))
                }
                placeholder="Enter text to analyze"
                class="w-full h-32 p-2 border rounded"
            ></textarea>

            <div class="mt-4">
                <h3>"Word Frequencies:"</h3>
                <ul class="mt-2">
                    {move || {
                        let all_counts = word_counts.get();
                        let mut items: Vec<_> = all_counts.iter().map(|(word, count)| {
                            (word.clone(), *count)
                        }).collect();
                        items.sort_by(|a, b| b.1.cmp(&a.1));
                        items.into_iter().take(10).map(
                            move |(word, count)| {
                                view! {
                                    <li>{word}: {count}</li>
                                }
                            }
                        ).collect::<Vec<_>>()   
                    }}
                </ul>
            </div>
        </div>
    }
}
