use crate::error_template::{AppError, ErrorTemplate};
use serde::{Deserialize, Serialize};
use html::Input;
use leptos::*;
use leptos_dom::logging::console_log;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
use crate::db;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
struct User {
    id: u16,
    name: String,
}

#[server]
pub async fn say_hi(name: String) -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    Ok(format!("Hello {}!", name))
}

#[server]
pub async fn get_greeted_people() -> Result<Vec<String>, ServerFnError> {
    let mut conn = db::conn().await?;
    let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(&mut conn).await?;

    let mut output: Vec<String>  = Vec::new();
    for i in users {
        output.push(i.name);
    }

    Ok(output)
}

#[server]
pub async fn add_greeted_people(name: String) -> Result<(), ServerFnError> {
    println!("Adding {}!", name);

    let mut conn = db::conn().await?;
    sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind(name)
        .execute(&mut conn).await?;

    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/greeter-app.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    let input_ref = NodeRef::<Input>::new();
    let (shout_result, set_shout_result) = create_signal("Click me".to_string());
    let (greeted_people, set_people) = create_signal( String::new() );

    create_effect(move |_| {
        spawn_local(async move {
            let people = get_greeted_people().await.map_err(|err| {
                let e = err.to_string();
                let es = e.as_str();
                console_log(es);
                return;
            }).unwrap();
            let people_string: String = people.join(", ");
            set_people.set( people_string );
        });
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>

        <input node_ref=input_ref placeholder="What is your name?"/>

        <button on:click=move |_| {
            let value = input_ref.get().unwrap().value();
            spawn_local(async move {
                let uppercase_text = say_hi(value.clone()).await.unwrap_or_else(|e| e.to_string());
                set_shout_result.set(uppercase_text);

                let _ = add_greeted_people(value).await;
                let people = get_greeted_people().await.unwrap_or_else(|e| vec![e.to_string()]);
                let people_string: String = people.join(", ");
                set_people.set( people_string );
            });
        }>
            {shout_result}
        </button>

        <br/><br/> 

        <div>
            <h2>People greered so far:</h2>
            { greeted_people }
        </div>
    }
}
