use serde::{Deserialize, Serialize};

pub mod completion;
pub mod edits;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(untagged)]
pub enum StringOrListParam {
    StringParam(String),
    ListParam(Vec<String>),
}

impl Clone for StringOrListParam {
    fn clone(&self) -> Self {
        match self {
            StringOrListParam::StringParam(str) => StringOrListParam::StringParam(str.clone()),
            StringOrListParam::ListParam(list) => StringOrListParam::ListParam(list.clone()),
        }
    }
}

impl From<&str> for StringOrListParam {
    fn from(value: &str) -> Self {
        StringOrListParam::StringParam(value.to_string())
    }
}

impl From<Vec<&str>> for StringOrListParam {
    fn from(value: Vec<&str>) -> Self {
        StringOrListParam::ListParam(value.iter().map(|s| s.to_string()).collect())
    }
}

impl From<&Vec<&str>> for StringOrListParam {
    fn from(value: &Vec<&str>) -> Self {
        StringOrListParam::ListParam(value.iter().map(|s| s.to_string()).collect())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: Option<i64>,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextResult {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: Option<String>,
    pub choices: Vec<TextChoice>,
    pub usage: Usage,
}

/// A choice result for text based operations
#[derive(Serialize, Deserialize, Debug)]
pub struct TextChoice {
    pub text: String,
    pub index: i64,
    pub logprobs: Option<i64>,
    pub finish_reason: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::requests::StringOrListParam::{ListParam, StringParam};
    use serde_json::Value;

    #[test]
    fn must_serde_string() {
        let test: StringOrListParam = StringParam("test_string".to_string());
        let value: Value = serde_json::to_value(&test).unwrap();
        let res: StringOrListParam = serde_json::from_value(value).unwrap();
        assert_eq!(test, res);
    }

    #[test]
    fn must_serde_list() {
        let test: StringOrListParam =
            ListParam(vec!["test_string".to_string(), "test_string2".to_string()]);
        let value: Value = serde_json::to_value(&test).unwrap();
        let res: StringOrListParam = serde_json::from_value(value).unwrap();
        match res {
            StringParam(_) => assert!(false),
            ListParam(ref list) => assert_eq!(list.len(), 2),
        }
        assert_eq!(test, res);
    }
}
