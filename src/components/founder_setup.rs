use leptos::*;
use crate::models::{Company, Founder};

#[component]
pub fn founder_setup(
    company: ReadSignal<Company>,
    set_company: WriteSignal<Company>,
) -> impl IntoView {
    let add_founder = create_action(move |name: &String| {
        let mut new_company = company.get();
        let new_founder = Founder {
            name: name.clone(),
            initial_ownership: 0.0,
            current_ownership: 0.0,
            shares: 0,
        };
        new_company.founders.push(new_founder);
        set_company.set(new_company);
    });

    let remove_founder = create_action(move |index: &usize| {
        let mut new_company = company.get();
        new_company.founders.remove(*index);
        set_company.set(new_company);
    });

    let update_founder_name = create_action(move |(index, name): &(usize, String)| {
        let mut new_company = company.get();
        if let Some(founder) = new_company.founders.get_mut(*index) {
            founder.name = name.clone();
            set_company.set(new_company);
        }
    });

    let update_founder_ownership = create_action(move |(index, ownership): &(usize, f64)| {
        let mut new_company = company.get();
        if let Some(founder) = new_company.founders.get_mut(*index) {
            founder.initial_ownership = *ownership;
            founder.current_ownership = *ownership;
            founder.shares = (new_company.total_shares as f64 * *ownership / 100.0) as u64;
            set_company.set(new_company);
        }
    });

    let (new_founder_name, set_new_founder_name) = create_signal("".to_string());

    view! {
        <div class="bg-white rounded-lg shadow-lg p-6">
            <h2 class="text-2xl font-bold text-gray-800 mb-6">Founder Setup</h2>
            
            // Add New Founder
            <div class="mb-6">
                <div class="flex space-x-2">
                    <input
                        type="text"
                        placeholder="Founder Name"
                        class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=new_founder_name
                        on:change=move |ev| {
                            set_new_founder_name.set(event_target_value(&ev));
                        }
                    />
                    <button
                        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        on:click=move |_| {
                            if !new_founder_name.get().is_empty() {
                                add_founder.dispatch(new_founder_name.get());
                                set_new_founder_name.set("".to_string());
                            }
                        }
                    >
                        Add Founder
                    </button>
                </div>
            </div>

            // Founders List
            <div class="space-y-4">
                {move || {
                    company.get().founders.iter().enumerate().map(|(index, founder)| {
                        view! {
                            <div key=index class="bg-gray-50 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-3">
                                    <input
                                        type="text"
                                        class="text-lg font-medium bg-transparent border-none focus:outline-none focus:ring-2 focus:ring-blue-500 rounded px-2"
                                        value={founder.name.clone()}
                                        on:change=move |ev| {
                                            let name = event_target_value(&ev);
                                            update_founder_name.dispatch((index, name));
                                        }
                                    />
                                    <button
                                        class="text-red-600 hover:text-red-800 text-sm"
                                        on:click=move |_| remove_founder.dispatch(index)
                                    >
                                        Remove
                                    </button>
                                </div>
                                
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            Initial Ownership
                                        </label>
                                        <div class="flex items-center space-x-2">
                                            <input
                                                type="number"
                                                step="0.1"
                                                min="0"
                                                max="100"
                                                class="w-20 px-2 py-1 border border-gray-300 rounded text-sm"
                                                value={founder.initial_ownership.to_string()}
                                                on:change=move |ev| {
                                                    let ownership = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                                    update_founder_ownership.dispatch((index, ownership));
                                                }
                                            />
                                            <span class="text-sm text-gray-500">%</span>
                                        </div>
                                    </div>
                                    
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-1">
                                            Current Ownership
                                        </label>
                                        <div class="text-lg font-semibold text-blue-600">
                                            {format!("{:.1}%", founder.current_ownership)}
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="mt-3 text-sm text-gray-600">
                                    Shares: {format!("{:,}", founder.shares)}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            // Total Ownership Warning
            {move || {
                let total_ownership: f64 = company.get().founders.iter().map(|f| f.initial_ownership).sum();
                if (total_ownership - 100.0).abs() > 0.1 {
                    view! {
                        <div class="mt-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                            <div class="text-yellow-800 text-sm">
                                Total founder ownership: {format!("{:.1}%", total_ownership)}
                                {if total_ownership > 100.0 { " (exceeds 100%)" } else { " (under 100%)" }}
                            </div>
                        </div>
                    }
                } else {
                    view! {
                        <div class="mt-4 p-3 bg-green-50 border border-green-200 rounded-lg">
                            <div class="text-green-800 text-sm">
                                Total founder ownership: {format!("{:.1}%", total_ownership)}
                            </div>
                        </div>
                    }
                }
            }}
        </div>
    }
}


