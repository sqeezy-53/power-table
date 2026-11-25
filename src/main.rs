const MIN_BASE: u64 = 2;
const MAX_BASE: u64 = 10;
const MIN_POWER: u32 = 1;
const MAX_POWER: u32 = 10;
const COLUMN_WIDTH: usize = 12;
const TABLE_WIDTH: usize = (MAX_POWER - MIN_POWER + 1) as usize * COLUMN_WIDTH + 8;

fn main() {
    print_header();
    println!("{}", "-".repeat(TABLE_WIDTH));
    print_table();
}

fn print_header() {
    print!("ЧИСЛО | ");

    for power in MIN_POWER..=MAX_POWER {
        print!("{:>COLUMN_WIDTH$}", power);
    }

    println!();
}

fn print_table() {
    for base in MIN_BASE..=MAX_BASE {
        print!("{:>6}| ", base);

        for power in MIN_POWER..=MAX_POWER {
            print!("{:>COLUMN_WIDTH$}", base.pow(power));
        }

        println!();
    }
}
