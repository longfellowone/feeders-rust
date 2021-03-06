use actix_cors::Cors;
use actix_web::{web, App, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, ID};
use async_graphql_actix_web::{Request, Response};
use sqlx::{FromRow, PgPool};
use std::env;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn index(schema: web::Data<Schema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

#[derive(FromRow, Debug)]
struct Todo {
    id: Uuid,
    text: String,
}

#[Object]
impl Todo {
    pub async fn id(&self) -> ID {
        self.id.into()
    }
    pub async fn text(&self) -> &str {
        &self.text
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> FieldResult<Vec<Todo>> {
        let db_pool = ctx.data::<PgPool>()?;

        let conductor = vd::T9.cu_resistance_steel.get("600").unwrap();
        println!("{:#?}", conductor);

        let todos = sqlx::query_as::<_, Todo>(
            r#"
                SELECT id, text
                FROM todos
            "#,
        )
        .fetch_all(db_pool)
        .await
        .unwrap();

        Ok(todos)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn test(&self, username: String) -> FieldResult<String> {
        sleep(Duration::from_secs(2));
        Ok(username)
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let db_pass = env::var("DB_PASS").unwrap();
    let db_url = format!("postgres://postgres:{}@localhost/rust", db_pass);

    let db_pool = PgPool::connect(&db_url).await?;

    actix_web::rt::spawn(async {
        let mut interval = actix_web::rt::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            println!("Synced");
            break;
        }
    });

    let schema = async_graphql::Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db_pool)
        .finish();

    println!("Stating server...");

    actix_web::HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(Cors::permissive())
            .route("/", web::post().to(index))
            .route("/", web::get().to(index_playground))
    })
    .bind("0.0.0.0:8080")? // Use 0.0.0.0 to access from network
    .run()
    .await?;

    Ok(())
}
