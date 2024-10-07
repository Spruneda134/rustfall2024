#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        //TODO! (implement printing info about Student
        // use match statement)
        match self.grade {
            GradeLevel::Bachelor => println!("Hello, my name is {}, i am studying to get a {:?} in {:?}", self.name, self.grade, self.major),
            GradeLevel::Master => println!("Hello, my name is {}, i am studying to get a {:?} in {:?}", self.name, self.grade, self.major),
            GradeLevel::PhD => println!("Hello, my name is {}, i am studying to get a {:?} in {:?}", self.name, self.grade, self.major),
        };
        
       
        }
    }

fn main() {
    let s1 = Student::new("Salvador".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
s1.introduce_yourself();
}