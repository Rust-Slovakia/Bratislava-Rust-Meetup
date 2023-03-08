#[derive(Clone)]
pub struct Request {
    pub parseable_user: Option<ParseableUser>,
}

#[derive(Clone)]
pub struct ParseableUser {
    payload: u8,
}

impl ParseableUser {
    pub fn parse(self) -> Result<User, anyhow::Error> {
        if self.payload == 0 {
            Err(anyhow::anyhow!("user data invalid"))
        } else {
            if self.payload == 1 {
                Ok(User { display_name: None })
            } else {
                Ok(User {
                    display_name: Some(format!("{}", self.payload)),
                })
            }
        }
    }
}

#[derive(Clone)]
pub struct User {
    pub display_name: Option<String>,
}

mod procedural {
    use super::*;

    pub fn get_user_display_name(req: Request) -> Result<String, anyhow::Error> {
        if let Some(parseable_user) = req.parseable_user {
            if let Ok(user) = parseable_user.parse() {
                if let Some(display_name) = user.display_name {
                    Ok(display_name)
                } else {
                    Ok("unknown".to_string())
                }
            } else {
                Err(anyhow::anyhow!("failed to parse user"))
            }
        } else {
            Ok("unknown".to_string())
        }
    }
}

mod unwrap_or {
    use super::*;

    pub fn get_user_display_name(req: Request) -> Result<String, anyhow::Error> {
        if let Some(parseable_user) = req.parseable_user {
            if let Ok(user) = parseable_user.parse() {
                Ok(user.display_name.unwrap_or("unknown".to_string()))
            } else {
                Err(anyhow::anyhow!("failed to parse user"))
            }
        } else {
            Ok("unknown".to_string())
        }
    }
}

mod question_mark {
    use super::*;

    pub fn get_user_display_name(req: Request) -> Result<String, anyhow::Error> {
        if let Some(parseable_user) = req.parseable_user {
            let user = parseable_user.parse()?;
            Ok(user.display_name.unwrap_or("unknown".to_string()))
        } else {
            Ok("unknown".to_string())
        }
    }
}

mod functional {
    use super::*;

    pub fn get_user_display_name(req: Request) -> Result<String, anyhow::Error> {
        Ok(req
            .parseable_user
            .map(|pu| pu.parse())
            .transpose()?
            .and_then(|u| u.display_name)
            .unwrap_or("unknown".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TO_TEST: [fn(Request) -> Result<String, anyhow::Error>; 4] = [
        procedural::get_user_display_name,
        unwrap_or::get_user_display_name,
        question_mark::get_user_display_name,
        functional::get_user_display_name,
    ];

    #[test]
    fn absent_user() {
        let req = Request {
            parseable_user: None,
        };

        for to_test in TO_TEST {
            assert_eq!(to_test(req.clone()).unwrap(), "unknown".to_string())
        }
    }

    #[test]
    fn parse_error() {
        let req = Request {
            parseable_user: Some(ParseableUser { payload: 0 }),
        };

        for to_test in TO_TEST {
            assert!(to_test(req.clone()).is_err());
        }
    }

    #[test]
    fn parse_empty_name() {
        let req = Request {
            parseable_user: Some(ParseableUser { payload: 1 }),
        };

        for to_test in TO_TEST {
            assert_eq!(to_test(req.clone()).unwrap(), "unknown".to_string());
        }
    }

    #[test]
    fn parse_present_name() {
        let req = Request {
            parseable_user: Some(ParseableUser { payload: 42 }),
        };

        for to_test in TO_TEST {
            assert_eq!(to_test(req.clone()).unwrap(), "42".to_string());
        }
    }
}
