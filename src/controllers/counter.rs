use crate::config::database::AppState;

use actix_web::{Error, HttpResponse, error, web};

pub async fn get_all(
    query: web::Query<dtos::Pagination>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Initialize the variables
    let page: u32 = query.page.clamp(1, u32::MAX);
    let limit: u32 = query.limit.clamp(1, 100);

    // Get a connection to the database
    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let mut stmt = conn
        .prepare(
            r#"
            SELECT username, word, count
            FROM counter
            ORDER BY count DESC, word ASC, username ASC
            LIMIT ?1
            OFFSET ?2;
            "#,
        )
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let data = stmt
        .query_map([limit, (page - 1) * limit], |row| {
            models::Count::from_row(row)
        })
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let mut stmt = conn
        .prepare(
            r#"
            SELECT SUM(count) AS total, COUNT(*) AS rows
            FROM counter;
            "#,
        )
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let Summary { total, rows } = stmt
        .query_row([], |row| models::Summary::from_row(row))
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(dtos::All {
        page,
        limit,
        total,
        rows,
        data,
    }))
}

pub async fn get_all_users(
    query: web::Query<dtos::Pagination>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Initialize the variables
    let page: u32 = query.page.clamp(1, u32::MAX);
    let limit: u32 = query.limit.clamp(1, 100);

    Ok(HttpResponse::NotImplemented().finish())
}

pub async fn get_all_words() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotImplemented().finish())
}

pub async fn get_users_by_word(path: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotImplemented().finish())
}

pub async fn get_words_by_user(path: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotImplemented().finish())
}
