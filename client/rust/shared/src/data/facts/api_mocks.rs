use super::api::FactResponse;
use crate::{domain::facts::Fact, DateTime};
use chrono::Utc;
use mockito::{Mock, Server};
use uuid::Uuid;

#[cfg(test)]
pub(crate) trait FactsApiMocks {
    fn mock_facts_success(
        &mut self, lesson_id: Uuid, facts: Vec<Fact>, with_deleted: u16, with_session: bool,
        in_pages: usize, at_timestamp: Option<u64>,
    ) -> Vec<Mock>;

    #[allow(unused)]
    fn mock_lessons_failure(&mut self) -> Mock;
}

// We mock the facts in domain terms, as the response data type is not available to packages external to data
// Then internally we will map those to the responses for mocking.
pub fn mock_facts(lesson_id: Uuid, count: usize) -> Vec<Fact> {
    (0..count)
        .map(|_| Fact {
            id: Uuid::new_v4(),
            lesson_id: lesson_id,
            element1: "Japan".to_string(),
            element2: "日本".to_string(),
            hint: "Country".to_string(),
        })
        .collect()
}

fn mock_fact_responses(facts: &Vec<Fact>, with_deleted: u16) -> Vec<FactResponse> {
    let mut i = 0u16;
    facts
        .iter()
        .map(|fact| {
            let response = FactResponse {
                id: fact.id,
                element1: fact.element1.clone(),
                element2: fact.element2.clone(),
                hint: fact.hint.clone(),
                is_deleted: i < with_deleted,
                updated_at: DateTime::from(Utc::now()).to_utc().timestamp(),
            };
            i += 1;
            response
        })
        .collect()
}

#[cfg(test)]
impl FactsApiMocks for Server {
    /// Generates a mock facts api response.
    ///
    /// Facts is a list of facts to mock, this list usually is a product of calling mock_facts()
    /// The first 'with_deleted' facts will be marked as deleted (useful for testing the sync logic)
    fn mock_facts_success(
        &mut self, lesson_id: Uuid, facts: Vec<Fact>, with_deleted: u16, with_session: bool,
        in_pages: usize, at_timestamp: Option<u64>,
    ) -> Vec<Mock> {
        use crate::data::api_mocks::mock_api_success;
        use log::debug;
        use std::collections::HashMap;

        debug!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
        let mock_responses = mock_fact_responses(&facts, with_deleted);
        return mock_api_success(
            self,
            "/facts",
            HashMap::from([("lesson_id".to_string(), lesson_id.to_string())]),
            facts,
            mock_responses,
            with_session,
            in_pages,
            at_timestamp,
        );
    }

    #[allow(unused)] // TODO: This will be used during the future lesson repo sync detail testing
    fn mock_lessons_failure(&mut self) -> Mock {
        self.mock("GET", "/facts").with_status(403).create()
    }
}
