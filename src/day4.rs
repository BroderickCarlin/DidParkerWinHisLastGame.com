struct Passport<'a> {
    birth_year: Option<u16>,      //byr
    issue_year: Option<u16>,      //iyr
    expiration_year: Option<u16>, //eyr
    height: Option<&'a str>,     //hgt
    hair_color: Option<&'a str>,  //hcl
    eye_color: Option<&'a str>,   //ecl
    passport_id: Option<&'a str>, //pid
    county_id: Option<&'a str>,  //cid
}

impl<'a> Passport<'a> {
    fn from_input(input: &'a str) -> Passport<'a> {
        let mut p = Passport {birth_year: None, issue_year: None, expiration_year: None, height: None, hair_color: None, eye_color: None, passport_id: None, county_id: None};
        for val in input.match_indices(':') {
            match &input[val.0-3..val.0] {
                "byr" => {
                    p.birth_year = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap().parse().unwrap());
                }, 
                "iyr" => {
                    p.issue_year = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap().parse().unwrap());
                }, 
                "eyr" => {
                    p.expiration_year = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap().parse().unwrap());
                }, 
                "hgt" => {
                    p.height = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap());
                }, 
                "hcl" => {
                    p.hair_color = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap());
                }, 
                "ecl" => {
                    p.eye_color = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap());
                }, 
                "pid" => {
                    p.passport_id = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap());
                }, 
                "cid" => {
                    p.county_id = Some(input[val.0+1..].split(" ").next().unwrap().split("\n").next().unwrap());
                },
                _ => {
                    // 
                }
            }
        }
        p
    }

    fn is_complete(&self) -> bool {
        self.birth_year.is_some() &&
        self.issue_year.is_some() &&
        self.expiration_year.is_some() &&
        self.height.is_some() &&
        self.hair_color.is_some() &&
        self.eye_color.is_some() &&
        self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        if let Some(year) = self.birth_year {
            if year < 1920 || year > 2002 {
                return  false;
            }
        } else {
            return false;
        }

        if let Some(year) = self.issue_year {
            if year < 2010 || year > 2020 {
                return  false;
            }
        } else {
            return false;
        }

        if let Some(year) = self.expiration_year {
            if year < 2020 || year > 2030 {
                return  false;
            }
        } else {
            return false;
        }

        if let Some(height) = self.height {
            if height.ends_with("cm") {
                if let Ok(height) = height[..(height.len()-2)].parse::<u32>() {
                    if height < 150 || height > 193 {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if height.ends_with("in") {
                if let Ok(height) = height[..(height.len()-2)].parse::<u32>() {
                    if height < 59 || height > 76 {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        if let Some(color) = self.hair_color {
            if color.chars().next() != Some('#') {
                return false;
            }
            if u64::from_str_radix(&color[1..], 16).is_err() {
                return false;
            }
        } else {
            return false;
        }

        if let Some(color) = self.eye_color {
           match color {
               "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
               _ => {
                   return false;
               }
           }
        } else {
            return false;
        }

        if let Some(id) = self.passport_id {
            if id.len() != 9 {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut count = 0;
    for p in input.split("\n\n") {
        let passport = Passport::from_input(p);
        if passport.is_complete() {
            count += 1;
        }
    };
    count
}

pub fn puzzle2(input: &str) -> i64 {
    let mut count = 0;
    for p in input.split("\n\n") {
        let passport = Passport::from_input(p);
        if passport.is_valid() {
            count += 1;
        }
    };
    count
}
