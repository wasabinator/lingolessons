use crate::{
    data::db::Db,
    domain::lessons::{Lesson, LessonType},
};
use chrono::{TimeZone, Utc};
use rusqlite::OptionalExtension;
use uuid::Uuid;

#[derive(PartialEq, Clone)]
pub(super) struct LessonData {
    pub(super) id: Uuid,
    pub(super) title: String,
    pub(super) r#type: u8,
    pub(super) language1: String,
    pub(super) language2: String,
    pub(super) owner: String,
    pub(super) updated_at: i64,
}

impl TryFrom<&rusqlite::Row<'_>> for LessonData {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row) -> rusqlite::Result<LessonData> {
        Ok(LessonData {
            id: row.get(0)?,
            title: row.get(1)?,
            r#type: row.get(2)?,
            language1: row.get(3)?,
            language2: row.get(4)?,
            owner: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}

impl From<u8> for LessonType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Grammar,
            _ => Self::Vocabulary,
        }
    }
}

impl From<&LessonData> for Lesson {
    fn from(lesson: &LessonData) -> Self {
        let utc = Utc.timestamp_opt(lesson.updated_at, 0).unwrap();

        Lesson {
            id: lesson.id,
            title: lesson.title.clone(),
            r#type: LessonType::from(lesson.r#type),
            language1: lesson.language1.clone(),
            language2: lesson.language2.clone(),
            owner: lesson.owner.clone(),
            updated_at: utc.into(),
        }
    }
}

pub(super) trait LessonDao {
    fn get_lessons(&self) -> rusqlite::Result<Vec<LessonData>>;
    #[allow(dead_code)] // Will be used in the future
    fn get_lesson(&self, id: Uuid) -> rusqlite::Result<Option<LessonData>>;
    fn set_lesson(&self, lesson: &LessonData) -> rusqlite::Result<()>;
    #[allow(dead_code)] // Will be used in the future
    fn del_lesson(&self, id: Uuid) -> rusqlite::Result<()>;
}

impl LessonDao for Db {
    fn get_lessons(&self) -> rusqlite::Result<Vec<LessonData>> {
        let rows = self.perform(|conn| {
            let mut stmt = conn
                .prepare(
                    r#"
                    SELECT id, title, type, language1, language2, owner, updated_at
                    FROM lesson
                    ORDER BY updated_at DESC;
                    "#,
                )
                .unwrap();
            let rows = stmt.query_map([], |row| LessonData::try_from(row)).unwrap();
            rows.collect::<Result<Vec<_>, _>>()
        })?;

        let mut lessons = Vec::new();
        for lesson in rows {
            lessons.push(lesson);
        }
        Ok(lessons)
    }

    fn get_lesson(&self, id: Uuid) -> rusqlite::Result<Option<LessonData>> {
        self.perform(|conn| {
            conn.query_row(
                r#"
                SELECT id, title, type, language1, language2, owner, updated_at
                FROM lesson WHERE id = ?;
                "#,
                [id],
                |row| LessonData::try_from(row),
            )
            .optional()
        })
    }

    fn set_lesson(&self, lesson: &LessonData) -> rusqlite::Result<()> {
        self.perform(|conn| {
            conn.execute(
                r#"
                INSERT OR REPLACE
                INTO lesson(id, title, type, language1, language2, owner, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?);
                "#,
                rusqlite::params![
                    lesson.id,
                    lesson.title,
                    lesson.r#type,
                    lesson.language1,
                    lesson.language2,
                    lesson.owner,
                    lesson.updated_at
                ],
            )
        })?;
        Ok(())
    }

    fn del_lesson(&self, id: Uuid) -> rusqlite::Result<()> {
        self.perform(|conn| {
            conn.execute(
                r#"
                DELETE FROM lesson
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
    use crate::data::lessons::db_fixtures::DbFixtures;

    #[test]
    fn test_lessons() {
        let db = Db::open("blah.txt".to_string()).unwrap();

        let r = db.get_lessons();
        assert!(r.unwrap().is_empty());

        // Insert 5 lessons into the db
        let lessons = DbFixtures::create_lessons(&db, 5);

        // Now fetch and compare the lessons
        for lesson in &lessons {
            let r = db.get_lesson(lesson.id).unwrap().unwrap();
            assert!(*lesson == r);
        }

        let id = lessons.first().unwrap().id;
        db.del_lesson(id).unwrap();
        let r = db.get_lesson(id).unwrap();
        assert!(r.is_none());

        // Make sure we have 4 lessons left in the db
        let lessons = db.get_lessons().unwrap();
        assert_eq!(4, lessons.len());
    }

    #[test]
    fn test_lessons_primary_key() {
        let db = Db::open("blah.txt".to_string()).unwrap();
        let r = db.get_lessons();
        assert!(r.unwrap().is_empty());

        let lessons = DbFixtures::create_lessons(&db, 1);
        let lesson = lessons.first().unwrap();
        db.set_lesson(lesson).unwrap();
        let r = db.get_lessons();
        assert_eq!(1, r.unwrap().len());

        // Should update rather than insert a duplicate
        db.set_lesson(lesson).unwrap();
        let r = db.get_lessons();
        assert_eq!(1, r.unwrap().len());
    }
}
