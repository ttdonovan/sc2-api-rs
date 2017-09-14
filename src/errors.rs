use ws;

error_chain! {
    foreign_links {
        Ws(ws::Error);
    }
}
