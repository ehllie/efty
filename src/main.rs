use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "ssr")]  {
    use std::env;

    use anyhow::Result;
    use axum::Router;
    use dotenvy::dotenv;
    use efty::app::*;
    use efty::discord;
    use efty::fileserv::file_and_error_handler;
    use efty::state::AppState;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let discord_token = env::var("DISCORD_TOKEN")?;
    let discord_client_id = env::var("DISCORD_CLIENT_ID")?;
    let discord_auth_url = discord::auth_url(discord_client_id.parse::<u64>()?);
    println!("Auth URL: {}", discord_auth_url);
    let mut discord_client = discord::setup_client(&discord_token).await?;

    let conf = get_configuration(None).await?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState::new(leptos_options, &discord_client);

    tokio::spawn(async move {
        discord_client.start().await.expect("Client start error");
    });

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    logging::log!("Listening on: {}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    println!("Client finished running!");

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
