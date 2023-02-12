struct City {
    name: String,
    population: i32,
}

#[cfg(test)]
mod tests {
    use super::City;

    fn city_population_descending(city: &City) -> i32 {
        -city.population
    }

    #[test]
    fn test_custom_sort() {
        let mut cities = vec![
            City {
                name: "fukuoka".to_string(),
                population: 100_000,
            },
            City {
                name: "tokyo".to_string(),
                population: 1_000_000,
            },
            City {
                name: "osaka".to_string(),
                population: 500_000,
            },
        ];

        cities.sort_by_key(city_population_descending);

        assert_eq!(cities[0].name, "tokyo".to_string());
        assert_eq!(cities[1].name, "osaka".to_string());
        assert_eq!(cities[2].name, "fukuoka".to_string());
    }

    #[test]
    fn test_closure() {
        let mut cities = vec![
            City {
                name: "fukuoka".to_string(),
                population: 100_000,
            },
            City {
                name: "tokyo".to_string(),
                population: 1_000_000,
            },
            City {
                name: "osaka".to_string(),
                population: 500_000,
            },
        ];

        cities.sort_by_key(|city| -city.population);

        assert_eq!(cities[0].name, "tokyo".to_string());
        assert_eq!(cities[1].name, "osaka".to_string());
        assert_eq!(cities[2].name, "fukuoka".to_string());
    }
}
