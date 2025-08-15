use leptos::*;
use crate::models::{Company, FundingRound, DealType};

#[component]
pub fn FundingRounds(
    current_round: ReadSignal<FundingRound>,
    set_current_round: WriteSignal<FundingRound>,
    add_round: Action<FundingRound, Result<(), ()>>,
    company: ReadSignal<Company>,
) -> impl IntoView {
    let update_round_name = create_action(move |name: &String| {
        let mut new_round = current_round.get();
        new_round.name = name.clone();
        set_current_round.set(new_round);
    });

    let update_round_amount = create_action(move |amount: &f64| {
        let mut new_round = current_round.get();
        new_round.amount = *amount;
        set_current_round.set(new_round);
    });

    let update_round_valuation = create_action(move |valuation: &f64| {
        let mut new_round = current_round.get();
        new_round.valuation = *valuation;
        set_current_round.set(new_round);
    });

    let update_equity_sold = create_action(move |equity: &f64| {
        let mut new_round = current_round.get();
        new_round.equity_sold = *equity;
        set_current_round.set(new_round);
    });

    let update_deal_type = create_action(move |deal_type: &DealType| {
        let mut new_round = current_round.get();
        new_round.deal_type = deal_type.clone();
        set_current_round.set(new_round);
    });

    let update_esop_allocation = create_action(move |esop: &f64| {
        let mut new_round = current_round.get();
        new_round.esop_allocation = *esop;
        set_current_round.set(new_round);
    });

    let update_anti_dilution = create_action(move |anti_dilution: &bool| {
        let mut new_round = current_round.get();
        new_round.anti_dilution = *anti_dilution;
        set_current_round.set(new_round);
    });

    let (new_investor, set_new_investor) = create_signal("".to_string());

    let add_investor = create_action(move |investor: &String| {
        let mut new_round = current_round.get();
        new_round.investors.push(investor.clone());
        set_current_round.set(new_round);
    });

    let remove_investor = create_action(move |index: &usize| {
        let mut new_round = current_round.get();
        new_round.investors.remove(*index);
        set_current_round.set(new_round);
    });

    // Calculate post-money valuation and equity percentage
    let post_money_valuation = create_memo(move |_| {
        let round = current_round.get();
        round.valuation + round.amount
    });

    let equity_percentage = create_memo(move |_| {
        let round = current_round.get();
        (round.amount / post_money_valuation.get()) * 100.0
    });

    view! {
        <div class="bg-white rounded-lg shadow-lg p-6">
            <h2 class="text-2xl font-bold text-gray-800 mb-6">Funding Round</h2>
            
            // Round Details
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Round Name
                    </label>
                    <input
                        type="text"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={move || current_round.get().name}
                        on:change=move |ev| {
                            let name = event_target_value(&ev);
                            update_round_name.dispatch(name);
                        }
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Deal Type
                    </label>
                    <select
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={move || match current_round.get().deal_type {
                            DealType::Equity => "Equity",
                            DealType::ConvertibleNote => "ConvertibleNote",
                            DealType::SAFE => "SAFE",
                            DealType::PreferredStock => "PreferredStock",
                        }}
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            let deal_type = match value.as_str() {
                                "Equity" => DealType::Equity,
                                "ConvertibleNote" => DealType::ConvertibleNote,
                                "SAFE" => DealType::SAFE,
                                "PreferredStock" => DealType::PreferredStock,
                                _ => DealType::Equity,
                            };
                            update_deal_type.dispatch(deal_type);
                        }
                    >
                        <option value="Equity">Equity</option>
                        <option value="ConvertibleNote">Convertible Note</option>
                        <option value="SAFE">SAFE</option>
                        <option value="PreferredStock">Preferred Stock</option>
                    </select>
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Investment Amount (USD)
                    </label>
                    <input
                        type="number"
                        step="100000"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={move || current_round.get().amount.to_string()}
                        on:change=move |ev| {
                            let amount = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            update_round_amount.dispatch(amount);
                        }
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Pre-Money Valuation (USD)
                    </label>
                    <input
                        type="number"
                        step="100000"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={move || current_round.get().valuation.to_string()}
                        on:change=move |ev| {
                            let valuation = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            update_round_valuation.dispatch(valuation);
                        }
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        Equity Sold (%)
                    </label>
                    <input
                        type="number"
                        step="0.1"
                        min="0"
                        max="100"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={move || current_round.get().equity_sold.to_string()}
                        on:change=move |ev| {
                            let equity = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            update_equity_sold.dispatch(equity);
                        }
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                        ESOP Allocation (%)
                    </label>
                    <input
                        type="number"
                        step="0.1"
                        min="0"
                        max="100"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value={move || current_round.get().esop_allocation.to_string()}
                        on:change=move |ev| {
                            let esop = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                            update_esop_allocation.dispatch(esop);
                        }
                    />
                </div>
            </div>

            // Anti-dilution Protection
            <div class="mb-6">
                <label class="flex items-center">
                    <input
                        type="checkbox"
                        class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        checked={move || current_round.get().anti_dilution}
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            update_anti_dilution.dispatch(checked);
                        }
                    />
                    <span class="ml-2 text-sm text-gray-700">Anti-dilution protection</span>
                </label>
            </div>

            // Investors
            <div class="mb-6">
                <label class="block text-sm font-medium text-gray-700 mb-2">Investors</label>
                <div class="flex space-x-2 mb-3">
                    <input
                        type="text"
                        placeholder="Investor Name"
                        class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=new_investor
                        on:change=move |ev| {
                            set_new_investor.set(event_target_value(&ev));
                        }
                    />
                    <button
                        class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500"
                        on:click=move |_| {
                            if !new_investor.get().is_empty() {
                                add_investor.dispatch(new_investor.get());
                                set_new_investor.set("".to_string());
                            }
                        }
                    >
                        Add
                    </button>
                </div>
                
                <div class="space-y-2">
                    {move || {
                        current_round.get().investors.iter().enumerate().map(|(index, investor)| {
                            view! {
                                <div key=index class="flex items-center justify-between bg-gray-50 px-3 py-2 rounded">
                                    <span class="text-sm">{investor}</span>
                                    <button
                                        class="text-red-600 hover:text-red-800 text-sm"
                                        on:click=move |_| remove_investor.dispatch(index)
                                    >
                                        Remove
                                    </button>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>

            // Round Summary
            <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
                <h3 class="font-medium text-blue-800 mb-2">Round Summary</h3>
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div>
                        <span class="text-blue-600">Post-Money Valuation:</span>
                        <span class="ml-2 font-medium">{move || format!("${:.1}M", post_money_valuation.get() / 1_000_000.0)}</span>
                    </div>
                    <div>
                        <span class="text-blue-600">Equity Percentage:</span>
                        <span class="ml-2 font-medium">{move || format!("{:.1}%", equity_percentage.get())}</span>
                    </div>
                </div>
            </div>

            // Add Round Button
            <button
                class="w-full px-4 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 font-medium"
                on:click=move |_| {
                    add_round.dispatch(current_round.get());
                }
            >
                Add Funding Round
            </button>

            // Existing Rounds
            {move || {
                let rounds = company.get().funding_rounds.clone();
                if !rounds.is_empty() {
                    view! {
                        <div class="mt-6">
                            <h3 class="text-lg font-medium text-gray-800 mb-3">Existing Rounds</h3>
                            <div class="space-y-2">
                                {rounds.iter().map(|round| {
                                    view! {
                                        <div class="bg-gray-50 px-3 py-2 rounded text-sm">
                                            <span class="font-medium">{round.name.clone()}</span>
                                            <span class="text-gray-600 ml-2">
                                                - {format!("${:.1}M", round.amount / 1_000_000.0)} at 
                                                {format!("${:.1}M", round.valuation / 1_000_000.0)} valuation
                                            </span>
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


