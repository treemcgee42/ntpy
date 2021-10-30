
mod md_mjax_parser; 

use std::path::Path;
use std::collections::HashMap;

use rocket_dyn_templates::{Template};

use serde_json::json;

use rusqlite::{params, Connection, Result};

use rocket::Request;
use rocket::form::Form;
use rocket::response::Redirect;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;


const DB_PATH: &str = "./persistent.db";

// ----	Regular page ------------------------------------------------------------ 

#[get("/show/<page_id>/<_..>")]
fn show_page(page_id: usize) -> Template
{
	let conn = Connection::open(DB_PATH).unwrap();

	let mut page_name = String::new();
	let mut text_id = 0;
	let mut text = String::new();

	// ----	Query page
	let success_page_query = conn.query_row(
		"SELECT title, text_id FROM pages WHERE id=?1",
		[page_id],
		|row| {
			page_name = row.get(0).unwrap();
			text_id = row.get(1).unwrap();
			Ok(())
		},
	);
	if success_page_query.is_err() { return not_found(); };

	// ----	Query text
	conn.query_row(
		"SELECT text FROM text WHERE id=?1",
		[text_id],
		|row| { 
			text = row.get(0).unwrap(); 
			Ok(())
		},
	);

        // ---- Convert markup to HTML
        let html = md_mjax_parser::convert(text);

	// ----	Render template
	let context = json!({ "page_name":page_name, "page_content":html });
	Template::render("regular_page", &context)
}

// ---- Edit page ---------------------------------------------------------------

#[get("/edit/<page_id>")]
fn edit_page(page_id: usize) -> Template
{
    // ---- Retrieve page from database
    let conn = Connection::open(DB_PATH).unwrap();

    let mut page_name = String::new();
    let mut text_id = 0;

    let page_query = conn.query_row(
        "SELECT title, text_id FROM pages WHERE id=?1",
        [page_id],
        |row| {
            page_name = row.get(0).unwrap();
            text_id = row.get(1).unwrap();
            Ok(())
        }
    );
    if page_query.is_err() { 
        println!("Couldn't query page.");
        return not_found(); 
    }

    // ---- Retrieve text from database
    let mut text = String::new();

    let text_query = conn.query_row(
        "SELECT text FROM text WHERE id=?1",
        [text_id],
        |row| { 
                text = row.get(0).unwrap(); 
                Ok(())
        },
    );
    if text_query.is_err() { 
        println!("Couldn't query text.");
        return not_found(); 
    }

    // ---- Render template
    let context = json!({"page_name":page_name, "markup_content":text});
    Template::render("edit", &context)
}

#[derive(FromForm)]
struct Edits<'r> {
    content: &'r str,
}

#[post("/edit/<page_id>", data="<edits>")]
fn submit_edits(page_id: usize, edits: Form<Edits<'_>>) -> Redirect
{
    // ---- Insert new data into database
    let conn = Connection::open(DB_PATH).unwrap();

    let mut text_id = 0;

    conn.query_row(
        "SELECT text_id FROM pages WHERE id=?1",
        [page_id],
        |row| {
            text_id = row.get(0).unwrap();
            Ok(())
        }
    );

    conn.execute(
        "UPDATE text SET text=?1 WHERE id=?2",
        params![edits.into_inner().content, text_id]
    );

    // ---- Redirect to newly updatedd page
    Redirect::to(format!("/show/{}", page_id))
}

// ----	Errors ------------------------------------------------------------------ 

#[catch(404)]
fn not_found() -> Template
{
	let context: HashMap<&str, &str> = HashMap::new();
	Template::render("not_found", context)
}

// ----	Launch ------------------------------------------------------------------ 

#[launch]
fn rocket() -> _
{
	rocket::build()
		.mount("/", routes![show_page, edit_page, submit_edits])
		.attach(Template::fairing())
		.register("/", catchers![not_found])
}
