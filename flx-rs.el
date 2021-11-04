;;; flx-rs.el --- flx in Rust using dynamic module  -*- lexical-binding: t; -*-

;; Copyright (C) 2021  Shen, Jen-Chieh
;; Created date 2021-10-28 00:53:46

;; Author: Shen, Jen-Chieh <jcs090218@gmail.com>
;; Description: flx in Rust using dynamic module
;; Keyword: fuzzy
;; Version: 0.1.0
;; Package-Requires: ((emacs "25.1"))
;; URL: https://github.com/jcs-elpa/flx-rs

;; This file is NOT part of GNU Emacs.

;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

;;; Commentary:
;;
;; flx in Rust using dynamic module
;;

;;; Code:


(require 'cl-lib)
(require 'subr-x)

(defconst flx-rs--bin-dir
  (concat (file-name-directory load-file-name) "bin/")
  "Pre-built binaries directory path.")

(defconst flx-rs--dyn-name "flx_rs_core"
  "Dynamic module name.")

;;
;; (@* "Externals" )
;;

(declare-function flx-rs-core-score "flx-rs-core")

;;
;; (@* "Aliases" )
;;

(defun flx-rs-score (str query &rest _)
  "Return best score matching QUERY against STR."
  (when-let ((vec (flx-rs-core-score str query)))
    (append vec nil)))  ; Convert vector to list

;;
;; (@* "Bootstrap" )
;;

;;;###autoload
(defun flx-rs-load-dyn ()
  "Load dynamic module."
  (interactive)
  (unless (featurep 'flx-rs-core)
    (let* ((dyn-name (cl-case system-type
                       ((windows-nt ms-dos cygwin) (concat flx-rs--dyn-name ".dll"))
                       (`darwin (concat "lib" flx-rs--dyn-name ".dylib"))
                       (t (concat "lib" flx-rs--dyn-name ".so"))))
           (dyn-path (concat flx-rs--bin-dir dyn-name)))
      (module-load dyn-path)
      (message "[INFO] Successfully load dynamic module, `%s`" dyn-name))))


(provide 'flx-rs)
;;; flx-rs.el ends here
