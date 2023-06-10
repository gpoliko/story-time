pub fn services_lambda_generate_story_audio() -> String {
    "services_lambda_generate_story_audio".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(services_lambda_generate_story_audio(), "services_lambda_generate_story_audio".to_string());
    }
}
