use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::common;

struct Passport {
    fields: HashMap<String, String>
}

impl Passport {
    fn from_file(filename: &str) -> Vec<Passport> {
        let lines = common::read_strings(filename);
        let mut passports = Vec::new();
        let mut entry = "".to_string();
        for line in lines {
            if line.trim().is_empty() {
                passports.push(Passport::parse(&entry));
                entry = "".to_string();
            } else {
                entry.push_str(&line.trim());
                entry.push_str(" ");
            }
        }
        passports.push(Passport::parse(&entry));
        return passports;
    }

    fn parse(string: &String) -> Passport {
        let field_strings = string.trim().split_whitespace();
        let mut fields = HashMap::new();
        for field_string in field_strings {
            let field_parts = field_string.split(":").collect::<Vec<&str>>();
            fields.insert(field_parts[0].to_string(), field_parts[1].to_string());
        }
        return Passport { fields };
    }

    fn fields_present(&self) -> bool {
        let mandatory_fields = vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid");
        let optional_fields = vec!("cid");

        let mut copy = HashMap::new();
        copy.extend(self.fields.iter());

        for m in mandatory_fields {
            match copy.remove(&m.to_string()) {
                Some(_v) => {}
                None => { return false; }
            }
        }
        for o in optional_fields {
            copy.remove(&o.to_string());
        }
        return copy.len() == 0;
    }

    fn fields_valid(&self) -> bool {
        for field in &self.fields {
            if !self.field_valid(field.0, field.1) {
                return false;
            }
        }
        return true;
    }

    fn field_valid(&self, key: &String, value: &String) -> bool {
        lazy_static! {
            static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
            static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref HGT_CM: Regex = Regex::new(r"^(\d{3})cm$").unwrap();
            static ref HGT_IN: Regex = Regex::new(r"^(\d{2})in$").unwrap();
        }

        return match key.as_str() {
            "byr" => { Passport::is_number_between(value, 1920, 2002) }
            "iyr" => { Passport::is_number_between(value, 2010, 2020) }
            "eyr" => { Passport::is_number_between(value, 2020, 2030) }
            "hgt" => {
                if HGT_CM.is_match(value) {
                    Passport::is_number_between(&HGT_CM.captures(value).unwrap()[1].to_string(), 150, 193)
                } else if HGT_IN.is_match(value) {
                    Passport::is_number_between(&HGT_IN.captures(value).unwrap()[1].to_string(), 59, 76)
                } else {
                    false
                }
            }
            "pid" => { PID.is_match(value) }
            "hcl" => { HCL.is_match(value) }
            "ecl" => { ECL.is_match(value) }
            "cid" => { true }
            _ => { false }
        };
    }

    fn is_number_between(string: &String, min: i32, max: i32) -> bool {
        return match string.parse::<i32>() {
            Ok(n) => { n >= min && n <= max }
            Err(_e) => { false }
        };
    }
}

pub fn part_one() {
    println!("--- Part One ---");

    let passports = Passport::from_file("./data/dec_04.txt");
    let result = passports.iter()
        .filter(|p| p.fields_present())
        .count();
    println!("Result: {}", result)
}

pub fn part_two() {
    println!("--- Part Two ---");

    let passports = Passport::from_file("./data/dec_04.txt");
    let result = passports.iter()
        .filter(|p| p.fields_present())
        .filter(|p| p.fields_valid())
        .count();
    println!("Result: {}", result)
}
