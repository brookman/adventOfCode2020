#[derive(Clone, Debug)]
pub struct SimulationData {
    pub seed_money: u32,
    pub properties: Vec<Property>,
}

#[derive(Clone, Debug)]
pub struct Property {
    pub name: String,
    pub construction_year: u32,
    pub income_property: bool,
    pub values: Vec<u32>,
    pub lifetime: u32,
}

#[derive(Clone, Debug)]
pub struct Output {
    pub actions: Vec<Vec<Action>>,
}

#[derive(Debug, Clone)]
pub enum Action {
    Buy(usize, u8),
    Sell(usize),
    // Renovate(usize),
}

#[derive(Debug, Clone)]
pub struct PurchaseRecord {
    pub year: u32,
    pub purchase_price: f64,
    pub loan_taken: f64,
}

#[derive(Debug, Clone)]
pub struct LoanPercentage {
    pub percentage: u8,
}

#[derive(Debug, Clone)]
pub struct PotentialPurchase {
    pub property_index: usize,
    pub price: u32,
    pub profit: u32,
    pub roi: f64,
}

impl Property {
    pub fn buy_value(&self, year: u32) -> Result<u32, String> {
        if !self.for_sale(year) {
            return Err(format!("Not for sale in year {}", year));
        }
        Ok(self.values[(year - self.construction_year) as usize])
    }

    pub fn sell_value(&self, year: u32) -> u32 {
        if self.for_sale(year) {
            self.values[(year - self.construction_year) as usize]
        } else {
            0
        }
    }

    pub fn can_buy(&self, year: u32, funds: f64, loan_percentage: &LoanPercentage) -> bool {
        if let Ok(value) = self.buy_value(year) {
            let price_to_pay = loan_percentage.get_out_of_pocket_value(value);
            if funds >= price_to_pay as f64 {
                return true;
            }
        }
        false
    }

    fn for_sale(&self, year: u32) -> bool {
        year >= self.construction_year && year < self.lifetime
    }
}

impl LoanPercentage {
    pub fn new(percentage: u8) -> Result<LoanPercentage, String> {
        if percentage > 50 {
            Err(format!("Percentage must be <= 50, but was {}", percentage))
        } else {
            Ok(LoanPercentage {
                percentage
            })
        }
    }

    pub fn get_out_of_pocket_value(&self, price: u32) -> f64 {
        (100 - self.percentage) as f64 * price as f64 / 100.0
    }

    // pub fn get_loan_value(&self, price: u32) -> f64 {
    //     self.percentage as f64 * price as f64 / 100.0
    // }
}


