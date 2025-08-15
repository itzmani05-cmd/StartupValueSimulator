use leptos::*;
use crate::models::{Company, ExitScenario, ExitType};
use crate::utils::{format_currency, format_percentage};

#[component]
pub fn ExitScenarios(
    scenarios: ReadSignal<Vec<ExitScenario>>,
    company: ReadSignal<Company>,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg p-6">
            <h2 class="text-2xl font-bold text-gray-800 mb-6">Exit Scenarios</h2>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {move || {
                    scenarios.get().iter().map(|scenario| {
                        view! {
                            <div key={scenario.name.clone()} class="bg-gray-50 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-4">
                                    <h3 class="text-lg font-semibold text-gray-800">
                                        {scenario.name.clone()}
                                    </h3>
                                    <span class="text-sm text-gray-500">
                                        {match scenario.exit_type {
                                            ExitType::IPO => "IPO",
                                            ExitType::Acquisition => "Acquisition",
                                            ExitType::Merger => "Merger",
                                            ExitType::SecondarySale => "Secondary Sale",
                                        }}
                                    </span>
                                </div>
                                
                                <div class="mb-4">
                                    <div class="text-2xl font-bold text-blue-600">
                                        {format_currency(scenario.exit_valuation)}
                                    </div>
                                    <div class="text-sm text-gray-600">Exit Valuation</div>
                                </div>
                                
                                // Founder Payouts
                                <div class="space-y-3">
                                    <h4 class="font-medium text-gray-700">Founder Payouts</h4>
                                    {scenario.founder_payouts.iter().map(|payout| {
                                        let payout_percentage = (payout.payout_amount / scenario.exit_valuation) * 100.0;
                                        
                                        view! {
                                            <div key={payout.founder_name.clone()} class="bg-white rounded p-3 border">
                                                <div class="flex justify-between items-center mb-2">
                                                    <span class="font-medium text-gray-800">
                                                        {payout.founder_name.clone()}
                                                    </span>
                                                    <span class="text-sm text-gray-500">
                                                        {format_percentage(payout.ownership_at_exit)}
                                                    </span>
                                                </div>
                                                
                                                <div class="grid grid-cols-2 gap-2 text-sm">
                                                    <div>
                                                        <span class="text-gray-600">Payout:</span>
                                                        <span class="ml-2 font-semibold text-green-600">
                                                            {format_currency(payout.payout_amount)}
                                                        </span>
                                                    </div>
                                                    <div>
                                                        <span class="text-gray-600">Dilution:</span>
                                                        <span class={if payout.dilution_impact > 10.0 { "ml-2 font-semibold text-red-600" } 
                                                                   else if payout.dilution_impact > 5.0 { "ml-2 font-semibold text-orange-600" } 
                                                                   else { "ml-2 font-semibold text-green-600" }}>
                                                            {if payout.dilution_impact > 0.0 {
                                                                format!("-{:.1}%", payout.dilution_impact)
                                                            } else {
                                                                "None".to_string()
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                                
                                // Total Founder Value
                                <div class="mt-4 pt-3 border-t border-gray-200">
                                    <div class="flex justify-between items-center">
                                        <span class="font-medium text-gray-700">Total Founder Value:</span>
                                        <span class="text-lg font-bold text-green-600">
                                            {format_currency(scenario.founder_payouts.iter().map(|p| p.payout_amount).sum::<f64>())}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
            
            // Scenario Comparison
            {move || {
                let scenarios_data = scenarios.get();
                if scenarios_data.len() > 1 {
                    view! {
                        <div class="mt-8">
                            <h3 class="text-lg font-medium text-gray-800 mb-4">Scenario Comparison</h3>
                            <div class="overflow-x-auto">
                                <table class="min-w-full divide-y divide-gray-200">
                                    <thead class="bg-gray-50">
                                        <tr>
                                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                                Exit Value
                                            </th>
                                            {company.get().founders.iter().map(|founder| {
                                                view! {
                                                    <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                                        {founder.name.clone()}
                                                    </th>
                                                }
                                            }).collect::<Vec<_>>()}
                                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                                Total Founders
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white divide-y divide-gray-200">
                                        {scenarios_data.iter().map(|scenario| {
                                            let total_founder_value: f64 = scenario.founder_payouts.iter().map(|p| p.payout_amount).sum();
                                            
                                            view! {
                                                <tr>
                                                    <td class="px-4 py-3 whitespace-nowrap text-sm font-medium text-gray-900">
                                                        {format_currency(scenario.exit_valuation)}
                                                    </td>
                                                    {scenario.founder_payouts.iter().map(|payout| {
                                                        view! {
                                                            <td class="px-4 py-3 whitespace-nowrap text-sm text-gray-900">
                                                                {format_currency(payout.payout_amount)}
                                                            </td>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                    <td class="px-4 py-3 whitespace-nowrap text-sm font-bold text-green-600">
                                                        {format_currency(total_founder_value)}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
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


