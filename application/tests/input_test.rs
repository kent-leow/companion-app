mod input_parser {
    fn parse(input: &str) -> (Vec<String>, bool) {
        let mut tags = Vec::new();
        let mut has_image = false;

        for word in input.split_whitespace() {
            if let Some(path) = word.strip_prefix('@') {
                if path.is_empty() {
                    continue;
                }
                let is_image = path.ends_with(".png")
                    || path.ends_with(".jpg")
                    || path.ends_with(".jpeg")
                    || path.ends_with(".gif")
                    || path.ends_with(".webp");

                if is_image {
                    has_image = true;
                }
                tags.push(path.to_string());
            }
        }
        (tags, has_image)
    }

    #[test]
    fn extracts_file_tags() {
        let (tags, has_image) = parse("explain @src/main.rs please");
        assert_eq!(tags, vec!["src/main.rs"]);
        assert!(!has_image);
    }

    #[test]
    fn extracts_multiple_tags() {
        let (tags, _) = parse("compare @src/a.rs and @src/b.rs");
        assert_eq!(tags, vec!["src/a.rs", "src/b.rs"]);
    }

    #[test]
    fn detects_image_tag() {
        let (tags, has_image) = parse("describe @screenshot.png");
        assert_eq!(tags, vec!["screenshot.png"]);
        assert!(has_image);
    }

    #[test]
    fn detects_directory_tag() {
        let (tags, _) = parse("list @src/");
        assert_eq!(tags, vec!["src/"]);
    }

    #[test]
    fn ignores_bare_at_symbol() {
        let (tags, _) = parse("send @ email");
        assert!(tags.is_empty());
    }

    #[test]
    fn no_tags_in_plain_text() {
        let (tags, has_image) = parse("just a normal question");
        assert!(tags.is_empty());
        assert!(!has_image);
    }
}
