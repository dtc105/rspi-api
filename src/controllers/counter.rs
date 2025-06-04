use crate::config::database::AppState;
use crate::dtos::requests::counter::{
    QueryPagination, QueryParams, SetQueryPagination, SetQueryParams,
};
use crate::dtos::responses::counter::{
    Data, Filters, Links, Meta, Pagination, PartMeta, Response, Sort, UserData, UserResponse,
    WordData, WordResponse,
};

use actix_web::{Error, HttpResponse, error, web};

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

    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get the main data
    let mut stmt = conn
        .prepare(&format!(
            r#"
            SELECT username, word, count
            FROM counter
            WHERE (?1 IS NULL OR ?1 = username) AND
                  (?2 IS NULL OR ?2 = word)
            ORDER BY count {}
            LIMIT ?3
            OFFSET ?4;
        "#,
            &order
        ))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let items = stmt
        .query_map(
            [
                &username,
                &word,
                &Some(limit.to_string()),
                &Some(((page - 1) * limit).to_string()),
            ],
            |row| Data::from_row(row),
        )
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get meta info
    let mut stmt = conn
        .prepare(
            r#"
            SELECT COUNT(*) AS total_rows
            FROM counter
            WHERE (?1 IS NULL OR ?1 = username) AND
                  (?2 IS NULL OR ?2 = word);
        "#,
        )
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let total_rows = stmt
        .query_row([&username, &word], |row| row.get::<usize, u32>(0))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let total_pages = total_rows.div_ceil(limit);
    let has_next = page < total_pages;
    let has_prev = page > 1;

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

    let links = Links {
        own: format!("/counter?page={page}&limit={limit}"),
        first: format!("/counter?page=1&limit={limit}"),
        last: format!("/counter?page={total_pages}&limit={limit}"),
        next: if has_next {
            Some(format!("/counter?page={}&limit={limit}", page + 1))
        } else {
            None
        },
        prev: if has_prev {
            Some(format!("/counter?page={}&limit={limit}", page - 1))
        } else {
            None
        },
    };

    Ok(HttpResponse::Ok().json(Response {
        data: items,
        meta,
        links,
    }))
}

pub async fn get_all_users(
    query: web::Query<QueryPagination>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Initialize the variables
    let SetQueryPagination { page, limit, order } = query.into_inner().into();

    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let mut stmt = conn
        .prepare(&format!(
            r#"
            SELECT username, SUM(count) AS total
            FROM counter
            GROUP BY username
            ORDER BY total {}
            LIMIT ?1
            OFFSET ?2;
            "#,
            &order
        ))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let items = stmt
        .query_map([limit, (page - 1) * limit], |row| UserData::from_row(row))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get meta info
    let mut stmt = conn
        .prepare(
            r#"
            SELECT COUNT(DISTINCT(username)) AS total_rows
            FROM counter;
            "#,
        )
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let total_rows = stmt
        .query_row([], |row| row.get::<usize, u32>(0))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

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
        own: format!("/counter?page={page}&limit={limit}"),
        first: format!("/counter?page=1&limit={limit}"),
        last: format!("/counter?page={total_pages}&limit={limit}"),
        next: if has_next {
            Some(format!("/counter?page={}&limit={limit}", page + 1))
        } else {
            None
        },
        prev: if has_prev {
            Some(format!("/counter?page={}&limit={limit}", page - 1))
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

    let mut stmt = conn
        .prepare(&format!(
            r#"
            SELECT word, SUM(count) AS total
            FROM counter
            GROUP BY word
            ORDER BY total {}
            LIMIT ?1
            OFFSET ?2;
            "#,
            &order
        ))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let items = stmt
        .query_map([limit, (page - 1) * limit], |row| WordData::from_row(row))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Get meta info
    let mut stmt = conn
        .prepare(
            r#"
            SELECT COUNT(DISTINCT(username)) AS total_rows
            FROM counter;
            "#,
        )
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let total_rows = stmt
        .query_row([], |row| row.get::<usize, u32>(0))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

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
        own: format!("/counter?page={page}&limit={limit}"),
        first: format!("/counter?page=1&limit={limit}"),
        last: format!("/counter?page={total_pages}&limit={limit}"),
        next: if has_next {
            Some(format!("/counter?page={}&limit={limit}", page + 1))
        } else {
            None
        },
        prev: if has_prev {
            Some(format!("/counter?page={}&limit={limit}", page - 1))
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
