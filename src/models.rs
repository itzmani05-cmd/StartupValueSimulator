use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Founder {
    pub name: String,
    pub initial_ownership: f64, // percentage
    pub current_ownership: f64, // percentage
    pub shares: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DealType {
    Equity,
    ConvertibleNote,
    SAFE,
    PreferredStock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRound {
    pub name: String,
    pub amount: f64,      // in USD
    pub valuation: f64,   // pre-money valuation
    pub equity_sold: f64, // percentage of company sold
    pub deal_type: DealType,
    pub investors: Vec<String>,
    pub esop_allocation: f64, // percentage for employee stock options
    pub anti_dilution: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESOPPool {
    pub total_allocation: f64, // percentage
    pub allocated: f64,        // percentage
    pub reserved: f64,         // percentage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub total_shares: u64,
    pub founders: Vec<Founder>,
    pub funding_rounds: Vec<FundingRound>,
    pub esop_pool: ESOPPool,
    pub exit_scenarios: Vec<ExitScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitScenario {
    pub name: String,
    pub exit_valuation: f64, // in USD
    pub exit_type: ExitType,
    pub founder_payouts: Vec<FounderPayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExitType {
    IPO,
    Acquisition,
    Merger,
    SecondarySale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderPayout {
    pub founder_name: String,
    pub ownership_at_exit: f64, // percentage
    pub shares_at_exit: u64,
    pub payout_amount: f64,   // in USD
    pub dilution_impact: f64, // percentage points lost
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipSnapshot {
    pub round_name: String,
    pub founder_ownership: Vec<(String, f64)>, // (name, percentage)
    pub investor_ownership: Vec<(String, f64)>, // (name, percentage)
    pub esop_ownership: f64,                   // percentage
    pub total_valuation: f64,                  // post-money
}

impl Default for Company {
    fn default() -> Self {
        Self {
            name: "My Startup".to_string(),
            total_shares: 10_000_000, // 10M shares
            founders: vec![
                Founder {
                    name: "Founder 1".to_string(),
                    initial_ownership: 50.0,
                    current_ownership: 50.0,
                    shares: 5_000_000,
                },
                Founder {
                    name: "Founder 2".to_string(),
                    initial_ownership: 50.0,
                    current_ownership: 50.0,
                    shares: 5_000_000,
                },
            ],
            funding_rounds: vec![],
            esop_pool: ESOPPool {
                total_allocation: 0.0,
                allocated: 0.0,
                reserved: 0.0,
            },
            exit_scenarios: vec![],
        }
    }
}

impl Company {
    pub fn calculate_ownership_after_round(&mut self, round: &FundingRound) -> OwnershipSnapshot {
        let pre_round_valuation = round.valuation;
        let post_round_valuation = pre_round_valuation + round.amount;

        // Calculate new total shares after dilution
        let new_shares =
            (self.total_shares as f64 * post_round_valuation / pre_round_valuation) as u64;
        let shares_sold = (new_shares as f64 * round.equity_sold / 100.0) as u64;

        // Update founder ownership
        for founder in &mut self.founders {
            let old_shares = founder.shares;
            let new_shares =
                (old_shares as f64 * pre_round_valuation / post_round_valuation) as u64;
            founder.shares = new_shares;
            founder.current_ownership = (new_shares as f64 / new_shares as f64) * 100.0;
        }

        // Update ESOP pool
        if round.esop_allocation > 0.0 {
            self.esop_pool.total_allocation += round.esop_allocation;
        }

        // Calculate ownership breakdown
        let founder_ownership: Vec<(String, f64)> = self
            .founders
            .iter()
            .map(|f| (f.name.clone(), f.current_ownership))
            .collect();

        let investor_ownership: Vec<(String, f64)> = round
            .investors
            .iter()
            .map(|name| {
                (
                    name.clone(),
                    round.equity_sold / round.investors.len() as f64,
                )
            })
            .collect();

        OwnershipSnapshot {
            round_name: round.name.clone(),
            founder_ownership,
            investor_ownership,
            esop_ownership: self.esop_pool.total_allocation,
            total_valuation: post_round_valuation,
        }
    }

    pub fn calculate_exit_scenario(&self, exit_valuation: f64) -> ExitScenario {
        let mut founder_payouts = Vec::new();

        for founder in &self.founders {
            let payout_amount = (founder.current_ownership / 100.0) * exit_valuation;
            let dilution_impact = founder.initial_ownership - founder.current_ownership;

            founder_payouts.push(FounderPayout {
                founder_name: founder.name.clone(),
                ownership_at_exit: founder.current_ownership,
                shares_at_exit: founder.shares,
                payout_amount,
                dilution_impact,
            });
        }

        ExitScenario {
            name: format!("${:.1}M Exit", exit_valuation / 1_000_000.0),
            exit_valuation,
            exit_type: ExitType::Acquisition,
            founder_payouts,
        }
    }
}
