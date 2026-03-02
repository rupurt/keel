//! Global story ID generation using Crockford Base62 encoding
//!
//! Generates 9-character IDs: 6 chars timestamp + 3 chars random.
//! IDs are lexicographically sortable by creation time.

use std::time::{SystemTime, UNIX_EPOCH};

/// Crockford Base62 alphabet - ordered for lexicographic sortability
/// Digits < uppercase < lowercase in ASCII, so sorted strings = sorted times
const CROCKFORD_BASE62: &[u8; 62] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Generate a new globally unique story ID
///
/// Format: 9 characters (6 timestamp + 3 random)
/// - First 6 chars: seconds since Unix epoch encoded in base62
/// - Last 3 chars: random value 0..238327 encoded in base62
///
/// IDs are lexicographically sortable by creation time.
pub fn generate_story_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    generate_story_id_with_timestamp(timestamp)
}

/// Generate a story ID with a specific timestamp (for migration/testing)
pub fn generate_story_id_with_timestamp(timestamp: u64) -> String {
    let random: u32 = rand_u32() % (62 * 62 * 62); // 0..238327

    let mut id = encode_base62(timestamp, 6);
    id.push_str(&encode_base62(random as u64, 3));
    id
}

/// Encode a number to base62 with fixed width (zero-padded)
pub fn encode_base62(mut value: u64, width: usize) -> String {
    let mut chars = vec![b'0'; width];

    for i in (0..width).rev() {
        chars[i] = CROCKFORD_BASE62[(value % 62) as usize];
        value /= 62;
    }

    String::from_utf8(chars).expect("Base62 chars are valid UTF-8")
}

/// Decode a base62 string back to a number
#[allow(dead_code)] // Utility for debugging/migration
pub fn decode_base62(s: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for c in s.chars() {
        let digit = match c {
            '0'..='9' => (c as u64) - ('0' as u64),
            'A'..='Z' => (c as u64) - ('A' as u64) + 10,
            'a'..='z' => (c as u64) - ('a' as u64) + 36,
            _ => return None,
        };
        result = result.checked_mul(62)?.checked_add(digit)?;
    }

    Some(result)
}

/// Extract the timestamp from a story ID (first 6 chars)
#[allow(dead_code)] // Utility for debugging/migration
pub fn extract_timestamp(id: &str) -> Option<u64> {
    if id.len() < 6 {
        return None;
    }
    decode_base62(&id[..6])
}

/// Simple xorshift random number generator
/// Uses global atomic counter + thread-local state for uniqueness
fn rand_u32() -> u32 {
    use std::sync::atomic::{AtomicU64, Ordering};

    // Global counter ensures uniqueness even when called from multiple threads
    // at the exact same nanosecond
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    // Get a unique value by combining time and counter
    let time_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x12345678);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);

    // Mix time and counter with SplitMix64-style mixing
    let mut s = time_ns.wrapping_add(count.wrapping_mul(0x9E3779B97F4A7C15));
    s = (s ^ (s >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    s = (s ^ (s >> 27)).wrapping_mul(0x94D049BB133111EB);
    s ^= s >> 31;

    s as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn generate_story_id_returns_9_chars() {
        let id = generate_story_id();
        assert_eq!(id.len(), 9, "ID should be 9 characters: {}", id);
    }

    #[test]
    fn generate_story_id_uses_valid_alphabet() {
        let id = generate_story_id();
        for c in id.chars() {
            assert!(
                c.is_ascii_alphanumeric(),
                "Character '{}' is not alphanumeric",
                c
            );
        }
    }

    #[test]
    fn encode_base62_roundtrips() {
        let cases = [0u64, 1, 61, 62, 100, 1000, 1_000_000, u64::MAX / 2];
        for &value in &cases {
            let encoded = encode_base62(value, 11); // 11 chars can hold u64::MAX / 2
            let decoded = decode_base62(&encoded).unwrap();
            assert_eq!(decoded, value, "Roundtrip failed for {}", value);
        }
    }

    #[test]
    fn encode_base62_fixed_width() {
        assert_eq!(encode_base62(0, 3), "000");
        assert_eq!(encode_base62(1, 3), "001");
        assert_eq!(encode_base62(61, 3), "00z");
        assert_eq!(encode_base62(62, 3), "010");
    }

    #[test]
    fn decode_base62_handles_invalid_input() {
        assert!(decode_base62("abc!").is_none());
        assert!(decode_base62("ab c").is_none());
        assert!(decode_base62("").is_some()); // Empty string = 0
    }

    #[test]
    fn generated_ids_are_lexicographically_sortable() {
        // Generate IDs with increasing timestamps
        let timestamps = [1000000u64, 1000001, 1000002, 1000100, 2000000];
        let ids: Vec<String> = timestamps
            .iter()
            .map(|&t| generate_story_id_with_timestamp(t))
            .collect();

        // IDs should already be in sorted order
        let mut sorted = ids.clone();
        sorted.sort();

        for i in 0..ids.len() {
            // The timestamp portion (first 6 chars) should be sorted
            assert!(
                ids[i][..6] == sorted[i][..6],
                "Timestamp portions should sort correctly"
            );
        }
    }

    #[test]
    fn extract_timestamp_works() {
        let timestamp = 1706400000u64; // Some arbitrary timestamp
        let id = generate_story_id_with_timestamp(timestamp);
        let extracted = extract_timestamp(&id).unwrap();
        assert_eq!(extracted, timestamp);
    }

    #[test]
    fn ids_are_unique_across_100_generations() {
        // With 238,328 possible random values per second,
        // generating 100 IDs has very low collision probability
        // (birthday paradox: n²/2m = 100²/(2×238328) ≈ 0.002%)
        // This tests realistic CLI usage (multiple stories created quickly)
        let mut seen = HashSet::new();
        for _ in 0..100 {
            let id = generate_story_id();
            assert!(seen.insert(id.clone()), "Duplicate ID generated: {}", id);
        }
    }

    #[test]
    fn random_suffix_produces_varied_output() {
        // Verify the random suffix varies across calls
        let ts = 1700000000u64;
        let ids: Vec<String> = (0..100)
            .map(|_| generate_story_id_with_timestamp(ts))
            .collect();

        // Extract just the random suffix (last 3 chars)
        let suffixes: HashSet<&str> = ids.iter().map(|id| &id[6..]).collect();

        // Should have high variety - at least 90 unique suffixes out of 100
        assert!(
            suffixes.len() >= 90,
            "Random suffixes should vary: got {} unique out of 100",
            suffixes.len()
        );
    }

    #[test]
    fn crockford_alphabet_is_lexicographically_ordered() {
        // Verify the alphabet is in correct ASCII order for sorting
        let alphabet: Vec<char> = CROCKFORD_BASE62.iter().map(|&b| b as char).collect();
        for i in 1..alphabet.len() {
            assert!(
                alphabet[i] > alphabet[i - 1],
                "Alphabet not sorted: {} should come after {}",
                alphabet[i],
                alphabet[i - 1]
            );
        }
    }

    #[test]
    fn timestamp_range_is_sufficient() {
        // 6 base62 chars = 62^6 = 56,800,235,584 seconds
        // That's about 1,800 years from epoch
        let max_timestamp = 62u64.pow(6) - 1;
        let years = max_timestamp / (365 * 24 * 60 * 60);
        assert!(
            years > 1700,
            "Timestamp range should cover >1700 years, got {}",
            years
        );
    }

    #[test]
    fn random_suffix_capacity() {
        // 3 base62 chars = 62^3 = 238,328 possibilities per second
        let capacity = 62u64.pow(3);
        assert_eq!(capacity, 238328);
    }
}
