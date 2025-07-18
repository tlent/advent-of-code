static INPUT: &str = include_str!("../input");

fn main() {
    let passports: Vec<_> = INPUT.split("\n\n").map(Passport::parse).collect();
    println!(
        "part one: {}",
        passports.iter().filter(|p| Passport::is_valid(p)).count()
    );
    println!(
        "part two: {}",
        passports
            .iter()
            .filter(|p| Passport::is_valid_part_two(p))
            .count()
    );
}

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn parse(input: &str) -> Self {
        let mut passport = Passport::default();
        for field in input.split_whitespace() {
            let mut parts = field.split(':');
            let key = parts.next().unwrap();
            let value = Some(String::from(parts.next().unwrap()));
            match key {
                "byr" => passport.birth_year = value,
                "iyr" => passport.issue_year = value,
                "eyr" => passport.expiration_year = value,
                "hgt" => passport.height = value,
                "hcl" => passport.hair_color = value,
                "ecl" => passport.eye_color = value,
                "pid" => passport.passport_id = value,
                "cid" => passport.country_id = value,
                _ => panic!("invalid key"),
            }
        }
        passport
    }

    fn is_valid(&self) -> bool {
        // Country ID purposely excluded
        [
            self.birth_year.as_ref(),
            self.issue_year.as_ref(),
            self.expiration_year.as_ref(),
            self.height.as_ref(),
            self.hair_color.as_ref(),
            self.eye_color.as_ref(),
            self.passport_id.as_ref(),
        ]
        .iter()
        .all(Option::is_some)
    }

    fn is_valid_birth_year(s: &str) -> bool {
        match s.parse::<u32>() {
            Err(_) => false,
            Ok(v) => v >= 1920 && v <= 2002,
        }
    }

    fn is_valid_issue_year(s: &str) -> bool {
        match s.parse::<u32>() {
            Err(_) => false,
            Ok(v) => v >= 2010 && v <= 2020,
        }
    }

    fn is_valid_expiration_year(s: &str) -> bool {
        match s.parse::<u32>() {
            Err(_) => false,
            Ok(v) => v >= 2020 && v <= 2030,
        }
    }

    fn is_valid_height(s: &str) -> bool {
        if s.len() < 3 {
            return false;
        }
        let value = match s[..s.len() - 2].parse::<u32>() {
            Err(_) => return false,
            Ok(v) => v,
        };
        let units = &s[s.len() - 2..];
        match units {
            "cm" => value >= 150 && value <= 193,
            "in" => value >= 59 && value <= 76,
            _ => false,
        }
    }

    fn is_valid_hair_color(s: &str) -> bool {
        if s.len() != 7 {
            return false;
        }
        let mut chars = s.chars();
        chars.next() == Some('#') && chars.all(|c| c.is_digit(16))
    }

    fn is_valid_eye_color(s: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
    }

    fn is_valid_passport_id(s: &str) -> bool {
        s.len() == 9 && s.chars().all(|c| c.is_digit(10))
    }

    fn is_valid_part_two(&self) -> bool {
        self.is_valid()
            && Self::is_valid_birth_year(self.birth_year.as_ref().unwrap())
            && Self::is_valid_issue_year(self.issue_year.as_ref().unwrap())
            && Self::is_valid_expiration_year(self.expiration_year.as_ref().unwrap())
            && Self::is_valid_height(self.height.as_ref().unwrap())
            && Self::is_valid_hair_color(self.hair_color.as_ref().unwrap())
            && Self::is_valid_eye_color(self.eye_color.as_ref().unwrap())
            && Self::is_valid_passport_id(self.passport_id.as_ref().unwrap())
    }
}
