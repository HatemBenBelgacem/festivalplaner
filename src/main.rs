use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/login")]
    Login {},
    
    // Alles innerhalb dieses Layouts hat die Seitenleiste
    #[layout(SidebarLayout)]
    #[route("/")]
    Dashboard {},
    #[route("/adressen")]
    Adressverwaltung {},
    #[route("/projekte")]
    Projektverwaltung {},
    #[route("/bars")]
    Barverwaltung {},
    #[route("/mitarbeiter")]
    Mitarbeiterverwaltung {},
    #[route("/kuenstler")]
    Kuenstlerverwaltung {},
    #[route("/benutzer")]
    Benutzerverwaltung {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Login Page
#[component]
fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let navigator = use_navigator();

    rsx! {
        div { class: "login-container",
            div { class: "card login-box",
                h2 { style: "text-align: center; margin-bottom: 24px;", "Google-Style Login" }
                input {
                    class: "input-field",
                    placeholder: "Benutzername",
                    value: "{username}",
                    oninput: move |e| username.set(e.value()),
                }
                input {
                    class: "input-field",
                    r#type: "password",
                    placeholder: "Passwort",
                    value: "{password}",
                    oninput: move |e| password.set(e.value()),
                }
                button {
                    class: "btn-primary",
                    style: "width: 100%; margin-top: 16px;",
                    onclick: move |_| {
                        // Hier würde die Authentifizierung stattfinden
                        navigator.push(Route::Dashboard {});
                    },
                    "Anmelden"
                }
            }
        }
    }
}

/// Das Layout für alle authentifizierten Seiten (enthält die Navigation)
#[component]
fn SidebarLayout() -> Element {
    rsx! {
        div { id: "app-container",
            nav { id: "sidebar",
                Link { to: Route::Dashboard {}, "Dashboard" }
                Link { to: Route::Adressverwaltung {}, "Adressen" }
                Link { to: Route::Projektverwaltung {}, "Projekte" }
                Link { to: Route::Barverwaltung {}, "Bars" }
                Link { to: Route::Mitarbeiterverwaltung {}, "Mitarbeiter" }
                Link { to: Route::Kuenstlerverwaltung {}, "Künstler" }
                Link { to: Route::Benutzerverwaltung {}, "Benutzer" }

                div { style: "margin-top: auto; padding: 24px;",
                    Link { to: Route::Login {}, "Abmelden" }
                }
            }
            main { id: "main-content", Outlet::<Route> {} }
        }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        div {
            h1 { "Dashboard" }
            div { class: "card",
                p { "Willkommen im Eventplaner. Wählen Sie ein Modul aus der Seitenleiste." }
            }
        }
    }
}

#[component]
fn Adressverwaltung() -> Element {
    let addresses = use_resource(|| async move { get_addresses_from_db().await });

    rsx! {
        div {
            h1 { "Adressverwaltung" }
            div { class: "card",
                button { class: "btn-primary", "Neue Adresse anlegen" }

                match &*addresses.read_unchecked() {
                    Some(Ok(data)) => rsx! {
                        ul { style: "margin-top: 20px;",
                            for address in data {
                                li { "{address}" }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        p { "Fehler beim Laden: {err}" }
                    },
                    None => rsx! {
                        p { "Lädt..." }
                    },
                }
            }
        }
    }
}

#[component]
fn Projektverwaltung() -> Element { rsx! {
    h1 { "Projektverwaltung" }
    div { class: "card",
        p { "Modul für Projekte" }
    }
} }

#[component]
fn Barverwaltung() -> Element { rsx! {
    h1 { "Barverwaltung" }
    div { class: "card",
        p { "Modul für Bars" }
    }
} }

#[component]
fn Mitarbeiterverwaltung() -> Element { rsx! {
    h1 { "Mitarbeiterverwaltung" }
    div { class: "card",
        p { "Modul für Mitarbeiter" }
    }
} }

#[component]
fn Kuenstlerverwaltung() -> Element { rsx! {
    h1 { "Künstlerverwaltung" }
    div { class: "card",
        p { "Modul für Künstler" }
    }
} }

#[component]
fn Benutzerverwaltung() -> Element { rsx! {
    h1 { "Benutzerverwaltung" }
    div { class: "card",
        p { "Modul für Benutzerrechte und Zugänge" }
    }
} }


// --- SERVER FUNKTIONEN FÜR SQLITE ---

#[server(GetAddresses)]
async fn get_addresses_from_db() -> Result<Vec<String>, ServerFnError> {
    // WICHTIG: sqlx darf nur hier innerhalb dieser Server-Funktion aufgerufen und importiert werden!
    // Dadurch wird verhindert, dass WebAssembly (Browser) versucht, die Datenbank-Treiber zu kompilieren.
    
    // Sobald deine SQLite-Datenbank eingerichtet ist, kannst du diesen Code einkommentieren:
    /*
    use sqlx::sqlite::SqlitePoolOptions;
    let pool = SqlitePoolOptions::new()
        .connect("sqlite:eventplaner.db").await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
    let rows = sqlx::query!("SELECT name FROM addresses").fetch_all(&pool).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    */
    
    // Simulierte Daten für den Moment (bis die DB bereit ist):
    Ok(vec![
        "Max Mustermann, Musterstraße 1".to_string(),
        "Konzert AG, Eventplatz 42".to_string(),
    ])
}