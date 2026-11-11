use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

pub const COLLECTION_ID: u32 = 0;
pub const FAVORITES_KEY: &str = "lets-build-sentence.favorites";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Sentence {
    pub id: u32,
    pub number: u32,
    pub chinese: String,
    pub english: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Bank {
    pub id: u32,
    pub name: String,
    pub sentences: Vec<Sentence>,
}

#[derive(Clone, Debug, Deserialize)]
struct BankFile {
    banks: Vec<Bank>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewSentence {
    pub id: u32,
    pub number: u32,
    pub chinese: String,
    pub english: String,
    pub favorite: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PracticeSet {
    pub id: u32,
    pub name: String,
    pub is_collection: bool,
    pub sentences: Vec<ViewSentence>,
}

pub struct Catalog {
    banks: Vec<Bank>,
    by_id: HashMap<u32, (usize, usize)>,
}

impl Catalog {
    pub fn banks(&self) -> &[Bank] {
        &self.banks
    }

    pub fn bank(&self, id: u32) -> Option<&Bank> {
        self.banks.iter().find(|bank| bank.id == id)
    }

    pub fn sentence(&self, id: u32) -> Option<&Sentence> {
        let (bank_index, sentence_index) = self.by_id.get(&id).copied()?;
        self.banks.get(bank_index)?.sentences.get(sentence_index)
    }
}

pub fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let file: BankFile = serde_json::from_str(include_str!("data/banks.json"))
            .expect("内置题库格式不正确");
        let mut by_id = HashMap::new();
        for (bank_index, bank) in file.banks.iter().enumerate() {
            for (sentence_index, sentence) in bank.sentences.iter().enumerate() {
                by_id.insert(sentence.id, (bank_index, sentence_index));
            }
        }
        Catalog {
            banks: file.banks,
            by_id,
        }
    })
}

pub fn practice_set(bank_id: u32, favorite_ids: &[u32]) -> Option<PracticeSet> {
    if bank_id == COLLECTION_ID {
        return Some(PracticeSet {
            id: COLLECTION_ID,
            name: "我的收藏".to_string(),
            is_collection: true,
            sentences: collection_sentences(favorite_ids),
        });
    }
    let bank = catalog().bank(bank_id)?;
    let favorite = favorite_ids.iter().copied().collect::<std::collections::HashSet<_>>();
    Some(PracticeSet {
        id: bank.id,
        name: bank.name.clone(),
        is_collection: false,
        sentences: bank
            .sentences
            .iter()
            .map(|sentence| ViewSentence {
                id: sentence.id,
                number: sentence.number,
                chinese: sentence.chinese.clone(),
                english: sentence.english.clone(),
                favorite: favorite.contains(&sentence.id),
            })
            .collect(),
    })
}

pub fn collection_sentences(ids: &[u32]) -> Vec<ViewSentence> {
    ids.iter()
        .filter_map(|id| catalog().sentence(*id))
        .enumerate()
        .map(|(index, sentence)| ViewSentence {
            id: sentence.id,
            number: (index as u32) + 1,
            chinese: sentence.chinese.clone(),
            english: sentence.english.clone(),
            favorite: true,
        })
        .collect()
}

pub fn known_sentence(id: u32) -> bool {
    catalog().sentence(id).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embeds_the_three_builtin_banks() {
        let catalog = catalog();
        assert_eq!(catalog.banks().len(), 3);
        assert_eq!(catalog.banks()[0].name, "英语造句练习");
        assert_eq!(catalog.banks()[0].sentences.len(), 300);
        assert_eq!(catalog.banks()[1].name, "英语造句练习·中级");
        assert_eq!(catalog.banks()[1].sentences.len(), 500);
        assert_eq!(catalog.banks()[2].name, "英语造句练习·高级");
        assert_eq!(catalog.banks()[2].sentences.len(), 500);
        assert_eq!(catalog.banks()[0].sentences[0].id, 1);
        assert_eq!(catalog.banks()[0].sentences[0].chinese, "我喜欢苹果。");
        assert_eq!(catalog.banks()[0].sentences[0].english, "I like apples.");
        assert_eq!(catalog.sentence(301).unwrap().chinese, "你应该早点睡觉。");
        assert_eq!(
            catalog.sentence(801).unwrap().english,
            "You should have told me the truth earlier."
        );
        for bank in catalog.banks() {
            for (index, sentence) in bank.sentences.iter().enumerate() {
                assert_eq!(sentence.number, (index as u32) + 1);
            }
        }
        assert_eq!(catalog.by_id.len(), 1300);
    }
}
