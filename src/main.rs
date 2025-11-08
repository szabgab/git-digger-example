use git_digger::Repository;
use std::path::Path;

fn main() {
    get_github_com_no_slash_at_the_end();
    get_github_com_link_to_a_file();
    https_gitlab_com();
    converting_to_lowercase_gitlab_com();

    check_good_url();
    check_missing_url();

    clone_missing_repo();
    clone_this_repo();
}

fn get_github_com_no_slash_at_the_end() {
    let repo = Repository::from_url("https://github.com/szabgab/rust-digger").unwrap();

    assert_eq!(repo.url(), "https://github.com/szabgab/rust-digger");
    assert_eq!(repo.get_owner(), "szabgab");
    //    assert_eq!(repo.repo, "rust-digger");
    assert!(repo.is_github());
    assert!(!repo.is_gitlab());

    let expected = Repository::new("github.com", "szabgab", "rust-digger");
    assert_eq!(repo, expected);

    let root = Path::new("/tmp");
    assert_eq!(
        repo.path(root).to_str(),
        Some("/tmp/github.com/szabgab/rust-digger")
    );
}

fn get_github_com_link_to_a_file() {
    let repo = Repository::from_url(
        "https://github.com/crypto-crawler/crypto-crawler-rs/tree/main/crypto-market-type",
    )
    .unwrap();
    assert_eq!(
        repo.url(),
        "https://github.com/crypto-crawler/crypto-crawler-rs"
    );
    assert!(repo.is_github());

    assert_eq!(
        repo,
        Repository::new("github.com", "crypto-crawler", "crypto-crawler-rs",)
    );
}

fn https_gitlab_com() {
    let repo = Repository::from_url("https://gitlab.com/szabgab/rust-digger").unwrap();
    assert_eq!(repo.url(), "https://gitlab.com/szabgab/rust-digger");
    assert!(!repo.is_github());
    assert!(repo.is_gitlab());
    assert_eq!(repo.get_owner(), "szabgab");

    assert_eq!(
        repo,
        Repository::new("gitlab.com", "szabgab", "rust-digger")
    );
}

fn converting_to_lowercase_gitlab_com() {
    let root = Path::new("/tmp");
    let repo = Repository::from_url("https://gitlab.com/Szabgab/Rust-digger/").unwrap();
    assert_eq!(
        repo,
        Repository::new("gitlab.com", "szabgab", "rust-digger")
    );
    assert_eq!(repo.url(), "https://gitlab.com/szabgab/rust-digger");
    assert_eq!(repo.get_owner(), "szabgab");
    //    assert_eq!(repo.repo, "rust-digger");
    assert_eq!(
        repo.path(root).to_str(),
        Some("/tmp/gitlab.com/szabgab/rust-digger")
    );
}

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

fn check_good_url() {
    let repo = Repository::from_url("https://github.com/szabgab/git-digger").unwrap();
    assert!(repo.check_url());
}

fn check_missing_url() {
    let repo = Repository::from_url("https://github.com/szabgab/no-such-repo").unwrap();
    assert!(!repo.check_url());
}

fn clone_missing_repo() {
    let temp_folder = tempfile::tempdir().unwrap();
    let repo = Repository::from_url("https://github.com/szabgab/no-such-repo").unwrap();
    repo.update_repository(Path::new(temp_folder.path()), true)
        .unwrap();
    let owner_path = temp_folder.path().join("github.com").join("szabgab");
    assert!(owner_path.exists());
    assert!(!owner_path.join("no-such-repo").exists());
}

fn clone_this_repo() {
    let temp_folder = tempfile::tempdir().unwrap();
    let repo = Repository::from_url("https://github.com/szabgab/git-digger").unwrap();
    repo.update_repository(Path::new(temp_folder.path()), true)
        .unwrap();
    let owner_path = temp_folder.path().join("github.com").join("szabgab");
    assert!(owner_path.exists());
    assert!(owner_path.join("git-digger").exists());
}
