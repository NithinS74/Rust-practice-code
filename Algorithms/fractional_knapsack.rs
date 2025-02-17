struct item {
    weight: f32,
    value: f32,
    unitv: f32,
}

impl item {
    fn new(weight: f32, value: f32) -> Self {
        let mut unitv: f32 = 0.0;
        if weight == 0 as f32 {
        } else {
            unitv = value / weight;
        }

        Self {
            weight,
            value,
            unitv,
        }
    }
}
fn fractional_knapsack() {
    let values = [
        360, 83, 59, 130, 431, 67, 230, 52, 93, 125, 670, 892, 600, 38, 48, 147, 86, 256, 63, 17,
        120, 164, 432, 35, 92, 110, 22, 42, 50, 323, 514, 28, 87, 73, 78, 15, 26, 78, 210, 36, 85,
        189, 274, 43, 33, 10, 19, 389, 276, 314,
    ];

    let weights = [
        7, 0, 30, 22, 80, 94, 11, 81, 70, 64, 59, 18, 0, 36, 3, 8, 15, 42, 9, 0, 42, 47, 52, 32,
        26, 48, 55, 6, 29, 84, 2, 4, 18, 56, 7, 29, 93, 44, 71, 3, 86, 66, 31, 65, 0, 79, 20, 64,
        58, 17,
    ];
    let knapsack: f32 = 800.00;
    let mut store: Vec<item> = vec![];
    if values.len() == weights.len() {
        for i in 0..values.len() {
            store.push(item::new(weights[i] as f32, values[i] as f32));
        }
    }
    store.sort_by_key(|item| item.unitv as i64);
    let total_profit = greedy_knapsack(&store, knapsack);
    println!(" the total profit is : {total_profit}");
}

fn greedy_knapsack(store: &Vec<item>, sack_size: f32) -> f32 {
    let mut sack: f32 = sack_size;
    let mut profit: f32 = 0.0;
    for i in store.iter().rev() {
        if sack >= i.weight {
            println!("added weight:{} added value: {} ", i.weight, i.value);
            sack -= i.weight;
            profit += i.value;
        } else {
            profit += i.value * sack / i.weight;
            println!(
                "added weight:{} added value: {} ",
                sack,
                (i.value * sack / i.weight)
            );
            break;
        }
    }
    return profit;
}
