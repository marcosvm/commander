load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "93ce7c9164fc5b9eabef1b9c4a904dff58ad867bf1f479a332ea7d1fe876ca8b",
    strip_prefix = "rules_rust-25f396fef2a08476a0baf5dfee9e01e99d15c0f2",
    urls = [
        # Main branch as of 2021-02-19
        "https://github.com/bazelbuild/rules_rust/archive/25f396fef2a08476a0baf5dfee9e01e99d15c0f2.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("//cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()
