use chrono::Duration;

#[derive(PartialEq, Default)]
struct Parsed {
    pub seconds: Option<u32>,
    pub minutes: Option<u32>,
    pub hours: Option<u32>,
}

impl Parsed {
    fn parse_seconds_count(s: &str) -> Result<Self, ()> {
        match s.parse::<u32>() {
            Ok(u) => Ok(Parsed {
                seconds: Some(u % 60),
                minutes: Some((u % 3600) / 60),
                hours: Some(u / 3600),
            }),
            Err(_) => Err(()),
        }
    }

    fn parse_duration_with_colon_separator(s: &str) -> Result<Self, ()> {
        let split = s.split(':').collect::<Vec<&str>>();

        match split.len() {
            2 => {
                let seconds = split[1].parse::<u32>();
                let minutes = split[0].parse::<u32>();
                if seconds.is_err()
                    || *seconds.as_ref().unwrap() >= 60
                    || minutes.is_err()
                    || *minutes.as_ref().unwrap() >= 60
                {
                    return Err(());
                }
                Ok(Parsed {
                    seconds: Some(seconds.unwrap()),
                    minutes: Some(minutes.unwrap()),
                    hours: None,
                })
            }
            3 => {
                let seconds = split[2].parse::<u32>();
                let minutes = split[1].parse::<u32>();
                let hours = split[0].parse::<u32>();
                if seconds.is_err()
                    || *seconds.as_ref().unwrap() >= 60
                    || minutes.is_err()
                    || *minutes.as_ref().unwrap() >= 60
                    || hours.is_err()
                {
                    return Err(());
                }
                Ok(Parsed {
                    seconds: Some(seconds.unwrap()),
                    minutes: Some(minutes.unwrap()),
                    hours: Some(hours.unwrap()),
                })
            }
            _ => Err(()),
        }
    }

    fn parse_duration_with_letters_separators(s: &str) -> Result<Self, ()> {
        let character_separators: Vec<Vec<char>> = vec![
            vec!['s'],
            vec!['m'],
            vec!['h'],
            vec!['s', 'm'],
            vec!['m', 'h'],
            vec!['s', 'm', 'h'],
        ];
        if !character_separators
            .iter()
            .any(|v| v.iter().all(|c| s.contains(*c)))
        {
            return Err(());
        }

        let last_character: char = s.chars().last().unwrap();

        let mut split = s.split(|c: char| !c.is_numeric()).collect::<Vec<&str>>();
        split.pop();
        if split.iter().any(|s| s.is_empty()) {
            return Err(());
        }

        let parsed_numbers: Vec<u32> = split.iter().fold(vec![], |mut accum, s| -> Vec<u32> {
            accum.push(s.parse::<u32>().unwrap());
            accum
        });
        if parsed_numbers.iter().any(|i| *i >= 60) {
            return Err(());
        }

        match parsed_numbers.len() {
            1 => Ok(Parsed {
                seconds: if last_character == 's' {
                    Some(parsed_numbers[0])
                } else {
                    None
                },
                minutes: if last_character == 'm' {
                    Some(parsed_numbers[0])
                } else {
                    None
                },
                hours: if last_character == 'h' {
                    Some(parsed_numbers[0])
                } else {
                    None
                },
            }),
            2 => Ok(Parsed {
                seconds: if last_character == 's' {
                    Some(parsed_numbers[1])
                } else {
                    None
                },
                minutes: if last_character == 's' {
                    Some(parsed_numbers[0])
                } else {
                    Some(parsed_numbers[1])
                },
                hours: if last_character == 'm' {
                    Some(parsed_numbers[1])
                } else {
                    None
                },
            }),
            3 => Ok(Parsed {
                seconds: Some(parsed_numbers[2]),
                minutes: Some(parsed_numbers[1]),
                hours: Some(parsed_numbers[0]),
            }),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Parsed {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.parse::<u32>().is_ok() {
            Parsed::parse_seconds_count(s)
        } else if s.contains(':') {
            Parsed::parse_duration_with_colon_separator(s)
        } else {
            Parsed::parse_duration_with_letters_separators(s)
        }
    }
}
pub fn string_to_duration(s: &str) -> Result<Duration, ()> {
    let parsed = Parsed::try_from(s);

    if let Ok(p) = parsed {
        let mut seconds = p.seconds.unwrap_or_default();
        seconds += p.minutes.unwrap_or_default() * 60;
        seconds += p.hours.unwrap_or_default() * 3600;
        return Ok(Duration::seconds(seconds.into()));
    }

    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("shdiedj2;3m" => Err(()))]
    #[test_case("3600" => Ok(Duration::seconds(3600)))]
    #[test_case("3711" => Ok(Duration::seconds(3711)))]
    #[test_case("01h1m02s" => Ok(Duration::seconds(3662)))]
    #[test_case("10m02s" => Ok(Duration::seconds(602)))]
    #[test_case("42s" => Ok(Duration::seconds(42)))]
    #[test_case("01h00m01s" => Ok(Duration::seconds(3601)))]
    #[test_case("2m" => Ok(Duration::seconds(120)))]
    #[test_case("02m" => Ok(Duration::seconds(120)))]
    #[test_case("02h" => Ok(Duration::seconds(7200)))]
    #[test_case("02:11" => Ok(Duration::seconds(131)))]
    #[test_case("00:02:11" => Ok(Duration::seconds(131)))]
    #[test_case("1:0:1" => Ok(Duration::seconds(3601)))]
    #[test_case("1:10:91" => Err(()))]
    #[test_case("1:69:00" => Err(()))]
    #[test_case("1:29;00" => Err(()))]
    #[test_case("1:29;0s" => Err(()))]
    #[test_case("shdiedj2;3m:2s" => Err(()))]
    #[test_case("01h00m61s" => Err(()))]
    #[test_case("59m61s" => Err(()))]
    #[test_case("60h" => Err(()))]
    fn test_string_to_duration(s: &str) -> Result<Duration, ()> {
        string_to_duration(s)
    }
}
