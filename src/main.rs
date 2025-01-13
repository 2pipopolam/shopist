use std::net::SocketAddr;
use axum::serve;
use sea_orm::Database;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use migration::{Migrator, MigratorTrait};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Настройка логирования
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    "shopist=debug,tower_http=debug,axum::rejection=trace".into()
                }),
        )
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    // Загрузка переменных окружения
    dotenvy::dotenv().ok();

    // Настройка подключения к базе данных
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@db:5432/shopist_db".to_string()
    });

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT должен быть числом");

    // Подключение к базе данных
    tracing::info!("Подключение к базе данных...");
    let db = Database::connect(&db_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    // Применение миграций
    tracing::info!("Применение миграций...");
    Migrator::up(&db, None)
        .await
        .expect("Не удалось применить миграции");

    // Создание маршрутизатора с middleware
    let app = routes::create_router(db)
        .layer(TraceLayer::new_for_http());

    // Настройка адреса сервера
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Запуск сервера по адресу: http://{}:{}", host, port);

    // Создание и запуск сервера
    let listener = TcpListener::bind(addr)
        .await
        .expect("Не удалось запустить сервер");

    serve(listener, app.into_make_service())
        .await
        .expect("Ошибка работы сервера");

    Ok(())
}
