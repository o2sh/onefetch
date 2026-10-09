use criterion::{Criterion, criterion_group, criterion_main};
use gix::{ThreadSafeRepository, open};
use onefetch::cli::Cli;
use onefetch::info::{InfoOptions, build_info};
use std::hint::black_box;

fn bench_repo_info(c: &mut Criterion) {
    let name = "make_repo.sh".to_string();
    let repo_path = gix_testtools::scripted_fixture_read_only(name)
        .unwrap()
        .join("repo");
    let repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated()).unwrap();
    let config: Cli = Cli {
        input: repo.path().to_path_buf(),
        ..Default::default()
    };
    let options = InfoOptions::from(&config);

    c.bench_function("get repo information", |b| {
        b.iter(|| {
            let result = black_box(build_info(&options));
            assert!(result.is_ok());
        });
    });
}

criterion_group!(benches, bench_repo_info);
criterion_main!(benches);
