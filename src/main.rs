use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// --- 1. DATENMODELLE ---
// Diese Struktur wird zwischen Server und Client geteilt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Adresse {
    pub id: i32,
    pub name: String,
    pub details: String,
}

// --- 2. ROUTING ---
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

// --- 3. KOMPONENTEN ---

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

// === DIE ADRESSVERWALTUNG MIT CRUD-LOGIK ===
#[component]
fn Adressverwaltung() -> Element {
    // Lade die Adressen vom Server
    let mut addresses = use_resource(|| async move { get_addresses_from_db().await });
    
    // State für unser Formular
    let mut name_input = use_signal(String::new);
    let mut details_input = use_signal(String::new);
    
    // Wenn 'Some(id)' gesetzt ist, sind wir im "Bearbeiten"-Modus, bei 'None' im "Hinzufügen"-Modus
    let mut editing_id = use_signal(|| None::<i32>);

    // Speichern-Funktion (Erstellen oder Aktualisieren)
    let save_data = move |_| {
        async move {
            let name = name_input().clone();
            let details = details_input().clone();
            
            if name.is_empty() {
                return; // Leere Eingaben ignorieren
            }

            if let Some(id) = editing_id() {
                // Update existierenden Datensatz
                let _ = update_address_in_db(id, name, details).await;
            } else {
                // Neuen Datensatz anlegen
                let _ = add_address_to_db(name, details).await;
            }

            // Formular zurücksetzen und Liste neu laden
            name_input.set(String::new());
            details_input.set(String::new());
            editing_id.set(None);
            addresses.restart();
        }
    };

    rsx! {
        div {
            h1 { "Adressverwaltung" }

            // --- FORMULAR BEREICH ---
            div { class: "card", style: "margin-bottom: 20px;",
                h3 {
                    if editing_id().is_some() {
                        "Adresse bearbeiten"
                    } else {
                        "Neue Adresse anlegen"
                    }
                }

                input {
                    class: "input-field",
                    placeholder: "Name (z.B. Max Mustermann)",
                    value: "{name_input}",
                    oninput: move |e| name_input.set(e.value()),
                }
                input {
                    class: "input-field",
                    placeholder: "Details (z.B. Musterstraße 1)",
                    value: "{details_input}",
                    oninput: move |e| details_input.set(e.value()),
                }

                div { style: "display: flex; gap: 10px; margin-top: 10px;",
                    button { class: "btn-primary", onclick: save_data,
                        if editing_id().is_some() {
                            "Aktualisieren"
                        } else {
                            "Hinzufügen"
                        }
                    }

                    // Abbrechen-Button nur im Bearbeitungsmodus anzeigen
                    if editing_id().is_some() {
                        button {
                            style: "padding: 10px 16px; border: 1px solid #ccc; border-radius: 6px; background-color: #f9f9f9; cursor: pointer;",
                            onclick: move |_| {
                                editing_id.set(None);
                                name_input.set(String::new());
                                details_input.set(String::new());
                            },
                            "Abbrechen"
                        }
                    }
                }
            }

            // --- LISTEN BEREICH ---
            div { class: "card",
                h3 { "Gespeicherte Adressen" }
                match &*addresses.read_unchecked() {
                    Some(Ok(data)) => rsx! {
                        ul { style: "margin-top: 20px; list-style: none; padding: 0;",
                            for address in data.clone() {
                                li { style: "display: flex; justify-content: space-between; padding: 10px; border-bottom: 1px solid #eee;",
                                    div {
                                        strong { "{address.name} " }
                                        span { "- {address.details}" }
                                    }
                                    div { style: "display: flex; gap: 10px;",
                                        button {
                                            style: "padding: 6px 12px; border: none; border-radius: 4px; background-color: #e0f2fe; color: #0284c7; cursor: pointer;",
                                            onclick: move |_| {
                                                // Werte ins Formular laden
                                                editing_id.set(Some(address.id));
                                                name_input.set(address.name.clone());
                                                details_input.set(address.details.clone());
                                            },
                                            "✏️ Bearbeiten"
                                        }
                                        button {
                                            style: "padding: 6px 12px; border: none; border-radius: 4px; background-color: #fee2e2; color: #dc2626; cursor: pointer;",
                                            onclick: move |_| {
                                                async move {
                                                    let _ = delete_address_from_db(address.id).await;
                                                    addresses.restart(); // Liste neu laden // Liste neu laden
                                                }
                                            },
                                            "🗑️ Löschen"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        p { color: "red", "Fehler beim Laden: {err}" }
                    },
                    None => rsx! {
                        p { "Lädt..." }
                    },
                }
            }
        }
    }
}

// === PLATZHALTER FÜR DIE RESTLICHEN MODULE ===
#[component]
fn Projektverwaltung() -> Element { rsx! {
    h1 { "Projektverwaltung" }
    div { class: "card",
        p { "Hier kommt später die CRUD-Logik für Projekte hin." }
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


// --- 4. SERVER FUNKTIONEN FÜR SQLITE (CRUD) ---

#[server]
async fn get_addresses_from_db() -> Result<Vec<Adresse>, ServerFnError> {
    // Simulierte Daten für den Moment (bis du SQLite verknüpfst):
    Ok(vec![
        Adresse { id: 1, name: "Max Mustermann".to_string(), details: "Musterstraße 1, Berlin".to_string() },
        Adresse { id: 2, name: "Konzert AG".to_string(), details: "Eventplatz 42, Hamburg".to_string() },
    ])
}

#[server]
async fn add_address_to_db(name: String, details: String) -> Result<(), ServerFnError> {
    println!("SERVER: Neue Adresse hinzugefügt: {} - {}", name, details);
    Ok(())
}

#[server]
async fn update_address_in_db(id: i32, name: String, details: String) -> Result<(), ServerFnError> {
    println!("SERVER: Adresse {} aktualisiert: {} - {}", id, name, details);
    Ok(())
}

#[server]
async fn delete_address_from_db(id: i32) -> Result<(), ServerFnError> {
    println!("SERVER: Adresse gelöscht: {}", id);
    Ok(())
}