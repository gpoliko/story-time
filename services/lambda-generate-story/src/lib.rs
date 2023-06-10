pub fn services_lambda_generate_story() -> String {
    "services_lambda_generate_story".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(services_lambda_generate_story(), "services_lambda_generate_story".to_string());
    }
}
