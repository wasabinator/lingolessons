use crate::data::api::PagedResponse;
#[cfg(test)]
use mockito::Server;
use mockito::{Matcher, Mock};
#[cfg(test)]
use serde::Serialize;
use std::collections::HashMap;

pub(crate) fn mock_api_success<T, U>(
    server: &mut Server, base_url: &str, params: HashMap<String, String>, entities: Vec<T>,
    mock_responses: Vec<U>, with_session: bool, in_pages: usize, at_timestamp: Option<u64>,
) -> Vec<Mock>
where
    T: Clone,
    U: Clone + Serialize,
{
    let count = entities.len();
    //let  = mock_responses(entities, with_deleted);
    let responses: Vec<Vec<U>> = if count > 0 {
        mock_responses.chunks(count / in_pages).map(|chunk| chunk.to_vec()).collect()
    } else {
        vec![Vec::new()]
    };

    let mut i = 1;
    let max = responses.len();

    responses
        .iter()
        .map(|response| {
            let previous = match i {
                1 => None,
                _ => Some(format!("page={}", i - 1)),
            };

            let next = if i < max { Some(format!("page={}", i + 1)) } else { None };

            let r =
                PagedResponse { count: count as u16, previous, next, results: response.clone() };

            let params: Vec<_> = params
                .clone()
                .into_iter()
                .chain(vec![("page_no".to_string(), format!("{i}"))])
                .map(|(key, val)| Matcher::UrlEncoded(key, val))
                .collect();

            let mut mock = server
                .mock("GET", base_url)
                .with_status(200)
                .match_query(Matcher::AllOf(params))
                .with_body(serde_json::to_string(&r).unwrap());

            mock = if with_session {
                mock.match_header("Authorization", "Bearer mock_access_token")
            } else {
                mock.match_header("Authorization", Matcher::Missing)
            };

            mock = if let Some(timestamp) = at_timestamp {
                mock.match_query(Matcher::UrlEncoded("since".into(), timestamp.to_string()))
            } else {
                mock
            };

            i += 1;

            mock.create()
        })
        .collect()
}
