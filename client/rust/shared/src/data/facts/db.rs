use crate::{data::db::Db, domain::facts::Fact};
use uuid::Uuid;

#[derive(PartialEq, Clone)]
pub(super) struct FactData {
    pub(super) id: Uuid,
    pub(super) lesson_id: Uuid,
    pub(super) element1: String,
    pub(super) element2: String,
    pub(super) hint: String,
    pub(super) updated_at: i64,
}

impl TryFrom<&rusqlite::Row<'_>> for FactData {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row) -> rusqlite::Result<FactData> {
        Ok(FactData {
            id: row.get(0)?,
            lesson_id: row.get(1)?,
            element1: row.get(2)?,
            element2: row.get(3)?,
            hint: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

impl From<&FactData> for Fact {
    fn from(fact: &FactData) -> Self {
        Fact {
            id: fact.id,
            lesson_id: fact.lesson_id,
            element1: fact.element1.clone(),
            element2: fact.element2.clone(),
            hint: fact.hint.clone(),
        }
    }
}

pub(super) trait FactDao {
    fn get_facts(&self, lesson_id: Uuid) -> rusqlite::Result<Vec<FactData>>;
    fn del_facts(&self, lesson_id: Uuid) -> rusqlite::Result<()>;
    fn set_fact(&self, fact: &FactData) -> rusqlite::Result<()>;
    fn del_fact(&self, id: Uuid) -> rusqlite::Result<()>;
}

impl FactDao for Db {
    fn get_facts(&self, lesson_id: Uuid) -> rusqlite::Result<Vec<FactData>> {
        let rows = self.perform(|conn| {
            let mut stmt = conn
                .prepare(
                    r#"
                SELECT id, lesson_id, element1, element2, hint, updated_at
                FROM fact
                WHERE lesson_id = (?1)
                ORDER BY updated_at DESC;
                "#,
                )
                .unwrap();
            let rows = stmt
                .query_map([lesson_id], |row| FactData::try_from(row))
                .unwrap();
            rows.collect::<Result<Vec<_>, _>>()
        })?;

        let mut facts = Vec::new();
        for fact in rows {
            facts.push(fact);
        }
        Ok(facts)
    }

    fn del_facts(&self, lesson_id: Uuid) -> rusqlite::Result<()> {
        self.perform(|conn| {
            conn.execute(
                r#"
                DELETE FROM fact
                WHERE lesson_id = ?;
                "#,
                rusqlite::params![lesson_id],
            )
        })?;
        Ok(())
    }

    fn set_fact(&self, fact: &FactData) -> rusqlite::Result<()> {
        self.perform(|conn| {
            conn.execute(
                r#"
                INSERT OR REPLACE
                INTO fact(id, lesson_id, element1, element2, hint, updated_at)
                VALUES (?, ?, ?, ?, ?, ?);
                "#,
                rusqlite::params![
                    fact.id,
                    fact.lesson_id,
                    fact.element1,
                    fact.element2,
                    fact.hint,
                    fact.updated_at
                ],
            )
        })?;
        Ok(())
    }

    fn del_fact(&self, id: Uuid) -> rusqlite::Result<()> {
        self.perform(|conn| {
            conn.execute(
                r#"
                DELETE FROM fact
                WHERE id = ?;
                "#,
                rusqlite::params![id],
            )
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::facts::db_fixtures::DbFixtures;

    #[test]
    fn test_facts() {
        let db = Db::open("blah.txt".to_string()).unwrap();

        let r = db.get_facts(Uuid::new_v4());
        assert!(r.unwrap().is_empty());

        let lesson_id = Uuid::new_v4();
        crate::data::lessons::db_fixtures::DbFixtures::insert_lesson(
            &db,
            lesson_id,
            "blah".to_string(),
        );

        // Insert 5 facts into the db
        let facts = DbFixtures::create_facts(&db, lesson_id, 5);

        // Now fetch and compare the facts
        let f = db.get_facts(lesson_id).unwrap();
        for fact in &f {
            assert!(facts.contains(fact));
        }

        let id = facts.first().unwrap().id;
        db.del_fact(id).unwrap();

        // Make sure we have 4 facts left in the db
        let facts = db.get_facts(lesson_id).unwrap();
        assert_eq!(4, facts.len());

        let facts = db.get_facts(lesson_id).unwrap();
        assert_eq!(4, facts.len());

        let facts = db.get_facts(Uuid::new_v4()).unwrap();
        assert_eq!(0, facts.len());
    }

    #[test]
    fn test_facts_primary_key() {
        let lesson_id: Uuid = Uuid::new_v4();

        let db = Db::open("blah.txt".to_string()).unwrap();
        let r = db.get_facts(lesson_id);
        assert!(r.unwrap().is_empty());

        crate::data::lessons::db_fixtures::DbFixtures::insert_lesson(
            &db,
            lesson_id,
            "blah".to_string(),
        );

        let facts = DbFixtures::create_facts(&db, lesson_id, 1);
        let fact = facts.first().unwrap();
        db.set_fact(fact).unwrap();
        let r = db.get_facts(lesson_id);
        assert_eq!(1, r.unwrap().len());

        // Should update rather than insert a duplicate
        db.set_fact(fact).unwrap();
        let r = db.get_facts(lesson_id);
        assert_eq!(1, r.unwrap().len());
    }
}
