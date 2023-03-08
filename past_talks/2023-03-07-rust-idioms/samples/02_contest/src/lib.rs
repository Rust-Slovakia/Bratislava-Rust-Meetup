struct Contestant {
    name: String,
    score: usize,
}

struct Country {
    name: String,
    contestants: Vec<Contestant>,
}

struct Contest {
    countries: Vec<Country>,
}

/// Returns lines to be printed
fn results_list_functional(con: Contest) -> Vec<String> {
    con.countries
        .iter()
        .flat_map(|country| country.contestants.iter())
        .map(|c| format!("{} | {}", &c.score, &c.name))
        .collect()
}

fn results_list_proc(con: Contest) -> Vec<String> {
    let mut result = vec![];
    for country in &con.countries {
        for c in &country.contestants {
            result.push(format!("{} | {}", c.score, c.name));
        }
    }
    result
}
