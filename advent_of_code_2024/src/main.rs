mod advent_of_code {
    #[cfg(feature = "Day01")]
    pub mod day01_historian_hysteria;
    #[cfg(feature = "Day02")]
    pub mod day02_red_nosed_reports;
    #[cfg(feature = "Day03")]
    pub mod day03_mull_it_over;
    #[cfg(feature = "Day04")]
    pub mod day04_ceres_search;
    #[cfg(feature = "Day05")]
    pub mod day05_print_queue;
    #[cfg(feature = "Day06")]
    pub mod day06_guard_gallivant;
}

fn main() {
    #[cfg(feature = "Day01")] {
        println!("Day 01:");
        println!("\tPart 1: {}",advent_of_code::day01_historian_hysteria::part1());
        println!("\tPart 2: {}",advent_of_code::day01_historian_hysteria::part2());
    }

    #[cfg(feature = "Day02")] {
        println!("Day 02:");
        println!("\tPart 1: {}",advent_of_code::day02_red_nosed_reports::part1());
        println!("\tPart 2: {}",advent_of_code::day02_red_nosed_reports::part2());
    }
    #[cfg(feature = "Day03")] {
        println!("Day 03:");
        println!("\tPart 1: {}",advent_of_code::day03_mull_it_over::part1());
        println!("\tPart 2: {}",advent_of_code::day03_mull_it_over::part2());
    }
    #[cfg(feature = "Day04")] {
        println!("Day 04:");
        println!("\tPart 1: {}",advent_of_code::day04_ceres_search::part1());
        println!("\tPart 2: {}",advent_of_code::day04_ceres_search::part2());
    }

    #[cfg(feature = "Day05")] {
        println!("Day 05:");
        println!("\tPart 1: {}",advent_of_code::day05_print_queue::part1());
        println!("\tPart 2: {}",advent_of_code::day05_print_queue::part2());
    }

    #[cfg(feature = "Day06")] {
        println!("Day 06:");
        println!("\tPart 1: {}",advent_of_code::day06_guard_gallivant::part1());
        println!("\tPart 2: {}",advent_of_code::day06_guard_gallivant::part2());
    }
}