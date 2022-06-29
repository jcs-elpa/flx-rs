EMACS ?= emacs
EASK ?= eask

.PHONY: clean checkdoc lint package install compile test

ci: clean package install compile test

package:
	@echo "Packaging..."
	$(EASK) package

install:
	@echo "Installing..."
	$(EASK) install

compile:
	@echo "Compiling..."
	$(EASK) compile

test:
	@echo "Testing..."
	$(EASK) test ert ./test/flx-rs-*.el

checkdoc:
	@echo "Run checkdoc..."
	$(EASK) lint checkdoc

lint:
	@echo "Run package-lint..."
	$(EASK) lint package

clean:
	$(EASK) clean-all

activate: package
	$(EASK) install --dev
	$(EASK) load ./test/activate.el
