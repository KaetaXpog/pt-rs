use scraper::Html;

/// If found, this returns Some thing like `?inclbookmarked=0&incldead=1`, 
/// a.k.a query string
pub fn parse_next_href(html: &str) -> Option<String>{
    let html = Html::parse_document(html);
    let pages = html
        .select(&"p.nexus-pagination".try_into().unwrap())
        .next().unwrap();

    pages.select(&"a".try_into().unwrap())
    .find(|ele| {
        ele.select(&"b[title$=Pagedown]".try_into().unwrap())
        .next().is_some()
    })  // if we really find such `a` tag
    .map(|ele| ele.value().attr("href").unwrap().to_owned())
}
