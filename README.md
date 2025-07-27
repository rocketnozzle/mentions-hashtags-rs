# 📣 mentions_hashtags

Extracts `@mentions` and `#hashtags` from text (like social media descriptions). Built in safe, fast, idiomatic Rust using `regex`.

## ✨ What it does

- 🧑‍💼 Pulls out all mentions (e.g. `@MrBeast`, `@DiorOfficial`)
- 🔖 Pulls out all hashtags (e.g. `#fyp`, `#LouisVuitton`)
- ♻️ Removes duplicates
- 🔡 Keeps original casing
- ⚙️ Works with common username formats (letters, numbers, `_`, `-`, `.`)

## 🚀 Example

```rust
use mentions_hashtags::mentions_hashtags::*;

let input = "@charlidamelio @GucciOfficial just posted! #fyp #CapCut #Chanel";
let result = parse_mentions_hashtags(input, true, true).unwrap();

assert_eq!(result.mentions, vec!["@charlidamelio", "@GucciOfficial"]);
assert!(result.hashtags.contains(&"#fyp".to_string()));
assert!(result.hashtags.contains(&"#Chanel".to_string()));
```

## 🛠️ Functions

### `parse_mentions_hashtags(description, mentions, hashtags) -> Result<MentionsHashtags, Box<dyn Error>>`

Parse both or either.

- ✅ Set `mentions = true` to extract `@user`s
- ✅ Set `hashtags = true` to extract `#tag`s

### `parse_mentions(description) -> Result<Vec<String>>`

Extract all `@user` names (no duplicates).

### `parse_hashtags(description) -> Result<Vec<String>>`

Extract all `#tags` (no duplicates).

## 📝 Notes

- ⚠️ Case-sensitive matching (but still deduplicated)
- 🕳️ Returns empty `Vec` if nothing found
- 🛡️ No panics
- 🔍 Uses `regex` and `HashSet` only

## 🧪 Testing

Run tests:

```bash
cargo test
```

Covers:

- 🎥 Instagram, TikTok and YouTube examples
- ♻️ Duplicates
- ✏️ Hashtags with punctuation
- 🈳 Empty input

## 📄 License

MIT


