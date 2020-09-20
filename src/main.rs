#![deny(warnings)]
use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use serde_json;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Post
{
    uuid:u32,
    title:String,
    category: String, 
    author: String,
    tags: Vec<String>,
    visibility:bool,
    date: SystemTime,
    body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostDatabase {
    posts: Vec<Post>
}

async fn find_post(request: &Request<Body>) -> Body{
    let db = PostDatabase {
        posts: vec![
            Post {
            uuid: 0,
            title: "First post".into(),
            category: "Juan Wheel".into(),
            author: "Lukle".into(),
            tags: vec!["esc".into(), "embedded".into()],
            visibility: true,
            date: SystemTime::now(),
            body: None
        },
        Post {
            uuid: 1,
            title: "Second post".into(),
            category: "Juan Wheel".into(),
            author: "Lukle".into(),
            tags: vec!["boo".into(), "bbaz".into()],
            visibility: true,
            date: SystemTime::now(),
            body: None
        },
        Post {
            uuid: 2,
            title: "Third post".into(),
            category: "LOL CAT".into(),
            author: "Lukle".into(),
            tags: vec!["boo".into(), "bbaz".into()],
            visibility: true,
            date: SystemTime::now(),
            body: None
        },
        ]
    };

    let mut uri_string = "http://127.0.0.1:3000".to_string();
    uri_string.push_str(&request.uri().to_string());

    let request_url = Url::parse(&uri_string).unwrap();
    let mut params = request_url.query_pairs();

    if let Some((_, search)) = params.find(|(k, _)| k == "tag")
    {
        println!("{:#?}", search);

        // let boo_posts: Vec<&Post> = db.posts.iter().filter(|p| p.title.to_lowercase() == search.to_lowercase()).collect();
        let boo_posts: Vec<&Post> = db.posts.iter().filter(|p| p.tags.iter().any(|t| t == &search)).collect();
    
        let json_p = serde_json::to_string(&boo_posts).expect("");
        println!("{}", json_p);
        // let parsed_p: Post = serde_json::from_str(&json_p.as_str()).expect("");
        // println!("{:#?}", parsed_p);

        Body::from(json_p)
    }
    else {
        Body::from("Can't do")
    }
}

async fn post_server(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("<html><title>Lukle blogal</title><body>Hi fren</body></html>");
        },
        (&Method::GET, "/db") => {
            *response.body_mut() = find_post(&req).await;
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(post_server)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}