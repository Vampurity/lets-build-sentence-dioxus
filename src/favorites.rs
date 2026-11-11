use crate::banks::known_sentence;

pub fn toggle(ids: &[u32], sentence_id: u32) -> (Vec<u32>, bool) {
    if let Some(index) = ids.iter().position(|id| *id == sentence_id) {
        let mut next = ids.to_vec();
        next.remove(index);
        (next, false)
    } else {
        let mut next = ids.to_vec();
        next.push(sentence_id);
        (next, true)
    }
}

pub fn clamp_index(index: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        index.min(len - 1)
    }
}

pub fn sanitize(ids: &[u32]) -> Vec<u32> {
    let mut next = Vec::new();
    for id in ids {
        if known_sentence(*id) && !next.contains(id) {
            next.push(*id);
        }
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::banks::{collection_sentences, catalog};

    #[test]
    fn toggle_appends_then_removes_without_reordering() {
        let (ids, on) = toggle(&[], 5);
        assert!(on);
        assert_eq!(ids, vec![5]);
        let (ids, on) = toggle(&[1, 5, 9], 5);
        assert!(!on);
        assert_eq!(ids, vec![1, 9]);
        let (ids, on) = toggle(&[1, 9], 301);
        assert!(on);
        assert_eq!(ids, vec![1, 9, 301]);
    }

    #[test]
    fn collection_keeps_favorite_order_and_renumbers() {
        let sentences = collection_sentences(&[801, 1]);
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[0].id, 801);
        assert_eq!(sentences[0].number, 1);
        assert_eq!(sentences[0].chinese, catalog().sentence(801).unwrap().chinese);
        assert!(sentences[0].favorite);
        assert_eq!(sentences[1].id, 1);
        assert_eq!(sentences[1].number, 2);
        assert_eq!(sentences[1].english, "I like apples.");
    }

    #[test]
    fn unfavorite_clamps_the_current_index() {
        assert_eq!(clamp_index(2, 2), 1);
        assert_eq!(clamp_index(0, 0), 0);
        assert_eq!(clamp_index(1, 3), 1);
    }

    #[test]
    fn sanitize_drops_unknown_and_duplicate_ids() {
        assert_eq!(sanitize(&[1, 1, 0, 99999, 301]), vec![1, 301]);
    }
}
