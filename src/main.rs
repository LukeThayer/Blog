#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;

mod post;
use post::*;


#[derive(Serialize, Deserialize, Debug)]
struct PostDatabase {
    posts: Vec<Post>
}

static mut DB: PostDatabase = PostDatabase {
    posts: vec![]
};

fn load_db(db:&mut PostDatabase) {
    let tmp_db = PostDatabase {
        posts: vec![
            Post {
            uuid: 0,
            title: "First post".into(),
            category: "Juan Wheel".into(),
            author: "Lukle".into(),
            tags: vec!["esc".into(), "embedded".into()],
            visibility: true,
            date: SystemTime::now(),
            body: Some("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into())
        },
        Post {
            uuid: 1,
            title: "Second post".into(),
            category: "Juan Wheel".into(),
            author: "Lukle".into(),
            tags: vec!["boo".into(), "bbaz".into()],
            visibility: true,
            date: SystemTime::now(),
            body: Some("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?".into())
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

    for p in tmp_db.posts {
        db.posts.push(p);
    }
}

#[get("/db/posts/<uuid>")]
fn get_post(uuid: u32) -> JsonValue
{
    unsafe {
        let by_uuid: &Post = DB.posts.iter().find(|p| p.uuid == uuid).unwrap();
        json!(by_uuid)
    }
}

#[get("/db/posts")]
fn list_posts() -> JsonValue
{
    unsafe {
        let by_id: Vec<Post> = DB.posts.iter().cloned().map(|p| Post {
            body: None,
            ..p
        }).collect();
        json!(by_id)
    }
}

#[derive(Serialize)]
struct PostIndexTemplateContext {
    title: String,
    url: String
}

fn get_post_index_list(db:&mut PostDatabase) -> Vec<PostIndexTemplateContext>
{
    db.posts.iter().cloned().map(|p| PostIndexTemplateContext {
        title: p.title,
        url: format!("/post/{}", p.uuid)
    } ).collect()
}

#[derive(Serialize)]
struct IndexTemplateContext {
    title: String,
    post: Post,
    posts: Vec<PostIndexTemplateContext>
}

#[get("/")]
fn index() -> Template
{
    unsafe {
        let context = IndexTemplateContext {
            title: "Lukle Blahg".into(),
            post: DB.posts[0].clone(),
            posts: get_post_index_list(&mut DB)
        };
        Template::render("index", &context)
    }
}

#[get("/post/<uuid>")]
fn post(uuid: u32) -> Template
{
    unsafe {
        let context = IndexTemplateContext {
            title: "Home".into(),
            post: DB.posts.iter().find(|p| p.uuid == uuid).unwrap().clone(),
            posts: get_post_index_list(&mut DB)
        };
        Template::render("index", &context)
    }
}

fn main()
{
    unsafe
    {
        load_db(&mut DB);
    }

    let routes = routes![index, post, get_post, list_posts];
    let server = rocket::ignite().mount("/", routes).attach(Template::fairing());

    server.launch();
}
