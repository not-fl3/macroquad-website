(setq inhibit-x-resources 't)
(setq inhibit-startup-screen t)
(setq frame-resize-pixelwise 't)
(setq kill-whole-line 't)
(setq sentence-end-double-space nil)
(setq vc-follow-symlinks t)
(setq backup-directory-alist `(("." . ,temporary-file-directory)))
(setq auto-save-file-name-transforms
      `((".*" ,temporary-file-directory t)))

(setq ring-bell-function 'ignore)
(setq global-hl-line-mode nil)

(fset 'yes-or-no-p 'y-or-n-p)
;(set-frame-parameter nil 'fullscreen 'fullboth)

(global-undo-tree-mode t)
(cua-mode t)
(tool-bar-mode -1)
(menu-bar-mode -1)
(scroll-bar-mode -1)
(column-number-mode t)
(blink-cursor-mode -1)
;(global-hl-line-mode t)
(show-paren-mode 1)
(auto-save-visited-mode t)
(add-hook 'text-mode-hook '(lambda () (flyspell-mode t)))

(eval-after-load "cc-mode"
  '(progn
     (define-key c-mode-map (kbd "C-c C-c") 'compile)))
    ;(define-key c-mode-map (kbd "C-c C-c") 'projectile-compile-project)))

(global-set-key (kbd "C-c C-c") 'compile)

(global-set-key (kbd "C-S-p") 'projectile-find-file)

(global-set-key (kbd "C-k") ctl-x-map)
(global-set-key (kbd "C-x") nil)
(global-set-key (kbd "C-z") 'undo-tree-undo)
(global-set-key (kbd "C-y") 'undo-tree-redo)
(global-set-key (kbd "C-f") 'isearch-forward)
(global-set-key (kbd "C-s") 'isearch-forward)
(global-set-key (kbd "C-k k") 'kill-this-buffer)
(define-key isearch-mode-map (kbd "C-f") 'isearch-repeat-forward)

(global-set-key (kbd "C-S-k") 'kill-whole-line)
(global-set-key (kbd "C-k a") 'mark-whole-buffer)

(global-set-key (kbd "C-S-f") 'ripgrep-regexp)


;; MACRO

(global-set-key [f5] 'call-last-kbd-macro)

;; QUIT CONFIRMATION

(setq confirm-kill-emacs 'y-or-n-p)


;; WINDOWS NAVIGATION

(global-set-key (kbd "C-k <up>") #'windmove-up)
(global-set-key (kbd "C-k <down>") #'windmove-down)
(global-set-key (kbd "C-k <left>") #'windmove-left)
(global-set-key (kbd "C-k <right>") #'windmove-right)

;; SPACE INSTEAD OF TABS

(setq-default indent-tabs-mode nil)

(global-set-key (kbd "C-.") #'xref-find-definitions)
(global-set-key (kbd "C-,") #'xref-pop-marker-stack)

(setq-default tab-width 4)
(setq c-basic-offset 4)

(set-face-attribute 'default nil :height 170)

;(require 'package)
;(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)
;(package-initialize)
;(custom-set-variables
; '(package-selected-packages '(undo-tree ripgrep projectile rust-mode)))
;(custom-set-faces)

