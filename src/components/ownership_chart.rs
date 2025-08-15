use leptos::*;
use crate::models::Company;
use crate::utils::{format_percentage, format_currency};

#[component]
pub fn ownership_chart(company: ReadSignal<Company>) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg p-6">
            <h2 class="text-2xl font-bold text-gray-800 mb-6">Ownership Timeline</h2>
            
            // Initial State
            <div class="mb-6">
                <h3 class="text-lg font-medium text-gray-700 mb-3">Initial State</h3>
                <div class="bg-green-50 border border-green-200 rounded-lg p-4">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div>
                            <span class="text-green-600 font-medium">Company Value:</span>
                            <span class="ml-2 text-lg font-bold">$0</span>
                        </div>
                        <div>
                            <span class="text-green-600 font-medium">Total Shares:</span>
                            <span class="ml-2 text-lg font-bold">{format!("{:,}", company.get().total_shares)}</span>
                        </div>
                        <div>
                            <span class="text-green-600 font-medium">Founder Ownership:</span>
                            <span class="ml-2 text-lg font-bold">100%</span>
                        </div>
                    </div>
                </div>
            </div>

            // Funding Rounds Timeline
            {move || {
                let rounds = company.get().funding_rounds.clone();
                if !rounds.is_empty() {
                    view! {
                        <div class="space-y-6">
                            <h3 class="text-lg font-medium text-gray-700">Funding Rounds</h3>
                            {rounds.iter().enumerate().map(|(index, round)| {
                                let post_money = round.valuation + round.amount;
                                let equity_percentage = (round.amount / post_money) * 100.0;
                                
                                view! {
                                    <div key=index class="border-l-4 border-blue-500 pl-6 relative">
                                        <div class="absolute -left-3 top-0 w-6 h-6 bg-blue-500 rounded-full flex items-center justify-center">
                                            <span class="text-white text-xs font-bold">{index + 1}</span>
                                        </div>
                                        
                                        <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
                                            <div class="flex items-center justify-between mb-3">
                                                <h4 class="text-lg font-semibold text-blue-800">
                                                    {round.name.clone()}
                                                </h4>
                                                <span class="text-sm text-blue-600 font-medium">
                                                    Round {index + 1}
                                                </span>
                                            </div>
                                            
                                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                                                <div>
                                                    <span class="text-blue-600 text-sm">Investment:</span>
                                                    <span class="ml-2 font-semibold">{format_currency(round.amount)}</span>
                                                </div>
                                                <div>
                                                    <span class="text-blue-600 text-sm">Pre-Money:</span>
                                                    <span class="ml-2 font-semibold">{format_currency(round.valuation)}</span>
                                                </div>
                                                <div>
                                                    <span class="text-blue-600 text-sm">Post-Money:</span>
                                                    <span class="ml-2 font-semibold">{format_currency(post_money)}</span>
                                                </div>
                                                <div>
                                                    <span class="text-blue-600 text-sm">Equity Sold:</span>
                                                    <span class="ml-2 font-semibold">{format_percentage(equity_percentage)}</span>
                                                </div>
                                            </div>
                                            
                                            // ESOP Allocation
                                            {if round.esop_allocation > 0.0 {
                                                view! {
                                                    <div class="mb-3 p-3 bg-green-100 border border-green-200 rounded">
                                                        <span class="text-green-700 text-sm">
                                                            ESOP Allocation: {format_percentage(round.esop_allocation)}
                                                        </span>
                                                    </div>
                                                }
                                            } else {
                                                view! { <div></div> }
                                            }}
                                            
                                            // Investors
                                            <div class="mb-3">
                                                <span class="text-blue-600 text-sm">Investors:</span>
                                                <div class="mt-1 flex flex-wrap gap-2">
                                                    {round.investors.iter().map(|investor| {
                                                        view! {
                                                            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                                                                {investor}
                                                            </span>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                            
                                            // Deal Type
                                            <div class="text-sm text-gray-600">
                                                Deal Type: <span class="font-medium">{match round.deal_type {
                                                    crate::models::DealType::Equity => "Equity",
                                                    crate::models::DealType::ConvertibleNote => "Convertible Note",
                                                    crate::models::DealType::SAFE => "SAFE",
                                                    crate::models::DealType::PreferredStock => "Preferred Stock",
                                                }}</span>
                                                {if round.anti_dilution {
                                                    view! {
                                                        <span class="ml-2 text-orange-600"> Anti-dilution</span>
                                                    }
                                                } else {
                                                    view! { <div></div> }
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }
                } else {
                    view! {
                        <div class="text-center py-8 text-gray-500">
                            <div class="text-4xl mb-4"></div>
                            <p class="text-lg">No funding rounds added yet</p>
                            <p class="text-sm">Add your first funding round to see the ownership timeline</p>
                        </div>
                    }
                }
            }}

            // Current State Summary
            {move || {
                let rounds = company.get().funding_rounds.clone();
                if !rounds.is_empty() {
                    let current_valuation = rounds.last().map(|r| r.valuation + r.amount).unwrap_or(0.0);
                    let total_esop = company.get().esop_pool.total_allocation;
                    let founder_ownership: f64 = company.get().founders.iter().map(|f| f.current_ownership).sum();
                    
                    view! {
                        <div class="mt-8">
                            <h3 class="text-lg font-medium text-gray-700 mb-3">Current State</h3>
                            <div class="bg-purple-50 border border-purple-200 rounded-lg p-4">
                                <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                                    <div>
                                        <span class="text-purple-600 font-medium">Current Value:</span>
                                        <span class="ml-2 text-lg font-bold">{format_currency(current_valuation)}</span>
                                    </div>
                                    <div>
                                        <span class="text-purple-600 font-medium">Founder Ownership:</span>
                                        <span class="ml-2 text-lg font-bold">{format_percentage(founder_ownership)}</span>
                                    </div>
                                    <div>
                                        <span class="text-purple-600 font-medium">ESOP Pool:</span>
                                        <span class="ml-2 text-lg font-bold">{format_percentage(total_esop)}</span>
                                    </div>
                                    <div>
                                        <span class="text-purple-600 font-medium">Investor Ownership:</span>
                                        <span class="ml-2 text-lg font-bold">{format_percentage(100.0 - founder_ownership - total_esop)}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}

            // Key Insights
            {move || {
                let rounds = company.get().funding_rounds.clone();
                if rounds.len() > 1 {
                    let total_dilution: f64 = company.get().founders.iter()
                        .map(|f| f.initial_ownership - f.current_ownership)
                        .sum();
                    let avg_round_size: f64 = rounds.iter().map(|r| r.amount).sum::<f64>() / rounds.len() as f64;
                    
                    view! {
                        <div class="mt-6">
                            <h3 class="text-lg font-medium text-gray-700 mb-3">Key Insights</h3>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
                                    <div class="text-yellow-800">
                                        <div class="font-medium mb-1">Total Founder Dilution</div>
                                        <div class="text-2xl font-bold">{format_percentage(total_dilution)}</div>
                                        <div class="text-sm">Average per round: {format_percentage(total_dilution / rounds.len() as f64)}</div>
                                    </div>
                                </div>
                                <div class="bg-indigo-50 border border-indigo-200 rounded-lg p-4">
                                    <div class="text-indigo-800">
                                        <div class="font-medium mb-1">Average Round Size</div>
                                        <div class="text-2xl font-bold">{format_currency(avg_round_size)}</div>
                                        <div class="text-sm">Total raised: {format_currency(rounds.iter().map(|r| r.amount).sum::<f64>())}</div>
                                    </div>
                                </div>
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


