use study_helper::study;

fn main() {
    let task = study::pick_random_chapter().unwrap_or_else(|err| {
        panic!("err: {err}");
    });

    let message = match task {
        "Build project" => {
            let random_the_book_section = study::pick_random_the_book_section();
            let random_ztm_section = study::pick_random_ztm_section();
            format!("Project time! Please build a project using the following requirements:
              1.) Build something with the following sections: {random_the_book_section} and {random_ztm_section},
              2.) Have fun! Also write down the sections you struggled with while building the small project!")
        },
        "New section" => {
            format!("New section/topic time! Complete the following steps:
                1.) Read each section
                2.) Take notes on each section
                3.) Practice Coding
                4.) Build a small project for each section
                5.) Dont come back until each of these steps are completed!")
        },
        "Explore standard library" => {
            let what_to_explore = study::pick_random_exploration_type();
            format!("Time to explore the Rust standard library! Explore: {what_to_explore}")
        }
        _ => task.to_owned(),
    };

    println!("Study the following: {}", message.as_str());



}