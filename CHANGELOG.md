# [](https://github.com/SirWindfield/markdown-composer/compare/v0.3.0...v) (2020-08-28)

# [0.3.0](https://github.com/SirWindfield/markdown-composer/compare/v0.2.0...v0.3.0) (2020-08-28)

### Features

- add transformation traits to public API ([db31f55](https://github.com/SirWindfield/markdown-composer/commit/db31f553d91f5f86f2cc2704474fa4bebae81e05)), closes [#14](https://github.com/SirWindfield/markdown-composer/issues/14)

### BREAKING CHANGES
- change List implementation ([0ec1eda](https://github.com/SirWindfield/markdown-composer/commit/0ec1eda53e9fd7c2c512299ba4dfe7db96130339)), closes [#12](https://github.com/SirWindfield/markdown-composer/issues/12)
  - `List::add` got removed in favor of
    direct field access. `List#ordered` got removed in
    favor of `ListType`.
- rename `github` extension feature ([4483679](https://github.com/SirWindfield/markdown-composer/commit/4483679aef1d45f758308dd7b3a460e8674f3479))
  - The feature is now called `extension-github`
- remove preliminary mark ([9e922d2](https://github.com/SirWindfield/markdown-composer/commit/9e922d29833f3bb2d5957613b7aff07a9df4ddcc)), closes [#9](https://github.com/SirWindfield/markdown-composer/issues/9)
  - Remove `PRELIMINARY_MARK` and
  `Markdown#with_remark` creator method.
- rename transform trait functions ([23c44e4](https://github.com/SirWindfield/markdown-composer/commit/23c44e4e0df314cb02217c86a74a718ba5fed811)), closes [#10](https://github.com/SirWindfield/markdown-composer/issues/10)
  - The trait functions now use a `to_` naming scheme.

# [0.2.0](https://github.com/SirWindfield/markdown-composer/compare/v0.1.1...v0.2.0) (2020-08-17)

### Features

- add footer links ([59089ab](https://github.com/SirWindfield/markdown-composer/commit/59089abbe03958c0a631effdacd00f73ddf90d15)), closes [#5](https://github.com/SirWindfield/markdown-composer/issues/5)
- add footer support to `Image` ([35688a6](https://github.com/SirWindfield/markdown-composer/commit/35688a6a497924e3eb2e2e8e8371d77b13abc192)), closes [#7](https://github.com/SirWindfield/markdown-composer/issues/7)

- refactor!: use `tousize` instead of custom implementation ([4a88fbc](https://github.com/SirWindfield/markdown-composer/commit/4a88fbc9190f93b200bb04551979b30fb59cc5ff))

### BREAKING CHANGES

- The `utils` module has been removed and is not part of
  the public API anymore.

## [0.1.1](https://github.com/SirWindfield/markdown-composer/compare/v0.1.0...v0.1.1) (2020-08-16)

### Features

- add `Image` element ([1e015ae](https://github.com/SirWindfield/markdown-composer/commit/1e015ae336784ffc296607c69d1862fedbadaf07)), closes [#4](https://github.com/SirWindfield/markdown-composer/issues/4)
- add `Strikethrough` to GitHub extension ([9a7b244](https://github.com/SirWindfield/markdown-composer/commit/9a7b244937a67aaf99673573f1b31ed2cc329437)), closes [#3](https://github.com/SirWindfield/markdown-composer/issues/3)
- add rest of text transformation traits ([c7ab1ef](https://github.com/SirWindfield/markdown-composer/commit/c7ab1ef66b720b2f07e8e85578345708b0327a71)), closes [#1](https://github.com/SirWindfield/markdown-composer/issues/1)

# [0.1.0](https://github.com/SirWindfield/markdown-composer/compare/v0.0.0...v0.1.0) (2020-08-15)

# 0.0.0 (2020-08-07)
