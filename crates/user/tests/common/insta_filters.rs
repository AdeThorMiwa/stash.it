pub mod redactions {
    use std::sync::OnceLock;

    static CLEANUP_DATE: OnceLock<Vec<(&'static str, &'static str)>> = OnceLock::new();
    static CLEANUP_MODEL: OnceLock<Vec<(&'static str, &'static str)>> = OnceLock::new();

    pub fn get_cleanup_ids() -> &'static Vec<(&'static str, &'static str)> {
        CLEANUP_MODEL.get_or_init(|| vec![(r"([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})", "PID")])
    }

    pub fn get_cleanup_date() -> &'static Vec<(&'static str, &'static str)> {
        CLEANUP_DATE.get_or_init(|| {
            vec![
                (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?\+\d{2}:\d{2}", "DATE"), // with tz
                (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+", "DATE"),
                (r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})", "DATE"),
            ]
        })
    }

    pub fn cleanup_model_generics() -> Vec<(&'static str, &'static str)> {
        let mut combined_filters = vec![];
        combined_filters.extend(get_cleanup_ids().iter().copied());
        combined_filters.extend(get_cleanup_date().iter().copied());
        combined_filters
    }
}
