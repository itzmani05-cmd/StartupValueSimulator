use leptos::*;
use crate::models::Company;
use crate::utils::{format_percentage, format_shares, format_currency};

#[component]
pub fn CapTable(
    cap_table: ReadSignal<Vec<(String, f64, u64)>>,
    company: ReadSignal<Company>,
) -> impl IntoView {
    let total_ownership: f64 = cap_table.get().iter().map(|(_, ownership, _)| ownership).sum();
    let total_shares = company.get().total_shares;

    view! {
        <div class="bg-white rounded-lg shadow-lg p-6">
            <h2 class="text-2xl font-bold text-gray-800 mb-6">Cap Table</h2>
            
            // Company Summary
            <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <span class="text-blue-600 font-medium">Total Shares:</span>
                        <span class="ml-2 text-lg font-bold">{format_shares(total_shares)}</span>
                    </div>
                    <div>
                        <span class="text-blue-600 font-medium">Total Ownership:</span>
                        <span class="ml-2 text-lg font-bold">{format_percentage(total_ownership)}</span>
                    </div>
                </div>
            </div>

            // Cap Table
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Stakeholder
                            </th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Ownership
                            </th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Shares
                            </th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Value at $10M
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        {move || {
                            cap_table.get().iter().map(|(name, ownership, shares)| {
                                let value_at_10m = (*ownership / 100.0) * 10_000_000.0;
                                let is_founder = company.get().founders.iter().any(|f| f.name == *name);
                                
                                view! {
                                    <tr class={if is_founder { "bg-blue-50" } else { "bg-white" }}>
                                        <td class="px-6 py-4 whitespace-nowrap">
                                            <div class="flex items-center">
                                                <div class="text-sm font-medium text-gray-900">
                                                    {name.clone()}
                                                </div>
                                                {if is_founder {
                                                    view! {
                                                        <span class="ml-2 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                                            Founder
                                                        </span>
                                                    }
                                                } else {
                                                    view! { <div></div> }
                                                }}
                                            </div>
                                        </td>
                                        <td class="px-6 py-4 whitespace-nowrap">
                                            <div class="text-sm text-gray-900 font-medium">
                                                {format_percentage(*ownership)}
                                            </div>
                                        </td>
                                        <td class="px-6 py-4 whitespace-nowrap">
                                            <div class="text-sm text-gray-900">
                                                {format_shares(*shares)}
                                            </div>
                                        </td>
                                        <td class="px-6 py-4 whitespace-nowrap">
                                            <div class="text-sm text-gray-900">
                                                {format_currency(value_at_10m)}
                                            </div>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </tbody>
                </table>
            </div>

            // Ownership Distribution
            {move || {
                let cap_table_data = cap_table.get();
                if !cap_table_data.is_empty() {
                    view! {
                        <div class="mt-6">
                            <h3 class="text-lg font-medium text-gray-800 mb-3">Ownership Distribution</h3>
                            <div class="space-y-2">
                                {cap_table_data.iter().map(|(name, ownership, _)| {
                                    let width = format!("{:.1}%", ownership);
                                    let color_class = if company.get().founders.iter().any(|f| f.name == *name) {
                                        "bg-blue-500"
                                    } else if name == "ESOP Pool" {
                                        "bg-green-500"
                                    } else {
                                        "bg-gray-500"
                                    };
                                    
                                    view! {
                                        <div class="flex items-center space-x-3">
                                            <div class="w-24 text-sm text-gray-600 truncate">
                                                {name.clone()}
                                            </div>
                                            <div class="flex-1 bg-gray-200 rounded-full h-4">
                                                <div
                                                    class={format!("h-4 rounded-full {}", color_class)}
                                                    style={format!("width: {}", width)}
                                                ></div>
                                            </div>
                                            <div class="w-16 text-sm text-gray-900 font-medium text-right">
                                                {format_percentage(*ownership)}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}

            // Dilution Impact
            {move || {
                let founders = company.get().founders.clone();
                if !founders.is_empty() {
                    view! {
                        <div class="mt-6">
                            <h3 class="text-lg font-medium text-gray-800 mb-3">Founder Dilution Impact</h3>
                            <div class="space-y-3">
                                {founders.iter().map(|founder| {
                                    let dilution = founder.initial_ownership - founder.current_ownership;
                                    let dilution_color = if dilution > 10.0 { "text-red-600" } 
                                                       else if dilution > 5.0 { "text-orange-600" } 
                                                       else { "text-green-600" };
                                    
                                    view! {
                                        <div class="flex justify-between items-center bg-gray-50 px-4 py-3 rounded-lg">
                                            <span class="text-sm font-medium text-gray-700">
                                                {founder.name.clone()}
                                            </span>
                                            <div class="text-right">
                                                <div class="text-sm text-gray-600">
                                                    {format_percentage(founder.initial_ownership)} → {format_percentage(founder.current_ownership)}
                                                </div>
                                                <div class={format!("text-sm font-medium {}", dilution_color)}>
                                                    {if dilution > 0.0 {
                                                        format!("-{:.1}%", dilution)
                                                    } else {
                                                        "No dilution".to_string()
                                                    }}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}


