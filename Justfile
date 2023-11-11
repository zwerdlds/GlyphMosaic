default:
    just watch "parallel-iter"

parallel-iter:
    @parallel just quietly -- \
        format \
        test \
        verify \
        update-coverage \
        build-documentation

watch watchtarget:
    cargo watch \
        --clear \
        --shell 'just {{watchtarget}}' \
        --ignore '**/*.svg' \
        --ignore 'lcov.info' \
        --ignore 'README.md' \
        --ignore 'documentation/*.pdf' \
        --ignore '.doc_build/**' \
        --ignore '**/*.pytxcode' \
        --ignore 'dist/' \
        --ignore 'mutants.out*/**'

reinit-workspace:
    cargo install cargo-watch --force
    cargo install cargo-tarpaulin --force
    cargo install cargo-mutants --force
    cargo install cargo-readme --force

test:
    CARGO_TERM_COLOR="always" \
    cargo test \
        --target-dir target/just-test \
        --workspace \
        --quiet \
        --all-targets

quietly recipe:
    @chronic unbuffer just {{recipe}}
    @echo -e "\033[0;32m{{recipe}} exited without error.\033[0m"

verify: verify-check verify-clippy

verify-check:
    RUSTFLAGS="-D warnings" \
    CARGO_TERM_COLOR="always" \
    cargo check \
        --target-dir target/just-check \
        --all-features

verify-clippy: 
    RUSTFLAGS="-D warnings" \
    CARGO_TERM_COLOR="always" \
    cargo clippy \
        --workspace \
        --target-dir target/just-clippy \
        -- \
            --deny clippy::pedantic \
            --deny clippy::correctness \
            --deny clippy::style \
            --deny clippy::complexity

format:
    CARGO_TERM_COLOR="always" \
    cargo fmt

build-documentation:
    make --jobs "$(nproc --all)" documentation

update-coverage:
    CARGO_TERM_COLOR="always" \
    cargo tarpaulin \
        --target-dir target/just-tarpaulin \
        --out Lcov \
        --skip-clean

clean:
    make clean

cp msg:
    git commit -am "{{msg}}"
    git push gh