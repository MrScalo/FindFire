use std::thread;
use std::time::{Instant};

mod utils;

//struct for thread to return
struct Website {
    url: String,
    text: String,
}

pub fn main() {
    //setup variables
    let mut url = String::new();
    let mut search = String::new();
    let mut thread_num = String::new();
    let mut same_website = String::new();
    let mut multiple_results_input = String::new();

    //get console input
    println!("Enter url:");
    std::io::stdin().read_line(&mut url).expect("Failed to read line");
    println!("Enter search string:");
    std::io::stdin().read_line(&mut search).expect("Failed to read line");
    println!("Enter amount of threads:");
    std::io::stdin().read_line(&mut thread_num).expect("Failed to read line");
    println!("Search same website (y/n):");
    std::io::stdin().read_line(&mut same_website).expect("Failed to read line");

    //convert input to correct types
    let url: String = url.trim().to_string();
    let search: String = search.trim().to_string();
    let thread_num: i32 = thread_num.trim().parse::<i32>().unwrap();
    let same_website: bool = same_website.trim().to_string() == "y";

    let mut multiple_results: bool = false;
    if same_website {
        println!("Multiple results (y/n):");
        std::io::stdin().read_line(&mut multiple_results_input).expect("Failed to read line");
        multiple_results = multiple_results_input.trim().to_string() == "y";
    }

    //execute the main run function
    run(url, search, thread_num, same_website, multiple_results);
}

//main run function
fn run(mut url: String, search: String, threads: i32, same_website: bool, multiple_results: bool) {
    let mut found_urls: Vec<String> = Vec::new();
    let mut link_queue: Vec<String> = Vec::new();
    let mut visited_links: Vec<String> = Vec::new();
    let mut run: i32 = 0;

    //start timer
    let start_time = Instant::now();

    //push start url to link_queue
    if url.ends_with('/') {
        url.pop();
    }
    link_queue.push(url);

    //console output
    println!("\nSearching for \"{}\"", search);
    println!("Starting search at \"{}\"", link_queue[0]);
    println!("----------");

    while found_urls.len() == 0 || multiple_results {
        //console output
        run += 1;
        println!("Run: {}", run);

        //stop if link_queue is empty
        if link_queue.len() == 0 {
            break;
        }

        //all threads are stored in handles vector
        let mut handles = vec![];

        //a thread takes in a link from the link_queue and
        //returns the websites html text

        for i in 0..threads {
            if link_queue.len() > 0 {
                //get link from link_queue and push it to visited_links
                let thread_url = link_queue[0].clone();
                link_queue.remove(0);
                visited_links.push(thread_url.clone());

                //create thread
                handles.push(thread::spawn(move || {
                    //get the text of the website
                    let mut text: String = "".to_string();
                    let res:  Result<String, Box<dyn std::error::Error>> = utils::text_of_website(thread_url.clone(), i);
                    if !res.is_err() {
                        text = res.unwrap();
                    }

                    //send text to main thread
                    let website: Website = Website {
                        url: thread_url.clone(),
                        text: text.clone(),
                    };

                    //return website
                    website
                }));
            }
        }

        //wait for every thread to finish
        for handle in handles {
            //get website from the thread
            let website: Website = handle.join().unwrap();

            //get the url
            let url: String = website.url.clone();

            //get the text
            let mut text: String = website.text.clone();

            //add all links from the text to the link_queue
            link_queue.append(&mut utils::links_of_text(text.clone(), url.clone(), link_queue.clone(), visited_links.clone(), same_website));

            //clear the texts from html tags
            text = utils::clear_text(text);

            //check if the search string is in the texts
            if utils::string_includes(text.clone(), search.clone(), true) {
                found_urls.push(url.clone());

                if !multiple_results {
                    break;
                }
            }
        }
    }

    //stop the timer and calculate the time
    let duration = start_time.elapsed();
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;

    //console output
    println!("----------");
    println!("Searched {} websites in {}m {}s", visited_links.len(), minutes, seconds);
    if found_urls.len() != 0 {
        for url in found_urls {
            println!("Found \"{}\" in \"{}\"", search, url);
        }
    } else {
        println!("No more links to visit");
    }
}
