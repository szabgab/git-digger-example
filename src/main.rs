use std::path::Path;
use git_digger::Repository;

fn main() {
    test_get_github_com_no_slash_at_the_end();
}


fn test_get_github_com_no_slash_at_the_end() {
    let root = Path::new("/tmp");
    //let expected = Repository::new("github.com", "szabgab", "rust-digger");

    let repo = Repository::from_url("https://github.com/szabgab/rust-digger").unwrap();
    //assert_eq!(repo, expected);
    assert_eq!(repo.url(), "https://github.com/szabgab/rust-digger");
//    assert_eq!(repo.owner, "szabgab");
//    assert_eq!(repo.repo, "rust-digger");
    assert_eq!(
        repo.path(root).to_str(),
        Some("/tmp/github.com/szabgab/rust-digger")
    );
    assert!(repo.is_github());
    assert!(!repo.is_gitlab());
}

//    // test https github.com link to a file
//    let repo = Repository::from_url(
//        "https://github.com/crypto-crawler/crypto-crawler-rs/tree/main/crypto-market-type",
//    )
//    .unwrap();
//    assert_eq!(
//        repo,
//        Repository::new("github.com", "crypto-crawler", "crypto-crawler-rs",)
//    );
//    assert_eq!(
//        repo.url(),
//        "https://github.com/crypto-crawler/crypto-crawler-rs"
//    );
//    assert!(repo.is_github());
//
//    // test https gitlab.com
//    let repo = Repository::from_url("https://gitlab.com/szabgab/rust-digger").unwrap();
//    assert_eq!(
//        repo,
//        Repository::new("gitlab.com", "szabgab", "rust-digger")
//    );
//    assert_eq!(repo.url(), "https://gitlab.com/szabgab/rust-digger");
//    assert!(!repo.is_github());
//    assert!(repo.is_gitlab());
//
//    // test converting to lowercase  gitlab.com
//    let repo = Repository::from_url("https://gitlab.com/Szabgab/Rust-digger/").unwrap();
//    assert_eq!(
//        repo,
//        Repository::new("gitlab.com", "szabgab", "rust-digger")
//    );
//    assert_eq!(repo.url(), "https://gitlab.com/szabgab/rust-digger");
//    assert_eq!(repo.owner, "szabgab");
//    assert_eq!(repo.repo, "rust-digger");
//    assert_eq!(
//        repo.path(root).to_str(),
//        Some("/tmp/gitlab.com/szabgab/rust-digger")
//    );
//
//    // test salsa
//    let repo = Repository::from_url("https://salsa.debian.org/szabgab/rust-digger/").unwrap();
//    assert_eq!(
//        repo,
//        Repository::new("salsa.debian.org", "szabgab", "rust-digger")
//    );
//    assert_eq!(repo.url(), "https://salsa.debian.org/szabgab/rust-digger");
//    assert_eq!(repo.owner, "szabgab");
//    assert_eq!(repo.repo, "rust-digger");
//    assert_eq!(
//        repo.path(root).to_str(),
//        Some("/tmp/salsa.debian.org/szabgab/rust-digger")
//    );
//    assert!(!repo.is_github());
//    assert!(repo.is_gitlab());
//
//    // test incorrect URL
//    let res = Repository::from_url("https://blabla.com/");
//    assert!(res.is_err());
//    assert_eq!(
//        res.unwrap_err().to_string(),
//        "No match for repo in 'https://blabla.com/'"
//    );



// fn test_check_good_url() {
//     let repo = Repository::from_url("https://github.com/szabgab/git-digger").unwrap();
//     assert!(repo.check_url());
// }
// 
// fn test_check_missing_url() {
//     let repo = Repository::from_url("https://github.com/szabgab/no-such-repo").unwrap();
//     assert!(!repo.check_url());
// }

//fn test_clone_missing_repo() {
//    let temp_folder = tempfile::tempdir().unwrap();
//    let repo = Repository::from_url("https://github.com/szabgab/no-such-repo").unwrap();
//    repo.update_repository(Path::new(temp_folder.path()), true)
//        .unwrap();
//    let owner_path = temp_folder.path().join("github.com").join("szabgab");
//    assert!(owner_path.exists());
//    assert!(!owner_path.join("no-such-repo").exists());
//}
//
//fn test_clone_this_repo() {
//    let temp_folder = tempfile::tempdir().unwrap();
//    let repo = Repository::from_url("https://github.com/szabgab/git-digger").unwrap();
//    repo.update_repository(Path::new(temp_folder.path()), true)
//        .unwrap();
//    let owner_path = temp_folder.path().join("github.com").join("szabgab");
//    assert!(owner_path.exists());
//    assert!(owner_path.join("git-digger").exists());
//}
