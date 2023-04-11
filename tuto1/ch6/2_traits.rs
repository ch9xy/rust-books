struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32
};

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String
};

trait GeneralInfo{
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str; 
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }

}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.country;
    }
}