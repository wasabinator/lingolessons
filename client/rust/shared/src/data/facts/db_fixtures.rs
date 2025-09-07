use crate::data::{
    db::Db,
    facts::db::{FactDao, FactData},
};
use chrono::Utc;
use uuid::Uuid;

pub(in crate::data) struct DbFixtures {}

impl DbFixtures {
    pub(super) fn create_facts(db: &Db, lesson_id: Uuid, count: u16) -> Vec<FactData> {
        let epoc_time = Utc::now().timestamp();

        let facts: Vec<FactData> = (0..count)
            .map(|i| FactData {
                id: Uuid::new_v4(),
                lesson_id: lesson_id,
                element1: "日本".to_string(),
                element2: "Japan".to_string(),
                hint: format!("fact{i}"), //.to_string(),
                updated_at: epoc_time + (i as i64),
            })
            .collect();

        for fact in facts.clone() {
            db.set_fact(&fact).unwrap();
        }

        facts
    }
}
