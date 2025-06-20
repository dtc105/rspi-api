use crate::{
    config::database::AppState,
    dtos::{
        requests::counter::{QueryPagination, QueryParams, SetQueryPagination, SetQueryParams},
        responses::counter::{
            Data, Filters, Links, Meta, Pagination, PartMeta, Response, Sort, UserData,
            UserResponse, WordData, WordResponse,
        },
    },
};

use actix_web::{Error, HttpResponse, error, web};

/// Get all counts with filters and pagination
///
/// # Route
/// `GET /counter`
///
/// # Request Query
/// - `page`: The page number to get
/// - `limit`: The amount of items to display per page
/// - `order`: The order to return the results (asc|desc). Default `desc`
/// - `username`: The username to look up with a fuzzy find
/// - `word`: The word to look up with a fuzzy find
///
/// # Responses
/// - `200 Ok`: Returns rows
/// - `400 Bad Request`: If invalid parameters
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `GET /counter?page=2&limit=3&username=di&word=hi`
///
/// # Example Response 200
/// ```
/// {
///     "data": [
///         {
///             "username": "urkodi",
///             "word": "this",
///             "count": 7,
///             "similarity": 0.33333334
///         },
///         {
///             "username": "adits87",
///             "word": "think",
///             "count": 9,
///             "similarity": 0.2857143
///         },
///         {
///             "username": "adits87",
///             "word": "shi",
///             "count": 8,
///             "similarity": 0.2857143
///         }
///     ],
///     "meta": {
///         "pagination": {
///             "page": 2,
///             "limit": 3,
///             "totalRows": 108,
///             "totalPages": 36,
///             "hasNext": true,
///             "hasPrev": true
///         },
///         "filters": {
///             "username": "di",
///             "word": "hi"
///         },
///         "sort": {
///             "by": "count",
///             "order": "desc"
///         }
///     },
///     "links": {
///         "self": "/counter?page=2&limit=3&order=desc&username=di&word=hi",
///         "first": "/counter?page=1&limit=3&order=desc&username=di&word=hi",
///         "last": "/counter?page=36&limit=3&order=desc&username=di&word=hi",
///         "prev": "/counter?page=1&limit=3&order=desc&username=di&word=hi",
///         "next": "/counter?page=3&limit=3&order=desc&username=di&word=hi"
///     }
/// }
/// ```
pub async fn get_all(
    query: web::Query<QueryParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Initialize the variables
    let SetQueryParams {
        page,
        limit,
        order,
        username,
        word,
    } = query.into_inner().into();

    // Connect to the database
    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Create the where clause for the queries
    let mut where_clauses = Vec::<String>::new();

    // If there is a username, add username LIKE to the clause
    if let Some(ref u) = username {
        let username_like = u
            .to_string()
            .split("")
            .into_iter()
            .collect::<Vec<&str>>()
            .join("%");
        where_clauses.push(format!("username LIKE '{}'", username_like));
    }

    // If there is a word, add word LIKE to the clause
    if let Some(ref w) = word {
        let word_like = w
            .to_string()
            .split("")
            .into_iter()
            .collect::<Vec<&str>>()
            .join("%");
        where_clauses.push(format!("word LIKE '{}'", word_like));
    }

    // Join the parts of the clause together
    let mut where_clause = where_clauses.join(" AND ");

    // If there is something in the clause add the WHERE keyword
    if where_clause.len() > 0 {
        where_clause = "WHERE ".to_owned() + where_clause.as_str();
    }

    // Format the query for the main data
    let query = format!(
        r#"
            SELECT username, word, count
            FROM counter
            {}
            ORDER BY count {}
            LIMIT ?1
            OFFSET ?2;
        "#,
        &where_clause, &order
    );

    // Create the statement for the main data
    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| error::ErrorInternalServerError(e.to_string() + "74"))?;

    // Get the main data
    let mut items = stmt
        .query_map([&limit, &((page - 1) * limit)], |row| {
            Data::from_row(row, &username, &word)
        })
        .map_err(|e| error::ErrorInternalServerError(e.to_string() + "78"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error::ErrorInternalServerError(e.to_string() + "80"))?;

    // Sort by similarity of found username to queried username
    if (&username).is_some() || (&word).is_some() {
        items.sort_by(|a, b| {
            b.similarity
                .unwrap_or(0.0)
                .partial_cmp(&a.similarity.unwrap_or(0.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    // Format the query for the meta data
    let query = format!(
        r#"
            SELECT COUNT(*) AS total_rows
            FROM counter
            {};
        "#,
        where_clause
    );

    // Create the statement for the meta data
    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| error::ErrorInternalServerError(e.to_string() + "92"))?;

    // Get the meta data
    let total_rows = stmt
        .query_row([], |row| row.get::<usize, u32>(0))
        .map_err(|e| error::ErrorInternalServerError(e.to_string() + "96"))?;

    // Format the response
    let total_pages = total_rows.div_ceil(limit);
    let has_next = page < total_pages;
    let has_prev = page > 1;

    let mut filter = String::new();

    if let Some(ref u) = username {
        filter += &format!("&username={u}");
    }

    if let Some(ref w) = word {
        filter += &format!("&word={w}");
    }

    let links = Links {
        own: format!("/counter?page={page}&limit={limit}&order={order}{filter}"),
        first: format!("/counter?page=1&limit={limit}&order={order}{filter}"),
        last: format!("/counter?page={total_pages}&limit={limit}&order={order}{filter}"),
        next: if has_next {
            Some(format!(
                "/counter?page={}&limit={limit}&order={order}{filter}",
                page + 1
            ))
        } else {
            None
        },
        prev: if has_prev {
            Some(format!(
                "/counter?page={}&limit={limit}&order={order}{filter}",
                page - 1
            ))
        } else {
            None
        },
    };

    let meta = Meta {
        pagination: Pagination {
            page,
            limit,
            total_rows,
            total_pages,
            has_next,
            has_prev,
        },
        filters: Filters { username, word },
        sort: Sort {
            by: "count".to_string(),
            order,
        },
    };

    Ok(HttpResponse::Ok().json(Response {
        data: items,
        meta,
        links,
    }))
}

/// Get all users with their total count
///
/// # Route
/// `GET /counter/users`
///
/// # Request Query
/// - `page`: The page number to get
/// - `limit`: The amount of items to display per page
/// - `order`: The order to return the results (asc|desc). Default `desc`
///
/// # Responses
/// - `200 Ok`: Returns rows
/// - `400 Bad Request`: If invalid parameters
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `GET /counter/users?page=1&limit=3&order=desc`
///
/// # Example Response 200
/// ```
/// {
///     "data": [
///         {
///             "username": "adits87",
///             "count": 5783
///         },
///         {
///             "username": "qa_z",
///             "count": 3830
///         },
///         {
///             "username": "lilith_dysnomia",
///             "count": 2501
///         }
///     ],
///     "meta": {
///         "pagination": {
///             "page": 1,
///             "limit": 3,
///             "totalRows": 5,
///             "totalPages": 2,
///             "hasNext": true,
///             "hasPrev": false
///         },
///         "sort": {
///             "by": "count",
///             "order": "desc"
///         }
///     },
///     "links": {
///         "self": "/counter?page=1&limit=3",
///         "first": "/counter?page=1&limit=3",
///         "last": "/counter?page=2&limit=3",
///         "prev": null,
///         "next": "/counter?page=2&limit=3"
///     }
/// }
/// ```
pub async fn get_all_users(
    query: web::Query<QueryPagination>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Initialize the variables
    let SetQueryPagination { page, limit, order } = query.into_inner().into();

    // Connect to the database
    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Format the query for the main data
    let query = format!(
        r#"
            SELECT username, SUM(count) AS total
            FROM counter
            GROUP BY username
            ORDER BY total {}
            LIMIT ?1
            OFFSET ?2;
        "#,
        &order
    );

    // Create the statement for the main data
    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get the main data
    let items = stmt
        .query_map([limit, (page - 1) * limit], |row| UserData::from_row(row))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Define the query for the meta data
    let query = r#"
        SELECT COUNT(DISTINCT(username)) AS total_rows
        FROM counter;
    "#;

    // Create the statement for the meta data
    let mut stmt = conn
        .prepare(query)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get the meta data
    let total_rows = stmt
        .query_row([], |row| row.get::<usize, u32>(0))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Format the response
    let total_pages = total_rows.div_ceil(limit);
    let has_next = page < total_pages;
    let has_prev = page > 1;

    let meta = PartMeta {
        pagination: Pagination {
            page,
            limit,
            total_rows,
            total_pages,
            has_next,
            has_prev,
        },
        sort: Sort {
            by: "count".to_string(),
            order,
        },
    };

    let links = Links {
        own: format!("/counter?page={page}&limit={limit}&order={order}"),
        first: format!("/counter?page=1&limit={limit}&order={order}"),
        last: format!("/counter?page={total_pages}&limit={limit}&order={order}"),
        next: if has_next {
            Some(format!(
                "/counter?page={}&limit={limit}&order={order}",
                page + 1
            ))
        } else {
            None
        },
        prev: if has_prev {
            Some(format!(
                "/counter?page={}&limit={limit}&order={order}",
                page - 1
            ))
        } else {
            None
        },
    };

    Ok(HttpResponse::Ok().json(UserResponse {
        data: items,
        meta,
        links,
    }))
}

/// Get all words with their total count
///
/// # Route
/// `GET /counter/words`
///
/// # Request Query
/// - `page`: The page number to get
/// - `limit`: The amount of items to display per page
/// - `order`: The order to return the results (asc|desc). Default `desc`
///
/// # Responses
/// - `200 Ok`: Returns rows
/// - `400 Bad Request`: If invalid parameters
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `GET /counter/words?page=2&limit=3&order=desc`
///
/// # Example Response 200
/// ```
/// {
///     "data": [
///         {
///             "word": "u",
///             "count": 187
///         },
///         {
///             "word": "a",
///             "count": 185
///         },
///         {
///             "word": "and",
///             "count": 146
///         }
///     ],
///     "meta": {
///         "pagination": {
///             "page": 2,
///             "limit": 3,
///             "totalRows": 5,
///             "totalPages": 2,
///             "hasNext": false,
///             "hasPrev": true
///         },
///         "sort": {
///             "by": "count",
///             "order": "desc"
///         }
///     },
///     "links": {
///         "self": "/counter?page=2&limit=3",
///         "first": "/counter?page=1&limit=3",
///         "last": "/counter?page=2&limit=3",
///         "prev": "/counter?page=1&limit=3",
///         "next": null
///     }
/// }
/// ```
pub async fn get_all_words(
    query: web::Query<QueryPagination>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Initialize the variables
    let SetQueryPagination { page, limit, order } = query.into_inner().into();

    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Format the query for the main data
    let query = format!(
        r#"
            SELECT word, SUM(count) AS total
            FROM counter
            GROUP BY word
            ORDER BY total {}
            LIMIT ?1
            OFFSET ?2;
        "#,
        &order
    );

    // Create the statement for the main data
    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get the main data
    let items = stmt
        .query_map([limit, (page - 1) * limit], |row| WordData::from_row(row))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Define the query for the meta data
    let query = r#"
        SELECT COUNT(DISTINCT(username)) AS total_rows
        FROM counter;
    "#;

    // Create the statement for the meta data
    let mut stmt = conn
        .prepare(query)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get the meta data
    let total_rows = stmt
        .query_row([], |row| row.get::<usize, u32>(0))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Format the response
    let total_pages = total_rows.div_ceil(limit);
    let has_next = page < total_pages;
    let has_prev = page > 1;

    let meta = PartMeta {
        pagination: Pagination {
            page,
            limit,
            total_rows,
            total_pages,
            has_next,
            has_prev,
        },
        sort: Sort {
            by: "count".to_string(),
            order,
        },
    };

    let links = Links {
        own: format!("/counter?page={page}&limit={limit}&order={order}"),
        first: format!("/counter?page=1&limit={limit}&order={order}"),
        last: format!("/counter?page={total_pages}&limit={limit}&order={order}"),
        next: if has_next {
            Some(format!(
                "/counter?page={}&limit={limit}&order={order}",
                page + 1
            ))
        } else {
            None
        },
        prev: if has_prev {
            Some(format!(
                "/counter?page={}&limit={limit}&order={order}",
                page - 1
            ))
        } else {
            None
        },
    };

    Ok(HttpResponse::Ok().json(WordResponse {
        data: items,
        meta,
        links,
    }))
}
