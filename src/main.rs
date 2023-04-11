use serde::Deserialize;
use serde::Serialize;
use tide::Request;
use tide::Result;
use tide::Server;

#[async_std::main]
async fn main() -> Result<()> {
    let app = setup_server();
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn setup_server() -> Server<()> {
    let mut app = tide::new();
    app.at("/hello").get(greet);
    app.at("/hello/:name").get(greet);
    app.at("/hello_json").post(greet_json);
    app
}

async fn greet(req: Request<()>) -> Result<String> {
    let name = req.param("name").unwrap_or("world");
    Ok(format!("Hello, {}!\n", name))
}

// Leverage SerDe to build a type to parse the JSON body
#[derive(Debug, Deserialize, Serialize)]
struct Greeting {
    name: String,
    age: u16,
}

async fn greet_json(mut req: Request<()>) -> Result {
    let Greeting { name, age } = req.body_json().await?;

    Ok(format!("Hello, {}!, you appear to be {}\n", name, age).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_default_get() {
        let app = setup_server();

        use tide_testing::TideTestingExt;

        assert_eq!(
            app.get("/hello").recv_string().await.unwrap(),
            "Hello, world!\n"
        )
    }

    #[async_std::test]
    async fn test_named_get() {
        let app = setup_server();

        use tide_testing::TideTestingExt;

        assert_eq!(
            app.get("/hello/Fred").recv_string().await.unwrap(),
            "Hello, Fred!\n"
        )
    }

    #[async_std::test]
    async fn test_json_post() {
        let app = setup_server();

        use tide_testing::TideTestingExt;

        assert_eq!(
            app.post("/hello_json")
                .body(
                    tide::Body::from_json(&Greeting {
                        name: "Barney".to_string(),
                        age: 79
                    })
                    .unwrap()
                )
                .recv_string()
                .await
                .unwrap(),
            "Hello, Barney!, you appear to be 79\n"
        )
    }
}
