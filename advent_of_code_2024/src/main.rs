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
    #[cfg(feature = "Day07")]
    pub mod day07_bridge_repair;
    #[cfg(feature = "Day08")]
    pub mod day08_resonant_collinearity;
    #[cfg(feature = "Day09")]
    pub mod day09_disk_fragmenter;
    #[cfg(feature = "Day10")]
    pub mod day10_hoof_it;
    #[cfg(feature = "Day11")]
    pub mod day11_plutonian_pebbles;
    #[cfg(feature = "Day12")]
    pub mod day12_garden_groups;
    #[cfg(feature = "Day13")]
    pub mod day13_claw_contraption;
    #[cfg(feature = "Day14")]
    pub mod day14_restroom_redoubt;
    #[cfg(feature = "Day15")]
    pub mod day15_warehouse_woes;
    #[cfg(feature = "Day16")]
    pub mod day16_reindeer_maze;
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

    #[cfg(feature = "Day07")] {
        advent_of_code::day07_bridge_repair::part1();
        advent_of_code::day07_bridge_repair::part2();
    }

    #[cfg(feature = "Day08")] {
        advent_of_code::day08_resonant_collinearity::part1();
        advent_of_code::day08_resonant_collinearity::part2();
    }

    #[cfg(feature = "Day09")] {
        advent_of_code::day09_disk_fragmenter::part1();
        advent_of_code::day09_disk_fragmenter::part2();
    }

    #[cfg(feature = "Day10")] {
        advent_of_code::day10_hoof_it::part1();
        advent_of_code::day10_hoof_it::part2();
    }

    #[cfg(feature = "Day11")] {
        advent_of_code::day11_plutonian_pebbles::part1();
        advent_of_code::day11_plutonian_pebbles::part2();
    }

    #[cfg(feature = "Day12")] {
        advent_of_code::day12_garden_groups::part1();
        advent_of_code::day12_garden_groups::part2();
    }

    #[cfg(feature = "Day13")] {
        advent_of_code::day13_claw_contraption::part1();
        advent_of_code::day13_claw_contraption::part2();
    }

    #[cfg(feature = "Day14")] {
        advent_of_code::day14_restroom_redoubt::part1();
        advent_of_code::day14_restroom_redoubt::part2();
    }

    #[cfg(feature = "Day15")] {
        advent_of_code::day15_warehouse_woes::part1();
        advent_of_code::day15_warehouse_woes::part2();
    }

    #[cfg(feature = "Day16")] {
        advent_of_code::day16_reindeer_maze::part1();
        advent_of_code::day16_reindeer_maze::part2();
    }

}