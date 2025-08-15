use leptos::*;
use leptos_meta::*;
use crate::models::{Company, FundingRound, DealType, ExitType};
use crate::utils::{simulate_funding_round, generate_exit_scenarios, calculate_cap_table_summary};
use crate::components::{FounderSetup, FundingRounds, CapTable, ExitScenarios, OwnershipChart};

#[component]
pub fn App() -> impl IntoView {
    let (company, set_company) = create_signal(Company::default());
    
    // Track current funding round being added
    let (current_round, set_current_round) = create_signal(FundingRound {
        name: "Seed Round".to_string(),
        amount: 1_000_000.0,
        valuation: 5_000_000.0,
        equity_sold: 20.0,
        deal_type: DealType::Equity,
        investors: vec!["VC Fund".to_string()],
        esop_allocation: 10.0,
        anti_dilution: false,
    });

    // Add funding round
    let add_funding_round = create_action(move |round: &FundingRound| {
        let mut new_company = company.get();
        new_company.funding_rounds.push(round.clone());
        
        // Simulate the round and update company state
        let snapshot = simulate_funding_round(&mut new_company, round);
        set_company.set(new_company);
    });

    // Generate exit scenarios
    let exit_scenarios = create_memo(move |_| {
        let valuations = vec![
            10_000_000.0,   // $10M
            25_000_000.0,   // $25M
            50_000_000.0,   // $50M
            100_000_000.0,  // $100M
            250_000_000.0,  // $250M
            500_000_000.0,  // $500M
            1_000_000_000.0, // $1B
        ];
        generate_exit_scenarios(&company.get(), valuations)
    });

    // Cap table summary
    let cap_table = create_memo(move |_| {
        calculate_cap_table_summary(&company.get())
    });

    view! {
        <Html lang="en" />
        <Head>
            <Title text="Startup Equity Scenario Builder" />
            <Meta name="description" content="Model equity splits, funding rounds, and exit scenarios for your startup" />
            <Meta charset="utf-8" />
            <Meta name="viewport" content="width=device-width, initial-scale=1" />
            
            // Tailwind CSS CDN
            <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet" />
        </Head>

        <div class="min-h-screen bg-gray-50">
            // Header
            <header class="bg-white shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center py-6">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <h1 class="text-3xl font-bold text-gray-900">
                                    🚀 Startup Equity Scenario Builder
                                </h1>
                            </div>
                        </div>
                        <div class="text-sm text-gray-500">
                            Model ownership changes across funding rounds
                        </div>
                    </div>
                </div>
            </header>

            // Main Content
            <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                <div class="mb-8">
                    <div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
                        <h2 class="text-xl font-semibold text-blue-800 mb-2">
                            Welcome to the Equity Scenario Builder
                        </h2>
                        <p class="text-blue-700">
                            Model how funding rounds, ESOP allocations, and equity splits affect founder ownership 
                            and eventual payouts. Test different scenarios to understand dilution impact.
                        </p>
                    </div>
                </div>

                // Company Setup
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    // Left Column - Setup & Inputs
                    <div class="lg:col-span-1 space-y-6">
                        <FounderSetup company=company set_company=set_company />
                        
                        <FundingRounds 
                            current_round=current_round 
                            set_current_round=set_current_round 
                            add_round=add_funding_round 
                            company=company 
                        />
                    </div>

                    // Right Column - Results & Visualizations
                    <div class="lg:col-span-2 space-y-6">
                        <CapTable cap_table=cap_table company=company />
                        
                        <OwnershipChart company=company />
                        
                        <ExitScenarios scenarios=exit_scenarios company=company />
                    </div>
                </div>

                // Footer
                <div class="mt-12 text-center text-gray-500 text-sm">
                    <p>
                        This tool helps model equity scenarios but should not be considered as financial or legal advice. 
                        Consult with qualified professionals for actual equity planning.
                    </p>
                </div>
            </main>
        </div>
    }
}
