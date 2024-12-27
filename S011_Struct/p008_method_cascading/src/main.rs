struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    fn new() -> Self {
        Self {
            year: 0,
            month: 0,
            day: 0,
        }
    }

    fn update_year(mut self, year: u32) -> Self {
        self.year = year;
        self
    }

    fn update_month(mut self, month: u32) -> Self {
        self.month = month;
        self
    }

    fn update_day(mut self, day: u32) -> Self {
        self.day = day;
        self
    }

    fn update(&mut self, year: u32, month: u32, day: u32) {
        self.year = year;
        self.month = month;
        self.day = day;
    }
}

fn main() {
    let mut my_date = Date::new();
    my_date.year = 2025;
    println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);

    let mut my_date = Date::new().update_year(2025);
    println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);

    let mut my_date = Date::new().update_year(2025).update_month(2).update_day(12);
    println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);

    let mut my_date = Date::new().update_month(2).update_day(12).update_year(2025);
    println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);

    let mut my_date = Date::new().update_day(12).update_year(2025);
    println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);
    // my_date.update_month(3);
    // println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);

    let mut my_date = my_date.update_month(3);
    println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);

    my_date.update(2026, 12, 2);
    println!("{}-{}-{}", my_date.year, my_date.month, my_date.day);
}
