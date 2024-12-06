use std::collections::HashSet;

#[path = "../input.rs"]
mod input;

type Page = u32;
type Update = Vec<Page>;
type Updates = Vec<Update>;
type OrderRule = (Page, Page);
type OrderRules = HashSet<OrderRule>;

fn main() {
    let (rules, updates) = parse_input(&input::read_stdin());
    println!(
        "sum of ok middle numbers: {}",
        part1_sum_ok_update_middle_nos(&rules, &updates)
    );
    println!(
        "sum of fixed middle numbers: {}",
        part2_sum_fixed_update_middle_nos(&rules, &updates)
    );
}

fn part1_sum_ok_update_middle_nos(rules: &OrderRules, updates: &Updates) -> Page {
    updates
        .iter()
        .filter(|update| is_update_ok(rules, update))
        .fold(0, |sum, update| sum + middle_value(&update))
}

fn part2_sum_fixed_update_middle_nos(rules: &OrderRules, updates: &Updates) -> Page {
    updates
        .iter()
        .filter(|update| !is_update_ok(rules, update))
        .map(|update| fix_update(rules, update))
        .fold(0, |sum, update| sum + middle_value(&update))
}

fn middle_value<T>(x: &[T]) -> &T {
    &x[x.len() / 2]
}

fn is_update_ok(rules: &OrderRules, update: &Update) -> bool {
    let mut seen: HashSet<Page> = HashSet::new();
    update.iter().all(|page| {
        let ok = seen
            .iter()
            .all(|seen_page| is_order_ok(rules, *seen_page, *page));
        seen.insert(*page);
        ok
    })
}

fn is_order_ok(rules: &OrderRules, page1: Page, page2: Page) -> bool {
    !rules.contains(&(page2, page1))
}

fn fix_update(rules: &OrderRules, update: &Update) -> Update {
    let update_len = update.len();
    let mut new_update = update.clone();
    for i in 0..update_len {
        for j in 0..update_len {
            if !is_order_ok(rules, new_update[i], new_update[j]) {
                new_update.swap(i, j)
            }
        }
    }
    new_update
}

fn parse_input(input: &str) -> (OrderRules, Updates) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    (
        sections
            .get(0)
            .unwrap()
            .lines()
            .fold(HashSet::new(), |mut set, line| {
                let line_split = line.split('|').collect::<Vec<_>>();
                set.insert((
                    line_split.get(0).unwrap().parse::<Page>().unwrap(),
                    line_split.get(1).unwrap().parse::<Page>().unwrap(),
                ));
                set
            }),
        sections
            .get(1)
            .unwrap()
            .lines()
            .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
            .collect(),
    )
}
