use crate::data::db::{Db, RowMapper};
use rusqlite::OptionalExtension;
use uuid::Uuid;

#[derive(PartialEq)]
pub(super) struct Lesson {
    pub(super) id: Uuid,
    pub(super) title: String,
    pub(super) r#type: u8,
    pub(super) language1: String,
    pub(super) language2: String,
    pub(super) owner: String,
    pub(super) updated_at: i64,
}

impl RowMapper<Lesson> for Lesson {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Lesson> {
        Ok(
            Lesson { 
                id: row.get(0)?, 
                title: row.get(1)?, 
                r#type: row.get(2)?,
                language1: row.get(3)?,
                language2: row.get(4)?,
                owner: row.get(5)?,
                updated_at: row.get(6)?,
            }
        )
    }
}

pub(super) trait LessonDao {
    fn get_lesson(&self, id: Uuid) -> rusqlite::Result<Option<Lesson>>;
    fn get_lessons(&self) -> rusqlite::Result<Vec<Lesson>>;
    fn set_lesson(&self, lesson: &Lesson) -> rusqlite::Result<()>;
    fn del_lesson(&self, id: Uuid) -> rusqlite::Result<()>;
}

impl LessonDao for Db {
    fn get_lessons(&self) -> rusqlite::Result<Vec<Lesson>> {
        let mut statement = self.connection.prepare(
            r#"
            SELECT id, title, type, language1, language2, owner, updated_at
            FROM lesson
            ORDER BY updated_at DESC;
            "#,
        )?;
        let rows = statement.query_map(
            [],
            |row| Lesson::from_row(row),
        )?;

        let mut lessons = Vec::new();
        for lesson in rows {
            lessons.push(lesson?);
        }

        Ok(lessons)
    }

    fn get_lesson(&self, id: Uuid) -> rusqlite::Result<Option<Lesson>> {
        self.connection.query_row(
            r#"
            SELECT id, title, type, language1, language2, owner, updated_at
            FROM lesson WHERE id = ?;"#,
            [id],
            |row| Lesson::from_row(row)
        ).optional()
    }

    fn set_lesson(&self, lesson: &Lesson) -> rusqlite::Result<()> {
        self.connection.execute(
            r#"
            INSERT OR REPLACE 
            INTO lesson(id, title, type, language1, language2, owner, updated_at) 
            VALUES (?, ?, ?, ?, ?, ?, ?);
            "#,
            rusqlite::params![lesson.id, lesson.title, lesson.r#type, lesson.language1, lesson.language2, lesson.owner, lesson.updated_at]
        )?;
        Ok(())
    }

    fn del_lesson(&self, id: Uuid) -> rusqlite::Result<()> {
        self.connection.execute(
            r#"
            DELETE FROM lesson
            WHERE id = ?;
            "#,
            rusqlite::params![id],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::lessons::db_fixtures::DbFixtures;

    use super::*;

    #[test]
    fn test_lessons() {
        let db = Db::open("blah.txt".to_string()).unwrap();
        let r = db.get_lessons();
        assert!(r.unwrap().is_empty());

        let lessons = DbFixtures::create_lessons(5);

        // Insert 5 lessons into the db
        for lesson in &lessons {
            db.set_lesson(&lesson).unwrap();
        }

        // Now fetch and compare the lessons
        for lesson in &lessons {
            let r = db.get_lesson(lesson.id).unwrap().unwrap();
            assert!(*lesson == r);
        }

        let id = lessons.get(0).unwrap().id;
        db.del_lesson(id).unwrap();
        let r = db.get_lesson(id).unwrap();
        assert!(r.is_none());
        
        // Make sure we have 4 lessons left in the db
        let lessons = db.get_lessons().unwrap();
        assert_eq!(4, lessons.len());
    }
}
