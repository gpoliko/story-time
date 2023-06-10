pub fn services_lambda_generate_story_images() -> String {
    "services_lambda_generate_story_images".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(services_lambda_generate_story_images(), "services_lambda_generate_story_images".to_string());
    }
}
