/// 🔎 Social Mentions and Hashtags Extractor
///
/// A lightweight utility for extracting unique `@mentions` and `#hashtags` from social media-style text.
///
/// Optimized for real-world use cases on platforms like YouTube, TikTok, Instagram, and Twitter.
///
/// # Overview
///
/// - Extracts both mentions (e.g., `@MrBeast`) and hashtags (e.g., `#fyp`) using regular expressions
/// - Ensures uniqueness with deduplication via `HashSet`
/// - Case-insensitive matching but preserves original casing
/// - Supports optional parsing (mentions-only, hashtags-only, or both)
pub mod mentions_hashtags {
    use regex::Regex;
    use std::collections::HashSet;
    use std::error::Error;

    /// Represents the result of parsing social text for mentions and hashtags.
    ///
    /// # Fields
    /// - `mentions`: A list of unique `@username` strings
    /// - `hashtags`: A list of unique `#topic` strings
    #[derive(Debug, Default)]
    pub struct MentionsHashtags {
        pub mentions: Vec<String>,
        pub hashtags: Vec<String>,
    }

    /// Parses the given description and extracts mentions and/or hashtags.
    ///
    /// # Arguments
    ///
    /// - `description`: The input text (e.g., social media caption or comment)
    /// - `mentions`: Whether to extract `@mentions`
    /// - `hashtags`: Whether to extract `#hashtags`
    ///
    /// # Returns
    /// A `Result` containing a `MentionsHashtags` struct with parsed values.
    ///
    /// # Behavior
    /// - If both `mentions` and `hashtags` are false, returns empty vectors.
    /// - Extracted values are **unique** and maintain original case.
    ///
    /// # Examples
    /// ```
    /// use mentions_hashtags_rs::mentions_hashtags::parse_mentions_hashtags;
    ///
    /// let text = "@MrBeast check out the #fyp and #Challenge2025!";
    /// let result = parse_mentions_hashtags(text, true, true).unwrap();
    /// assert!(result.mentions.contains(&"@MrBeast".to_string()));
    /// assert!(result.hashtags.contains(&"#fyp".to_string()));
    /// assert!(result.hashtags.contains(&"#Challenge2025".to_string()));
    /// ```
    pub fn parse_mentions_hashtags(
        description: &str,
        mentions: bool,
        hashtags: bool,
    ) -> Result<MentionsHashtags, Box<dyn Error>> {
        let mut mentions_hashtags = MentionsHashtags::default();

        if !mentions && !hashtags {
            return Ok(mentions_hashtags);
        }
        if mentions {
            mentions_hashtags.mentions = parse_mentions(description)?;
        }
        if hashtags {
            mentions_hashtags.hashtags = parse_hashtags(description)?;
        }

        Ok(mentions_hashtags)
    }

    /// Extracts unique `@mentions` from the input text.
    ///
    /// # Arguments
    /// - `description`: The input text (e.g., TikTok or YouTube description)
    ///
    /// # Returns
    /// A `Result` containing a `Vec<String>` of unique mentions.
    ///
    /// # Behavior
    /// - Matches alphanumeric usernames including `_`, `-`, and `.`
    /// - Preserves original casing (e.g., `@PewDiePie`, `@pewdiepie` both included if present)
    ///
    /// # Examples
    /// ```
    /// use mentions_hashtags_rs::mentions_hashtags::parse_mentions;
    ///
    /// let result = parse_mentions("@charlidamelio @Khaby.Lame").unwrap();
    /// assert!(result.contains(&"@charlidamelio".to_string()));
    /// assert!(result.contains(&"@Khaby.Lame".to_string()));
    /// ```
    pub fn parse_mentions(description: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let matches = Regex::new(r"(?i)@[a-zA-Z0-9_\-.]+")?;
        let unique_mentions: HashSet<String> = matches
            .find_iter(description)
            .map(|m| m.as_str().to_string())
            .collect();
        Ok(unique_mentions.into_iter().collect())
    }

