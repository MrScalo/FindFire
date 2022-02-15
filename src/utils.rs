use regex::Regex;

//get the html text of a website
#[tokio::main]
pub async fn text_of_website(url: String, thread_num: i32) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    //get the response
    let res = client.get(url.clone())
        .send()
        .await?
        .text()
        .await?;

    //if res length > 1000000 return error
    if res.len() > 1000000 {
        return Err("Res too long".into());
    }

    //console output
    println!("Thread {} -> Searching: {}", thread_num, url.clone());

    //filter out the text between <body> and </body>
    let re = Regex::new(r"<body.*?>").unwrap();
    let body_start = re.find(&res);
    if body_start.is_none() {
        return Err("First body not found".into());
    }
    let body_start = body_start.unwrap().end();
    let body_end = res.find("</body>");
    let body_end = body_end.unwrap_or(0);

    if body_start > body_end {
        return Err("Probably last body not found".into());
    }

    let body = &res[body_start..body_end + 7];
    let body: String = body.to_string();

    //return the text
    Ok(body)
}

//clear the text from html tags
pub fn clear_text(text: String) -> String {
    //remove all html tags from body
    let re = Regex::new(r"<[^>]*>").unwrap();
    let mut clear_text: String = re.replace_all(&text, "").to_string();

    //remove all whitespaces
    let re = Regex::new(r"\s+").unwrap();
    clear_text = re.replace_all(&clear_text, " ").to_string();

    clear_text
}

//check if text includes search string
pub fn string_includes(text: String, search: String, ignore_case: bool) -> bool {
    if ignore_case {
        text.to_lowercase().contains(search.to_lowercase().as_str())
    } else {
        text.contains(&search)
    }
}

//get all links from the text
pub fn links_of_text(html_body: String, url: String, link_queue: Vec<String>, visited_links: Vec<String>, same_website: bool) -> Vec<String> {
    let mut new_link_queue: Vec<String> = Vec::new();

    //prepare url
    let url_parts: Vec<&str> = url.split("/").collect();
    let url = url_parts[0..3].join("/");

    //find all links
    let re = Regex::new(r#"href="(.*?)""#).unwrap();
    for cap in re.captures_iter(&html_body) {
        let mut link: String = cap[1].to_string();

        //remove latest backslash if exists in link
        if link.ends_with('/') {
            link.pop();
        }

        //check if it is not a website
        let endings: Vec<&str> = ["png", "jpg", "svg", "ico", "pdf", "css"].to_vec();
        let ending: &str = link.split(".").last().unwrap();
        if endings.contains(&ending.to_lowercase().as_str()) {
            continue;
        }

        //check link
        let mut valid_link = (&link).to_string();
        if link.starts_with("/") {
            //relative link
            let mut link_url: String = url.clone();
            link_url.push_str(&link);
            valid_link = link_url
        }

        //check if link starts with http
        if !valid_link.starts_with("http") {
            continue;
        }

        //when same_website check if link is from the same website
        if same_website && !valid_link.starts_with(&url) {
            continue;
        }

        //check if valid_link is not already in link_queue or visited_links
        if !link_queue.contains(&valid_link) && !visited_links.contains(&valid_link) && *&valid_link.len() as i32 > 0 {
            new_link_queue.push(valid_link);
        }
    }

    new_link_queue
}