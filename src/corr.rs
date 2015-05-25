//! Correlation functions.

use csr;

/// Pearson correlation.
fn pearson(a: csr::Row, b: csr::Row) -> f64 {
    let mut peekable_a = a.iter().peekable();
    let mut peekable_b = b.iter().peekable();

    let mut n = 0;
    let mut sum_a = 0.0;
    let mut sum_squared_a = 0.0;
    let mut sum_b = 0.0;
    let mut sum_squared_b = 0.0;
    let mut product_sum = 0.0;

    while let (Some(&value_a), Some(&value_b)) = (peekable_a.peek(), peekable_b.peek()) {
        if value_a.column < value_b.column {
            peekable_a.next();
        } else if value_a.column > value_b.column {
            peekable_b.next();
        } else {
            n += 1;
            sum_a += value_a.value;
            sum_squared_a += value_a.value * value_a.value;
            sum_b += value_b.value;
            sum_squared_b += value_b.value * value_b.value;
            product_sum += value_a.value * value_b.value;
            peekable_a.next();
            peekable_b.next();
        }
    }

    if n == 0 {
        return 0.0;
    }

    let numerator = product_sum - (sum_a * sum_b / n as f64);
    let denominator = ((sum_squared_a - sum_a  *sum_a / n as f64) * (sum_squared_b - sum_b * sum_b / n as f64)).sqrt();

    if denominator > 0.000001 { numerator / denominator } else { 0.0 }
}

#[test]
fn test_pearson() {
    let mut table = csr::Csr::new();

    const LADY_IN_THE_WATER: usize = 0;
    const SNAKES_ON_A_PLANE: usize = 1;
    const JUST_MY_LUCK: usize = 2;
    const SUPERMAN_RETURNS: usize = 3;
    const YOU_ME_AND_DUPREE: usize = 4;
    const THE_NIGHT_LISTENER: usize = 5;

    // Lisa Rose.
    table.start();
    table.next(LADY_IN_THE_WATER, 2.5);
    table.next(SNAKES_ON_A_PLANE, 3.5);
    table.next(JUST_MY_LUCK, 3.0);
    table.next(SUPERMAN_RETURNS, 3.5);
    table.next(YOU_ME_AND_DUPREE, 2.5);
    table.next(THE_NIGHT_LISTENER, 3.0);

    // Gene Seymour.
    table.start();
    table.next(LADY_IN_THE_WATER, 3.0);
    table.next(SNAKES_ON_A_PLANE, 3.5);
    table.next(JUST_MY_LUCK, 1.5);
    table.next(SUPERMAN_RETURNS, 5.0);
    table.next(YOU_ME_AND_DUPREE, 3.5);
    table.next(THE_NIGHT_LISTENER, 3.0);

    // Michael Phillips.
    table.start();
    table.next(LADY_IN_THE_WATER, 2.5);
    table.next(SNAKES_ON_A_PLANE, 3.0);
    table.next(SUPERMAN_RETURNS, 3.5);
    table.next(THE_NIGHT_LISTENER, 4.0);

    // Claudia Puig.
    table.start();
    table.next(SNAKES_ON_A_PLANE, 3.5);
    table.next(JUST_MY_LUCK, 3.0);
    table.next(SUPERMAN_RETURNS, 4.0);
    table.next(YOU_ME_AND_DUPREE, 2.5);
    table.next(THE_NIGHT_LISTENER, 4.5);

    // Mick LaSalle.
    table.start();
    table.next(LADY_IN_THE_WATER, 3.0);
    table.next(SNAKES_ON_A_PLANE, 4.0);
    table.next(JUST_MY_LUCK, 2.0);
    table.next(SUPERMAN_RETURNS, 3.0);
    table.next(YOU_ME_AND_DUPREE, 2.0);
    table.next(THE_NIGHT_LISTENER, 3.0);

    // Jack Matthews.
    table.start();
    table.next(LADY_IN_THE_WATER, 3.0);
    table.next(SNAKES_ON_A_PLANE, 4.0);
    table.next(SUPERMAN_RETURNS, 5.0);
    table.next(YOU_ME_AND_DUPREE, 3.5);
    table.next(THE_NIGHT_LISTENER, 3.0);

    // Toby.
    table.start();
    table.next(SNAKES_ON_A_PLANE, 4.5);
    table.next(SUPERMAN_RETURNS, 4.0);
    table.next(YOU_ME_AND_DUPREE, 1.0);

    // Unknown Artist.
    table.start();
    table.next(THE_NIGHT_LISTENER, 4.5);

    table.start();

    assert_eq!(pearson(table.get_row(0), table.get_row(1)), 0.39605901719066976);
    assert_eq!(pearson(table.get_row(6), table.get_row(0)), 0.99124070716192991);
    assert_eq!(pearson(table.get_row(6), table.get_row(3)), 0.89340514744156474);
    assert_eq!(pearson(table.get_row(6), table.get_row(4)), 0.92447345164190486);
    assert_eq!(pearson(table.get_row(6), table.get_row(7)), 0.0);
}