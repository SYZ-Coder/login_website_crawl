load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Rust 工具链
http_archive(
    name = "rules_rust",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.20.0/rules_rust-0.20.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rust_register_toolchains")

rust_register_toolchains(
    name = "rust_toolchains",
    host_triple = "x86_64-pc-windows-msvc",
    target_triples = ["x86_64-pc-windows-msvc"],
)

# Bazel Rust 规则
load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

# 依赖仓库
http_archive(
    name = "com_github_spider_rs_spider",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/spider-rs/spider/archive/refs/tags/v1.29.0.tar.gz"],
)

http_archive(
    name = "com_github_reqwest",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/seanmonstar/reqwest/archive/refs/tags/v0.11.24.tar.gz"],
)

http_archive(
    name = "com_github_serde",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/serde-rs/serde/archive/refs/tags/v1.0.197.tar.gz"],
)

http_archive(
    name = "com_github_serde_json",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/serde-rs/json/archive/refs/tags/v1.0.113.tar.gz"],
)

http_archive(
    name = "com_github_tokio",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/tokio-rs/tokio/archive/refs/tags/v1.37.0.tar.gz"],
)

http_archive(
    name = "com_github_url",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/servo/rust-url/archive/refs/tags/2.5.2.tar.gz"],
)

http_archive(
    name = "com_github_log",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/rust-lang/log/archive/refs/tags/0.4.21.tar.gz"],
)

http_archive(
    name = "com_github_env_logger",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/env-logger-rs/env_logger/archive/refs/tags/0.11.3.tar.gz"],
)

http_archive(
    name = "com_github_warp",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/seanmonstar/warp/archive/refs/tags/0.3.6.tar.gz"],
)

http_archive(
    name = "com_github_csv",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/BurntSushi/rust-csv/archive/refs/tags/1.1.6.tar.gz"],
)

http_archive(
    name = "com_github_chrono",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/chronotope/chrono/archive/refs/tags/0.4.22.tar.gz"],
)

http_archive(
    name = "com_github_cron",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/zslayton/cron/archive/refs/tags/0.8.0.tar.gz"],
)

http_archive(
    name = "com_github_regex",
    sha256 = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    urls = ["https://github.com/rust-lang/regex/archive/refs/tags/1.5.4.tar.gz"],
)