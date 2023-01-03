use std::collections::HashSet;
use crate::open_api::PrimitiveType;

pub struct Comment {
    pub possible_types: HashSet<PrimitiveType>,
    pub name: String,
    pub required: Option<bool>,
    pub default: Option<String>,
}

impl Comment {
    /// Creates formatted string from the comment parameters
    ///
    /// # Examples
    ///
    /// ```
    /// let comment = Comment {
    ///     r#type: ValueType::String,
    ///     name: "id".to_owned(),
    ///     required: Some(false),
    ///     default: None,
    /// };
    ///
    /// let result = comment.get_formatted();
    /// assert_eq!("id?: String", result);
    /// ```
    fn get_formatted(&self) -> String {
        let mut name_with_optional_indicator = self.name.to_owned();
        if self.required.is_some() && !self.required.unwrap() {
            name_with_optional_indicator.push_str("?");
        }
        return format!(
            "{}: {}",
            name_with_optional_indicator,
            &self.possible_types.iter().map(|p_type| p_type.to_string()).collect::<Vec<String>>().join(",")
        );
    }
}

pub struct CommentsHolder {
    pub query: Vec<Comment>,
    pub parameters: Vec<Comment>,
    pub body: Vec<Comment>,
}

impl CommentsHolder {
    /// Creates formatted string from all non-zero length parameters (query, parameters, ..)
    /// # Examples
    /// ```
    /// let comment = Comment {
    ///     r#type: ValueType::String,
    ///     name: "id".to_owned(),
    ///     required: Some(false),
    ///     default: None,
    /// };
    ///
    /// let holder = CommentsHolder {
    ///     query: Vec::from([comment]),
    ///     parameters: Vec::new(),
    ///     body: Vec::new(),
    /// }
    ///
    /// // this would return the following string
    /// // # Parameters
    /// // #    - id?: String
    /// // #
    /// let result = holder.get_formatted();
    /// ```
    pub fn get_formatted(&self) -> String {
        let mut output: Vec<String> = Vec::new();

        if self.query.len() > 0 {
            output.push(get_formatted_comment(&self.query, &"Query".to_owned()));
        }

        if self.parameters.len() > 0 {
            output.push(get_formatted_comment(
                &self.parameters,
                &"Parameters".to_owned(),
            ));
        }

        if self.body.len() > 0 {
            output.push(get_formatted_comment(&self.body, &"Body".to_owned()));
        }

        return output.join("\n");
    }
}

/// Creates formatted string from provided comment vector
///
/// # Examples
/// ```
/// let comment = Comment {
///     r#type: ValueType::String,
///     name: "id".to_owned(),
///     required: Some(false),
///     default: None,
/// };
/// let location = "Parameters".to_owned();
///
/// // this would return the following string
/// // # Parameters
/// // #    - id?: String
/// // #
/// let formatted_comment = get_formatted_comment(&Vec::from([comment]), &location);
/// ```
fn get_formatted_comment(value: &Vec<Comment>, location: &String) -> String {
    let query: Vec<String> = value
        .iter()
        .map(|comment| format!("#  - {}\n", &comment.get_formatted()))
        .collect();

    return format!("# {}\n{}#", location, &query.join(""));
}
