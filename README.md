[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![CELPA](https://celpa.conao3.com/packages/flx-rs-badge.svg)](https://celpa.conao3.com/#/flx-rs)
[![JCS-ELPA](https://raw.githubusercontent.com/jcs-emacs/badges/master/elpa/v/flx-rs.svg)](https://jcs-emacs.github.io/jcs-elpa/#/flx-rs)

# flx-rs
> flx in Rust using dynamic module

[![CI](https://github.com/jcs-elpa/flx-rs/actions/workflows/test.yml/badge.svg)](https://github.com/jcs-elpa/flx-rs/actions/workflows/test.yml)
[![Build](https://github.com/jcs-elpa/flx-rs/actions/workflows/build.yml/badge.svg)](https://github.com/jcs-elpa/flx-rs/actions/workflows/build.yml)

The `rust` implementation is under [jcs090218/flx-rs](https://github.com/jcs090218/flx-rs)
; hence this repo will only contain releases to ELPA and ready-to-use binary files.

## 🔨 Usage

Load by calling the following function,

```el
(flx-rs-load-dyn)
```

Calculate the score with `PATTERN` and `SOURCE`:

```el
(flx-rs-score "something" "some else thing")
```

## 💥 Replace `flx`

To completely replace `flx` with this package, add the following line to your
configuration.

```el
(advice-add 'flx-score :override #'flx-rs-score)
```

## Contribute

[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)
[![Elisp styleguide](https://img.shields.io/badge/elisp-style%20guide-purple)](https://github.com/bbatsov/emacs-lisp-style-guide)
[![Donate on paypal](https://img.shields.io/badge/paypal-donate-1?logo=paypal&color=blue)](https://www.paypal.me/jcs090218)

If you would like to contribute to this project, you may either
clone and make pull requests to this repository. Or you can
clone the project and establish your own branch of this tool.
Any methods are welcome!
