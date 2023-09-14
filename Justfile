roopes-split-watch:
    tmux \
        set -g mouse on \; \
        new-session 'just watch "parallel-iter"' \; \
        select-pane -T "GlyphMosaic" \; \
        split-window 'just serve' \; \
        select-pane -T "Trunk" \; \
        resize-pane -y 15 \; \
        select-pane -t 0 \; \
        split-window -p 50 'just watch-roopes' \; \
        select-pane -T "Roopes" \; \
        rename-session 'glyphmosaic' \; \
        set -g pane-border-status top \; \

watch-roopes:
    cd ../roopes/ ; \
    just watch "parallel-iter"

kill:
    tmux kill-session -t 'glyphmosaic'

parallel-iter:
    @parallel just quietly -- \
        format \
        test \
        verify \
        update-coverage \
        build-diagrams

watch watchtarget:
    cargo watch \
        --clear \
        --shell 'just {{watchtarget}}' \
        --ignore '**/*.svg' \
        --ignore 'lcov.info' \
        --ignore 'README.md' \
        --ignore 'dist/' \
        --ignore 'mutants.out*/**'

serve:
    trunk serve

reinit-workspace:
    rustup target add wasm32-unknown-unknown
    cargo install cargo-watch --force
    cargo install cargo-tarpaulin --force
    cargo install cargo-doc --force
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

build-diagrams:
    make svg

update-coverage:
    CARGO_TERM_COLOR="always" \
    cargo tarpaulin \
        --target-dir target/just-tarpaulin \
        --out Lcov \
        --skip-clean