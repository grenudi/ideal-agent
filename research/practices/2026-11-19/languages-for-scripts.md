# Languages for our scripts: Mojo, Rust, Python, C#, F# and others (2026-11-19)

Asked by the author: "shell is even worse than python for any scripts that isnt a one off glue ... research and
consider mojo and compare how it holds against rust,python,c#/f#, especially ide intelligence side with
visualstudio being ethalon". Two researchers, web sources dated below; timings measured on the lab machine
(x86_64). The verdict and what changed: decision 2026-11-19-rust-for-scripts-shell-only-glue.

## Mojo (Modular), as of 2026-11

- **Version and stability:** 1.0 shipped 2026-08-11; now 1.2.0 (2026-10-22). Breaking changes still come with
  minor releases: 1.1 removed `fn`, `alias` and `.mojopkg`; 1.2 renamed async/await and moved `std.hashlib`.
  Sources: [changelog](https://mojolang.org/docs/changelog/), [1.1](https://mojolang.org/releases/v1.1.0/),
  [1.2](https://mojolang.org/releases/v1.2.0/).
- **Licence and ownership:** compiler open under Apache-2.0 with LLVM exceptions since 2026-08-18; no outside
  contributions yet. MAX and the runtime still sit under the non-OSI Modular Community License. Qualcomm owns
  Modular since 2026-07-29. Sources: [modular.com/blog/mojo-open-source](https://modular.com/blog/mojo-open-source),
  [Willison](https://simonwillison.net/2026/Aug/18/mojo-is-now-open-source/).
- **Platforms and binaries:** needs glibc 2.34+, and on ARM64 at least a Neoverse N1 core. No static linking.
  Cross-compiling stops at object files and the target must supply the runtime `.so` files.
  Sources: [requirements](https://mojolang.org/docs/requirements/),
  [compilation](https://mojolang.org/docs/tools/compilation.md).
- **Python and purpose:** not a Python superset. Python modules run through a CPython the program loads at run
  time. It is built for CPU/GPU kernels and AI, not general scripting.
  Source: [InfoWorld](https://www.infoworld.com/article/4173158/first-look-at-mojo-1-0.html).
- **IDE:** the VS Code extension has completion, diagnostics and LLDB. It lists no rename or inlay types and
  warns it "may crash". Source: [vscode-mojo](https://github.com/modular/vscode-mojo).
- **Verdict:** not for us today. Not stable, no static aarch64-musl binary, and part of it under a non-OSI
  licence. Re-check in 2027.

## The comparison (scores 1-5; IDE scored against Visual Studio + C# Roslyn = 5)

| | reliability | single-file scripts | aarch64-musl binaries for phones | IDE |
|---|---|---|---|---|
| Rust + rust-script | 5: types, `Result`, pinned inline deps | 4: warm 4 ms, cold 0.41 s (measured) | 5: static musl, ~0.3-0.5 MB, easy cross | 4 in projects; 2-3 in script files |
| Python | 2 | 5 | 2: needs python3 on the phone | 3 |
| C# file-based apps (.NET 10/11) | 5: types, nullable, `#:package X@v` | 4: cold 7.0 s, warm 0.19 s (measured) | 3: NativeAOT 1.27 MB on x64; musl-arm64 needs clang, lld and an Alpine sysroot | 5 (the benchmark) |
| F# .fsx | 5: inference, `Result` | 3: 1.1 s every run (measured), no cache | 2-3: only as a project | 3: Ionide, few refactorings |
| Go | 4: errors easy to ignore; no inline deps | 3 | 5: trivial cross, ~1.5-2 MB | 4: gopls, Delve |
| Nim | 3 | 3 | 4 | 2 |
| Zig | 3: pre-1.0 | 2 | 5 | 2-3 |
| Kotlin .main.kts | 4 | 2: JVM start-up | 1 | 3: Kotlin LSP alpha |
| Deno + TypeScript | 4: types checked only with `--check` | 5 | 1: no musl target, 60-90 MB | 4 |

**Rust single-file scripts:** cargo's built-in scripts (`-Zscript`) are still unstable. Tracking issue
[cargo#12207](https://github.com/rust-lang/cargo/issues/12207) and stabilisation PR
[cargo#16569](https://github.com/rust-lang/cargo/pull/16569) are both open. rust-script remains the stable way.
rust-analyzer parses a script file's frontmatter but does not yet know its dependencies
([rust-analyzer#20854](https://github.com/rust-lang/rust-analyzer/pull/20854)).

**.NET:** file-based apps: <https://learn.microsoft.com/en-us/dotnet/core/sdk/file-based-apps>. NativeAOT cannot
cross-compile to another OS: <https://learn.microsoft.com/en-us/dotnet/core/deploying/native-aot/cross-compile>.

## What is ours today

**Shell in our repositories** (counted 2026-11-19):

| repository | files | lines |
|---|---|---|
| google-walleye-lab | 106 | 6316 |
| docs | 38 | 2169 |
| surya-lab | 26 | 844 |
| lab-tools | 26 | 731 |
| ideal-agent scripts | 13 | ~500 |

Much of it is past one-off glue: loops over data, parsing, checks.

**Not known yet:**
- the NativeAOT musl-arm64 size, and whether it links fully statically;
- the start-up times of Go, Deno and Kotlin (not measured);
- how well rust-analyzer handles a script's dependencies today.
