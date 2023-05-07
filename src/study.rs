use rand::Rng;

fn pick_random_study_activity<'a>() -> &'a str {

    let study_material = vec![
        "ZTM",
        "The Book",
        "Build a project with a given topic",
        "Learn a new section",
        "Explore standard library",
    ];

    let random_index = rand::thread_rng().gen_range(0..=study_material.len() - 1);

   &study_material[random_index]
}

pub fn pick_random_ztm_section<'a>() -> &'a str {
    let ztm_sections = vec![
        "ZTM: Rust Fundamentals",
        "ZTM: Making Decisions with Rust",
        "ZTM: Repetition",
        "ZTM: Working with Data",
        "ZTM: Rust's Memory Model",
        "ZTM: Data Collections",
        "ZTM: Expanding Knowledge",
        "ZTM: Fallible Functions",
        "ZTM: Data Collection: HashMap",
        "ZTM: Easier Data Management",
        "ZTM: Managing Code",
    ];

    let random_index = rand::thread_rng().gen_range(0..=ztm_sections.len() - 1);
    &ztm_sections[random_index]
}

pub fn pick_random_the_book_section<'a>() -> &'a str {
    let the_book_sections = vec![
        "The Book: Getting Started",
        "The Book: Programming a Guessing Game",
        "The Book: Common Programming Concepts",
        "The Book: Understanding Ownership",
        "The Book: Using structs to Structure Related Data",
        "The Book: Enums and Pattern Matching",
        "The Book: Managing Growing Projects with Packages, Crates, and Modules",
        "The Book: Common Collections",
        "The Book: Error Handling",
        "The Book: Generic Types, Traits, and Lifetimes",
        "The Book: Writing Automated Tests",
        "The Book: An I/O Project",
    ];

    let random_index = rand::thread_rng().gen_range(0..=the_book_sections.len() - 1);
    &the_book_sections[random_index]
}

fn build_project<'a>() -> &'a str {
    "Build project"
}

fn new_section<'a>() -> &'a str {
    "New section"
}

pub fn pick_random_chapter<'a>() -> Result<&'a str, String> {
    match pick_random_study_activity() {
        "ZTM" => Ok(pick_random_ztm_section()),
        "The Book" => Ok(pick_random_the_book_section()),
        "Build a project with a given topic" => Ok(build_project()),
        "Learn a new section" => Ok(new_section()),
        "Explore standard library" => Ok(explore_standard_library()),
        _ => Err(String::from("This feature has not yet been implemented!")),
    }
}

pub fn pick_random_exploration_type<'a>() -> &'a str {
    let exploration_type = vec!["types", "functionality"];
    let random_index = rand::thread_rng().gen_range(0..=exploration_type.len() - 1);

    exploration_type[random_index]
}

fn explore_standard_library<'a>() -> &'a str {
    "Explore standard library"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_random_chapter() {
        let random_chapter = pick_random_chapter().unwrap_or_else(|err| {
            panic!("err: {err}");
        });


        let chapter = "An I/O Project";

        assert_ne!(random_chapter, chapter, "It should return a random chapter");
    }

    #[test]
    fn it_returns_a_random_study_activity() {
        let random_material = pick_random_study_activity();

        let material = "ZTM";

        assert_ne!(random_material, material, "It should return a random study material");
    }

    #[test]
    fn it_returns_a_random_ztm_chapter() {
        let random_chapter = pick_random_ztm_section();

        let chapter = "Rust Fundamentals";

        assert_ne!(random_chapter, chapter, "It should return a random ZTM chapter");
    }

    #[test]
    fn it_returns_a_random_the_book_chapter() {
        let random_section = pick_random_the_book_section();

        let section = "Getting Started";

        assert_ne!(random_section, section, "It should return a random section in \"The Book\"");
    }

    #[test]
    fn it_returns_build_project() {
        let output = build_project();
        let desired_output = "Build project";

        assert_eq!(desired_output, output, "It should equal Build project.")
    }

    #[test]
    fn it_returns_new_section() {
        let output = new_section();
        let desired_output = "New section";

        assert_eq!(desired_output, output, "It should equal New Section.")
    }

    #[test]
    fn it_returns_explore_standard_library() {
        let output = explore_standard_library();
        let desired_output = "Explore standard library";

        assert_eq!(output, desired_output, "It should equal Explore standard library");
    }
}