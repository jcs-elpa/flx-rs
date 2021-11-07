[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![CELPA](https://celpa.conao3.com/packages/flx-rs-badge.svg)](https://celpa.conao3.com/#/flx-rs)

# flx-rs
> flx in Rust using dynamic module

[![CI](https://github.com/jcs-elpa/flx-rs/actions/workflows/test.yml/badge.svg)](https://github.com/jcs-elpa/flx-rs/actions/workflows/test.yml)
[![Build](https://github.com/jcs-elpa/flx-rs/actions/workflows/build.yml/badge.svg)](https://github.com/jcs-elpa/flx-rs/actions/workflows/build.yml)

## 🔨 Usage

Load by calling the following function,

```el
(flx-rs-load-dyn)
```

Calculate the score with `PATTERN` and `SOURCE`:

```el
(flx-rs-score "something" "some else thing")
```

## Replace `flx`

To completely replace `flx` with this package, add the following line to your
configuration.

```el
(advice-add 'flx-score :override #'flx-rs-score)
```

## 🔗 Links

* https://github.com/jcs090218/flx-rs

## Contribution

If you would like to contribute to this project, you may either
clone and make pull requests to this repository. Or you can
clone the project and establish your own branch of this tool.
Any methods are welcome!
