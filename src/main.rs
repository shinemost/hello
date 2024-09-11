use std::collections::HashMap;

fn main() {
    let mut country_major_cities = HashMap::new();
    country_major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    country_major_cities.insert("US", vec!["Portland", "Nashville"]);
    country_major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    country_major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);

    // flat_map返回任意种类的可迭代者
    let countries = ["Japan", "US", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &country_major_cities[country]) {
        println!("{}", city);
    }
}