#[cfg(all(test, feature = "language"))]
mod tests {
    use emixai::{SourceSize, language::openai::OpenAiSource};

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
    fn test_openai_source_default() {
        assert_eq!(OpenAiSource::default(), OpenAiSource::gpt_4o_mini);
    }

    #[test]
    fn test_openai_source_display() {
        assert_eq!(OpenAiSource::gpt_3_5_turbo.to_string(), "gpt-3.5-turbo");
        assert_eq!(OpenAiSource::gpt_4o_mini.to_string(), "gpt-4o-mini");
        assert_eq!(OpenAiSource::gpt_4o.to_string(), "gpt-4o");
        assert_eq!(OpenAiSource::gpt_4o_2024_08_06.to_string(), "gpt-4o-2024-08-06");
        assert_eq!(OpenAiSource::gpt_4.to_string(), "gpt-4");
        assert_eq!(OpenAiSource::gpt_4_turbo.to_string(), "gpt-4-turbo");
        assert_eq!(OpenAiSource::gpt_4_turbo_preview.to_string(), "gpt-4-turbo-preview");
        assert_eq!(OpenAiSource::o1_mini.to_string(), "o1-mini");
        assert_eq!(OpenAiSource::o1_preview.to_string(), "o1-preview");
        assert_eq!(OpenAiSource::o3_mini.to_string(), "o3-mini");
    }

    #[test]
    fn test_openai_source_from_source_size() {
        assert_eq!(
            OpenAiSource::from(SourceSize::Tiny),
            OpenAiSource::gpt_4o_mini
        );
        assert_eq!(OpenAiSource::from(SourceSize::Small), OpenAiSource::gpt_4o);
        assert_eq!(OpenAiSource::from(SourceSize::Base), OpenAiSource::gpt_4o);
        assert_eq!(
            OpenAiSource::from(SourceSize::Medium),
            OpenAiSource::gpt_4o_2024_08_06
        );
        assert_eq!(
            OpenAiSource::from(SourceSize::Large),
            OpenAiSource::gpt_4_turbo
        );
    }
}
