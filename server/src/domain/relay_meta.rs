use std::fmt::Display;

use async_graphql::{
    connection::{
        query, Connection, CursorType, DefaultConnectionName, DefaultEdgeName, DisableNodesField,
        Edge, EmptyFields,
    },
    Error, Interface, OutputType, ID,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use tracing::instrument;

use super::{
    app_user::AppUser,
    comment::Comment,
    db_id::{DbId, HasDbId},
    post::Post,
};

#[derive(Interface)]
#[graphql(field(name = "id", ty = "ID"))]
pub enum Node {
    AppUser(AppUser),
    Comment(Comment),
    Post(Post),
}

#[derive(Debug, PartialEq)]
pub struct AppCursor(pub DbId);

#[derive(Debug)]
pub struct AppCursorError(String);

impl Display for AppCursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl CursorType for AppCursor {
    type Error = AppCursorError;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        let bytes = URL_SAFE
            .decode(s)
            .map_err(|_| AppCursorError("Could not decode cursor".to_string()))?;

        let slice: [u8; 4] = bytes
            .try_into()
            .map_err(|_| AppCursorError("Cursor had unexpected content".to_string()))?;

        let as_i32 = i32::from_le_bytes(slice);

        Ok(AppCursor(DbId::from(as_i32)))
    }

    fn encode_cursor(&self) -> String {
        URL_SAFE.encode(self.0.to_le_bytes())
    }
}

pub type AppConnection<T> = Connection<
    AppCursor,
    T,
    EmptyFields,
    EmptyFields,
    DefaultConnectionName,
    DefaultEdgeName,
    DisableNodesField,
>;

/* This is needlessly async ("query") and forces the user to load everything.
Maybe benchmark if this is more/less performant than two roundtrips (one to get count). */
#[instrument(skip(results), err(Debug))]
pub async fn paginate<T: OutputType + HasDbId>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    mut results: Vec<T>,
) -> Result<AppConnection<T>, Error> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let results_len = results.len();

            let after =
                after.and_then(|a: AppCursor| results.iter().position(|x| x.db_id() == a.0));

            let before =
                before.and_then(|b: AppCursor| results.iter().position(|x| x.db_id() == b.0));

            let (start, end) = match determine_range(after, before, first, last, results_len) {
                Some(val) => val,
                None => return Ok(Connection::new(false, false)),
            };

            if start > end {
                return Err("\"after\" should not be greater than or equal \"before\"".into());
            }

            let slice: Vec<_> = results.drain(start..=end).collect();

            let mut connection: AppConnection<T> = Connection::new(start > 0, end < results.len());

            connection.edges.extend(
                slice
                    .into_iter()
                    .map(|item| Edge::new(AppCursor(item.db_id()), item)),
            );

            Ok::<_, Error>(connection)
        },
    )
    .await
}

fn determine_range(
    after: Option<usize>,
    before: Option<usize>,
    first: Option<usize>,
    last: Option<usize>,
    results_len: usize,
) -> Option<(usize, usize)> {
    let mut start: usize = 0;
    let mut end: usize = results_len.saturating_sub(1);

    if let Some(after) = after {
        if after >= results_len {
            return None;
        }
        start = after + 1;
    }

    if let Some(before) = before {
        if before == 0 {
            return None;
        }
        end = before.saturating_sub(1);
    }

    if start > end {
        return None;
    }

    if let Some(first) = first {
        let offset = first.saturating_sub(1);
        let new_end = end.min(start + offset);
        if start <= new_end {
            end = new_end;
        }
    } else if let Some(last) = last {
        let offset = last.saturating_sub(1);
        let new_start = start.max(end.saturating_sub(offset));
        if end >= new_start {
            start = new_start;
        }
    }

    Some((start, end))
}

#[cfg(test)]
mod tests {
    use async_graphql::connection::CursorType;

    use crate::domain::{db_id::DbId, relay_meta::AppCursor};

    use super::determine_range;

    #[test]
    fn encode() {
        for x in [i32::MIN, -1, 0, 1, i32::MAX] {
            let cursor = AppCursor(DbId::from(x));
            assert_eq!(
                cursor,
                AppCursor::decode_cursor(&AppCursor::encode_cursor(&cursor)).unwrap()
            );
        }
    }

    const ARRAY: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    #[test]
    fn determine_range_forward() {
        let (from, to) = determine_range(Some(2), None, Some(4), None, ARRAY.len()).unwrap();
        assert_eq!([3, 4, 5, 6], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_forward_single() {
        let (from, to) = determine_range(Some(2), None, Some(1), None, ARRAY.len()).unwrap();
        assert_eq!([3], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_backward() {
        let (from, to) = determine_range(None, Some(2), None, Some(4), ARRAY.len()).unwrap();
        assert_eq!([0, 1], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_backward_single() {
        let (from, to) = determine_range(None, Some(2), None, Some(1), ARRAY.len()).unwrap();
        assert_eq!([1], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_too_few_results() {
        let (from, to) = determine_range(Some(7), None, Some(4), None, ARRAY.len()).unwrap();
        assert_eq!([8, 9], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_after_with_before() {
        let (from, to) = determine_range(Some(2), Some(5), Some(4), None, ARRAY.len()).unwrap();
        assert_eq!([3, 4], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_all() {
        let (from, to) = determine_range(None, None, None, None, ARRAY.len()).unwrap();
        assert_eq!(ARRAY, ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_after_w_last() {
        let (from, to) = determine_range(Some(5), None, None, Some(2), ARRAY.len()).unwrap();
        assert_eq!([8, 9], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_after_w_last_edge() {
        let result = determine_range(Some(9), None, None, Some(1), ARRAY.len());
        assert!(result.is_none());
    }

    #[test]
    fn determine_range_before_w_first() {
        let (from, to) = determine_range(None, Some(4), Some(3), None, ARRAY.len()).unwrap();
        assert_eq!([0, 1, 2], ARRAY[from..=to]);
    }

    #[test]
    fn determine_range_before_w_first_edge() {
        let result = determine_range(None, Some(0), Some(1), None, ARRAY.len());
        assert!(result.is_none());
    }

    #[test]
    fn determine_range_short() {
        let array = [0, 1];
        let (from, to) = determine_range(Some(0), None, Some(3), None, array.len()).unwrap();
        assert_eq!([1], array[from..=to]);
    }

    #[test]
    fn determine_range_single_input() {
        let array = [0];
        let result = determine_range(Some(0), None, Some(3), None, array.len());
        assert!(result.is_none());
    }

    #[test]
    fn determine_range_empty_input() {
        let array: [i32; 0] = [];
        let result = determine_range(Some(0), None, Some(3), None, array.len());
        assert!(result.is_none());
    }
}
