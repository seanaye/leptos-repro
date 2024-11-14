use std::ops::Not;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, ProtectedRoute, Route, Router, Routes},
    path, MatchNestedRoutes, StaticSegment,
};
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html lang="en" data-theme="retro">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body class="md:px-6">
                <App />
            </body>
        </html>
    }
}



#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AuthState {
    Anonymous,
    Unverified { user_id: u32 },
    Verified { user_id: u32 },
    
}




#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();


    view! {
        <Stylesheet id="leptos" href="/pkg/tinyrs.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />
        <AppRoutes />
    }
}

#[component]
fn AppRoutes() -> impl IntoView {

    
    #[server]
    async fn load_user_session() -> Result<(), ServerFnError> {
        use axum_extra::extract::CookieJar;
        use leptos_axum::extract;
        use service::AppState;

        let jar: CookieJar = extract().await?;
        dbg!(Owner::current());
        dbg!("Before this");

        let AppState { pool, .. } = expect_context::<AppState>();
        dbg!("cant get here");
        Ok(())
    }

    view! {
        <Await future=load_user_session() let:data>
            <Router>
                <Routes transition=true fallback=|| "Page not found.".into_view()>
                    <ParentRoute path=path!("") view=D>
                        <Route path=path!("/") view=D />
                    </ParentRoute>
                    <ParentRoute path=path!("/signup") view=D>
                        <Route path=path!("") view=D />
                        <Route path=path!("/signup/check-your-email") view=D />
                        <Route path=path!("/signup/confirm-email") view=D />
                    </ParentRoute>
                </Routes>
            </Router>
        </Await>
    }
    
}

#[component]
fn D() -> impl IntoView {
    view! { <div></div> }
}
