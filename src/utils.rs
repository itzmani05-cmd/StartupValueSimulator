use crate::models::{Company, ExitScenario, FundingRound, OwnershipSnapshot};

pub fn format_currency(amount: f64) -> String {
    if amount >= 1_000_000_000.0 {
        format!("${:.1}B", amount / 1_000_000_000.0)
    } else if amount >= 1_000_000.0 {
        format!("${:.1}M", amount / 1_000_000.0)
    } else if amount >= 1_000.0 {
        format!("${:.1}K", amount / 1_000.0)
    } else {
        format!("${:.0}", amount)
    }
}

pub fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value)
}

pub fn format_shares(shares: u64) -> String {
    if shares >= 1_000_000 {
        format!("{:.1}M", shares as f64 / 1_000_000.0)
    } else if shares >= 1_000 {
        format!("{:.1}K", shares as f64 / 1_000.0)
    } else {
        shares.to_string()
    }
}

pub fn calculate_dilution_impact(initial_ownership: f64, current_ownership: f64) -> f64 {
    initial_ownership - current_ownership
}

pub fn calculate_ownership_value(ownership_percentage: f64, company_valuation: f64) -> f64 {
    (ownership_percentage / 100.0) * company_valuation
}

pub fn calculate_post_money_valuation(pre_money: f64, investment_amount: f64) -> f64 {
    pre_money + investment_amount
}

pub fn calculate_equity_percentage(investment_amount: f64, post_money_valuation: f64) -> f64 {
    (investment_amount / post_money_valuation) * 100.0
}

pub fn calculate_new_shares_issued(
    investment_amount: f64,
    pre_money_valuation: f64,
    existing_shares: u64,
) -> u64 {
    let post_money = pre_money_valuation + investment_amount;
    let equity_percentage = investment_amount / post_money;
    let new_shares =
        (existing_shares as f64 * equity_percentage / (1.0 - equity_percentage)) as u64;
    new_shares
}

pub fn calculate_founder_dilution(
    founder_shares: u64,
    total_shares_before: u64,
    total_shares_after: u64,
) -> f64 {
    let ownership_before = (founder_shares as f64 / total_shares_before as f64) * 100.0;
    let ownership_after = (founder_shares as f64 / total_shares_after as f64) * 100.0;
    ownership_before - ownership_after
}

pub fn simulate_funding_round(company: &mut Company, round: &FundingRound) -> OwnershipSnapshot {
    // Calculate new total shares
    let pre_round_shares = company.total_shares;
    let post_round_valuation = round.valuation + round.amount;
    let new_shares = (pre_round_shares as f64 * post_round_valuation / round.valuation) as u64;

    // Update company total shares
    company.total_shares = new_shares;

    // Calculate dilution for each founder
    for founder in &mut company.founders {
        let old_ownership = founder.current_ownership;
        let new_ownership = (founder.shares as f64 / new_shares as f64) * 100.0;
        founder.current_ownership = new_ownership;

        // Note: shares remain the same, only ownership percentage changes
    }

    // Update ESOP pool
    if round.esop_allocation > 0.0 {
        company.esop_pool.total_allocation += round.esop_allocation;
    }

    // Create ownership snapshot
    let founder_ownership: Vec<(String, f64)> = company
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
        esop_ownership: company.esop_pool.total_allocation,
        total_valuation: post_round_valuation,
    }
}

pub fn generate_exit_scenarios(company: &Company, exit_valuations: Vec<f64>) -> Vec<ExitScenario> {
    exit_valuations
        .into_iter()
        .map(|valuation| company.calculate_exit_scenario(valuation))
        .collect()
}

pub fn calculate_cap_table_summary(company: &Company) -> Vec<(String, f64, u64)> {
    let mut cap_table = Vec::new();

    // Add founders
    for founder in &company.founders {
        cap_table.push((
            founder.name.clone(),
            founder.current_ownership,
            founder.shares,
        ));
    }

    // Add ESOP pool
    if company.esop_pool.total_allocation > 0.0 {
        let esop_shares =
            (company.total_shares as f64 * company.esop_pool.total_allocation / 100.0) as u64;
        cap_table.push((
            "ESOP Pool".to_string(),
            company.esop_pool.total_allocation,
            esop_shares,
        ));
    }

    // Add investors from funding rounds
    for round in &company.funding_rounds {
        for investor in &round.investors {
            let investor_ownership = round.equity_sold / round.investors.len() as f64;
            let investor_shares = (company.total_shares as f64 * investor_ownership / 100.0) as u64;
            cap_table.push((investor.clone(), investor_ownership, investor_shares));
        }
    }

    cap_table
}

pub fn validate_funding_round(round: &FundingRound) -> Result<(), String> {
    if round.amount <= 0.0 {
        return Err("Investment amount must be positive".to_string());
    }

    if round.valuation <= 0.0 {
        return Err("Pre-money valuation must be positive".to_string());
    }

    if round.equity_sold <= 0.0 || round.equity_sold > 100.0 {
        return Err("Equity sold must be between 0% and 100%".to_string());
    }

    if round.esop_allocation < 0.0 || round.esop_allocation > 100.0 {
        return Err("ESOP allocation must be between 0% and 100%".to_string());
    }

    if round.investors.is_empty() {
        return Err("At least one investor must be specified".to_string());
    }

    Ok(())
}
