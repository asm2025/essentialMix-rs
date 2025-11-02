#[cfg(all(test, feature = "language"))]
mod tests {
    use emixai::{SourceSize, language::OpenAiSource};

    #[test]
    fn test_source_size_default() {
        assert_eq!(SourceSize::default(), SourceSize::Small);
    }

    #[test]
    fn test_source_size_clone() {
        let size = SourceSize::Tiny;
        let cloned = size;
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_source_size_copy() {
        let size = SourceSize::Medium;
        let copied = size;
        assert_eq!(size, copied); // Copy trait allows this
        assert_eq!(size, SourceSize::Medium);
    }

    #[test]
    fn test_source_size_partial_eq() {
        assert_eq!(SourceSize::Tiny, SourceSize::Tiny);
        assert_ne!(SourceSize::Tiny, SourceSize::Small);
    }

    #[test]
    fn test_source_size_partial_ord() {
        assert!(SourceSize::Tiny < SourceSize::Small);
        assert!(SourceSize::Small < SourceSize::Base);
        assert!(SourceSize::Base < SourceSize::Medium);
        assert!(SourceSize::Medium < SourceSize::Large);
    }

    #[test]
    fn test_source_size_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(SourceSize::Tiny, "tiny");
        map.insert(SourceSize::Small, "small");

        assert_eq!(map.get(&SourceSize::Tiny), Some(&"tiny"));
        assert_eq!(map.get(&SourceSize::Small), Some(&"small"));
        assert_eq!(map.get(&SourceSize::Base), None);
    }

    #[test]
    fn test_openai_source_default() {
        assert_eq!(OpenAiSource::default(), OpenAiSource::gpt_4o_mini);
    }

    #[test]
    fn test_openai_source_display() {
        assert_eq!(OpenAiSource::gpt_3_5_turbo.to_string(), "gpt-3.5-turbo");
        assert_eq!(OpenAiSource::gpt_4o_mini.to_string(), "gpt-4o-mini");
        assert_eq!(OpenAiSource::gpt_4o.to_string(), "gpt-4o");
        assert_eq!(OpenAiSource::gpt_4.to_string(), "gpt-4");
        assert_eq!(OpenAiSource::gpt_4_turbo.to_string(), "gpt-4-turbo");
        assert_eq!(OpenAiSource::o1_mini.to_string(), "o1-mini");
    }

    #[test]
    fn test_openai_source_from_source_size() {
        assert_eq!(
            OpenAiSource::from(SourceSize::Tiny),
            OpenAiSource::gpt_4o_mini
        );
        assert_eq!(OpenAiSource::from(SourceSize::Small), OpenAiSource::gpt_4o);
        assert_eq!(OpenAiSource::from(SourceSize::Base), OpenAiSource::gpt_4o);
        assert_eq!(OpenAiSource::from(SourceSize::Medium), OpenAiSource::gpt_4);
        assert_eq!(
            OpenAiSource::from(SourceSize::Large),
            OpenAiSource::gpt_4_turbo
        );
    }

    #[test]
    fn test_openai_source_clone() {
        let source = OpenAiSource::gpt_4o;
        let cloned = source;
        assert_eq!(source, cloned);
    }

    #[test]
    fn test_openai_source_copy() {
        let source = OpenAiSource::gpt_4;
        let copied = source;
        assert_eq!(source, copied);
        assert_eq!(source, OpenAiSource::gpt_4);
    }

    #[test]
    fn test_openai_source_partial_eq() {
        assert_eq!(OpenAiSource::gpt_4o_mini, OpenAiSource::gpt_4o_mini);
        assert_ne!(OpenAiSource::gpt_4o_mini, OpenAiSource::gpt_4o);
    }

    #[test]
    fn test_openai_source_ord() {
        assert!(OpenAiSource::gpt_3_5_turbo < OpenAiSource::gpt_4o_mini);
        assert!(OpenAiSource::gpt_4o_mini < OpenAiSource::gpt_4o);
        assert!(OpenAiSource::gpt_4o < OpenAiSource::gpt_4);
    }

    #[test]
    fn test_openai_source_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(OpenAiSource::gpt_4o_mini, "mini");
        map.insert(OpenAiSource::gpt_4o, "o");

        assert_eq!(map.get(&OpenAiSource::gpt_4o_mini), Some(&"mini"));
        assert_eq!(map.get(&OpenAiSource::gpt_4o), Some(&"o"));
        assert_eq!(map.get(&OpenAiSource::gpt_4), None);
    }
}
