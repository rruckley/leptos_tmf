//! AccountRef 
//! 
//! 
use leptos::*;

use tmflib::tmf666::AccountRef;

#[component]
pub fn account_ref(_item : AccountRef) -> impl IntoView {
    let href = String::from("http://fake.com");
    let name = String::from("FakeName");
    view!{
        <a href={href}>{name}</a>
    }
}