    /// Extracts unique `#hashtags` from the input text.
    ///
    /// # Arguments
    /// - `description`: The input text (e.g., Instagram caption or Shorts comment)
    ///
    /// # Returns
    /// A `Result` containing a `Vec<String>` of unique hashtags.
    ///
    /// # Behavior
    /// - Matches alphanumeric hashtags including `_`, `-`, and `.`
    /// - Preserves original casing (e.g., `#Music` and `#music` both included if present)
    ///
    /// # Examples
    /// ```
    /// use mentions_hashtags_rs::mentions_hashtags::parse_hashtags;
    ///
    /// let result = parse_hashtags("#fyp #CapCut #go_crazy.").unwrap();
    /// assert!(result.contains(&"#CapCut".to_string()));
    /// assert!(result.contains(&"#go_crazy.".to_string()));
    /// ```
    pub fn parse_hashtags(description: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let matches = Regex::new(r"(?i)#[a-zA-Z0-9_\-.]+")?;
        let unique_hashtags: HashSet<String> = matches
            .find_iter(description)
            .map(|x| x.as_str().to_string())
            .collect();
        Ok(unique_hashtags.into_iter().collect())
    }
}


#[cfg(test)]
mod tests {
    use super::mentions_hashtags::*;
    use std::collections::HashSet;

    // === Mentions Tests ===
    #[test]
    fn test_youtube_mentions() {
        let result = parse_mentions("@MrBeast @EmmaChamberlain @PewDiePie").unwrap();
        let expected: HashSet<_> = ["@MrBeast", "@EmmaChamberlain", "@PewDiePie"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(result.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn test_tiktok_mentions_with_duplicates() {
        let result = parse_mentions("@charlidamelio @charlidamelio @Khaby.Lame").unwrap();
        let expected: HashSet<_> = ["@charlidamelio", "@Khaby.Lame"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(result.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn test_mentions_case_insensitive_but_preserved() {
        let result = parse_mentions("@AddisonRae @addisonrae").unwrap();
        assert!(result.contains(&"@AddisonRae".to_string()));
        assert!(result.contains(&"@addisonrae".to_string()));
        assert_eq!(result.len(), 2); // preserves case
    }

    #[test]
    fn test_mentions_empty_input() {
        let result = parse_mentions("").unwrap();
        assert!(result.is_empty());
    }

    // === Hashtags Tests ===
    #[test]
    fn test_tiktok_hashtags_trending() {
        let result = parse_hashtags("#fyp #trending #CapCut #viral").unwrap();
        let expected: HashSet<_> = ["#fyp", "#trending", "#CapCut", "#viral"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(result.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn test_youtube_hashtags_mixed_case() {
        let result = parse_hashtags("#Shorts #YouTubeShorts #Music #music").unwrap();
        let expected: HashSet<_> = ["#Shorts", "#YouTubeShorts", "#Music", "#music"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(result.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn test_hashtags_with_punctuation() {
        let result = parse_hashtags("Try this! #Challenge-2025 #fun.time #go_crazy.").unwrap();
        let expected: HashSet<_> = ["#Challenge-2025", "#fun.time", "#go_crazy."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(result.into_iter().collect::<HashSet<_>>(), expected);
    }

    #[test]
    fn test_hashtags_empty_input() {
        let result = parse_hashtags("").unwrap();
        assert!(result.is_empty());
    }

    // === Combined Parser ===
    #[test]
    fn test_parse_both_mentions_and_hashtags() {
        let result = parse_mentions_hashtags(
            "@MrBeast just posted a new video! #fyp #MrBeastChallenge",
            true,
            true,
        )
        .unwrap();

        assert!(result.mentions.contains(&"@MrBeast".to_string()));
        assert!(result.hashtags.contains(&"#fyp".to_string()));
        assert!(result.hashtags.contains(&"#MrBeastChallenge".to_string()));
    }

    #[test]
    fn test_parse_none_enabled() {
        let result = parse_mentions_hashtags("@Khaby.Lame #viral", false, false).unwrap();
        assert!(result.mentions.is_empty());
        assert!(result.hashtags.is_empty());
    }
}
