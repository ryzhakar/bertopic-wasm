use leptos::prelude::*;
mod components;

use components::text_analyzer::TextAnalyzer;

fn main() {
    mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="p-4 max-w-3xl mx-auto">
            <h1 class="text-3xl font-bold mb-6">"BERTopic Explorer: Phase 1"</h1>
            <p class="mb-4">
                "Welcome to the first phase of our BERTopic implementation. Start by analyzing some text below."
            </p>
            <TextAnalyzer/>
        </div>
    }
}
