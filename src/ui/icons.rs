use leptos::*;

#[component]
pub fn AddPerson(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class={format!("inline w-4 h-4 {}", class)} width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M6 12H7V13H13V12H14V11H15V5H14V4H13V3H7V4H6V5H5V11H6V12ZM7 7H8V6H9V5H11V6H12V7H13V9H12V10H11V11H9V10H8V9H7V7Z" fill="currentColor"/>
            <path d="M17 15H16V14H4V15H3V16H2V21H4V18H5V17H6V16H14V17H15V18H16V19V20V21H18V16H17V15Z" fill="currentColor"/>
            <path d="M20 7V6V5H18V7H16V9H18V11H20V9H22V7H20Z" fill="currentColor"/>
        </svg>
    }
}

#[component]
pub fn CopyDocument(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class={format!("inline w-4 h-4 {}", class)} width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M15 15H9V17H15V15Z" fill="currentColor"/>
            <path d="M15 12H9V14H15V12Z" fill="currentColor"/>
            <path d="M19 7V6H18V5H17V4H16V3H15V2H5V3H4V21H5V22H19V21H20V7H19ZM18 19H17V20H7V19H6V5H7V4H12V10H18V19ZM16 7V8H14V6H15V7H16Z" fill="currentColor"/>
        </svg>
    }
}

#[component]
pub fn GeoTag(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=format!("inline w-5 h-5 {}", class) width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M14 7H10V8H9V12H10V13H14V12H15V8H14V7ZM13 11H11V9H13V11Z" fill="currentColor"/>
            <path d="M19 4V3H18V2H6V3H5V4H4V15H5V16H6V17H7V18H8V19H9V20H10V21H11V22H13V21H14V20H15V19H16V18H17V17H18V16H19V15H20V4H19ZM18 14H17V15H16V16H15V17H14V18H13V19H11V18H10V17H9V16H8V15H7V14H6V6H7V5H8V4H16V5H17V6H18V14Z" fill="currentColor"/>
        </svg>
    }
}

#[component]
pub fn PersonIcon(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class={format!("inline w-5 h-5 {}", class)} width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M8 12H9V13H15V12H16V11H17V5H16V4H15V3H9V4H8V5H7V11H8V12ZM9 7H10V6H11V5H13V6H14V7H15V9H14V10H13V11H11V10H10V9H9V7Z" fill="currentColor"/>
            <path d="M19 16V15H18V14H6V15H5V16H4V21H6V18H7V17H8V16H16V17H17V18H18V21H20V16H19Z" fill="currentColor"/>
        </svg>
    }
}

#[component]
pub fn TextIcon(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class={format!("inline w-4, h-4 {}", class)} width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 3V7H5V5H11V19H9V21H15V19H13V5H19V7H21V3H3Z" fill="black"/>
        </svg>
    }
}
