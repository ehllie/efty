use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/efty.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main class="bg-blue-500 h-screen">
                <Routes>
                    <Route path="" view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <h2>"Discord Members:"</h2>
        <Await future=|| discord_members() let:members>
            {match members {
                Ok(members) => {
                    view! {
                        <ul>
                            {members
                                .iter()
                                .map(|member| view! { <li>{member}</li> })
                                .collect::<Vec<_>>()}
                        </ul>
                    }
                        .into_view()
                }
                Err(_) => view! { <p>"Error loading"</p> }.into_view(),
            }}
        </Await>
    }
}

#[server(DiscordMembers, "/api")]
pub async fn discord_members() -> Result<Vec<String>, ServerFnError> {
    use crate::state::AppState;

    let discord_client = expect_context::<AppState>().discord_client;
    let mut members = Vec::new();
    for guild_id in discord_client.cache.guilds() {
        let guild = match discord_client.http.get_guild(guild_id).await {
            Ok(guild) => guild,
            Err(_) => continue,
        };
        let guild_members = match guild.members(&discord_client.http, None, None).await {
            Ok(members) => members,
            Err(_) => continue,
        };

        members.extend(guild_members.into_iter().map(|member| member.user.name));
    }
    Ok(members)
}
