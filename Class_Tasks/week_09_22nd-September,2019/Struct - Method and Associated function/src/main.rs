#[derive(Debug)]
struct Student {
    name: String,
    marks: u32,
    suspended: bool,
}

impl Student {
    fn print_marks(&self) -> u32 {
        self.marks
    }
    fn do_suspend(&mut self) {
        self.suspended = true;
    }
}

impl Student {
    fn chg_marks(&mut self, new_marks: u32) {
        self.marks = new_marks;
    }
    fn calculate_per(a: f32, b: f32, c: f32) -> f32 {
        let score = a + b + c;
        let per = (score / 300.0) * 100.0;

        per
    }
}

fn main() {
    let mut std1 = Student {
        name: String::from("ali"),
        marks: 5,
        suspended: false,
    };
    let std2 = Student {
        name: String::from("hamza"),
        marks: 7,
        suspended: false,
    };

    println!("student marks are: {} ", std1.print_marks());
    println!("student is suspended: {} ", std1.suspended);
    println!("student marks: {} ", std1.marks);

    std1.do_suspend();
    std1.chg_marks(12);

    println!("student is suspended: {} ", std1.suspended);
    println!("student marks: {} ", std1.marks);

    println!("percentage: {} ", Student::calculate_per(100.0, 100.0, 50.0));
}
