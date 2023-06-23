/// Project $name$
/// 
/// Author: $param.author_name$ <$param.author_email_lower_case$>
///


fn main() {
    println!("Welcome to $name$ version {}", env!("CARGO_PKG_VERSION");

    // <% if param.with_serde %>
    println!("aplikasi ini mendukung serde!");
    // <% endif %>
}
