#[cfg(test)]
mod helpers {
    use topology;
    #[test]
    fn small_example_output() {
        let answ = topology::read_output("small_example.txt");
        assert_eq!(answ, (2, 4));
    }
    #[test]
    fn many_islands_output() {
        let answ = topology::read_output("many_islands.txt");
        assert_eq!(answ, (10, 20));
    }
    #[test]
    fn single_continent() {
        let answ = topology::read_output("single_continent_with_big_tiles.txt");
        assert_eq!(answ, (1, 16));
    }
    #[test]
    fn complex_map() {
        let answ = topology::read_output("complex_map.txt");
        assert_eq!(answ, (3, 130));
    }
}

#[cfg(test)]
mod codinggame {
    #[test]
    fn small_example() {
        
    }
}

