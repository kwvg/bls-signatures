GO=go
CGO_ENABLED := 1

OS := $(shell uname -s)
ARCH := $(shell uname -m)

COVERAGE_OUTPUT ?= coverage.out

MAKEFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))

BUILD_DIR=$(PWD)/../build
CURR_DIR := $(dir $(MAKEFILE_PATH))
SRC_DIR=$(PWD)/../src

GMP_PREFIX := /usr/local

ifeq ("$(OS)", "Darwin")
  ifneq ($(wildcard /opt/local/bin/port),)
    GMP_PREFIX := /opt/local
  endif
ifeq ("$(ARCH)", "arm64")
  ifneq ($(wildcard /opt/homebrew/bin/brew),)
    GMP_PREFIX := /opt/homebrew
  endif
endif
else ifeq ("$(OS)", "Linux")
  ifneq ($(wildcard /home/linuxbrew/.linuxbrew/bin/brew),)
    GMP_PREFIX := /home/linuxbrew/.linuxbrew
  endif
endif

MIMALLOC_LIB := mimalloc-secure

ifneq ($(wildcard $(MIMALLOC_PREFIX)/libmimalloc-secure-debug.a),)
  MIMALLOC_LIB := mimalloc-secure-debug
endif

CGO_CXXFLAGS ?= "\
-I$(CURR_DIR)../include \
-I$(GMP_PREFIX)/include \
-I$(CURR_DIR)../depends/mimalloc/include \
-I$(CURR_DIR)../build/depends/relic/include \
-I$(CURR_DIR)../depends/relic/include"

CGO_LDFLAGS ?= "\
-L$(CURR_DIR)../build/src -ldashbls \
-L$(GMP_PREFIX)/lib -lgmp \
-L$(CURR_DIR)../build/depends/mimalloc -l$(MIMALLOC_LIB) \
-L$(CURR_DIR)../build/depends/relic/lib -lrelic_s"

.PHONY: default vet test clean config

default: prepare vet test clean

prepare:
	@mkdir -p ../build/src/dashbls
	cp -rv ../src/* ../build/src/dashbls

fmt:  ## Run go fmt to format Go files
	$(GO) fmt

test: ## Run a basic test suite
	CGO_CXXFLAGS=$(CGO_CXXFLAGS) CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) test

cover:  ## Run tests and generate test coverage file, output coverage results and HTML coverage file.
	CGO_CXXFLAGS=$(CGO_CXXFLAGS) CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) test -coverprofile $(COVERAGE_OUTPUT)
	CGO_CXXFLAGS=$(CGO_CXXFLAGS) CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) tool cover -func=$(COVERAGE_OUTPUT)
	CGO_CXXFLAGS=$(CGO_CXXFLAGS) CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) tool cover -html=$(COVERAGE_OUTPUT)
	rm -f $(COVERAGE_OUTPUT)

vet:  ## Go vet all project code
	CGO_CXXFLAGS=$(CGO_CXXFLAGS) CGO_LDFLAGS=$(CGO_LDFLAGS) go vet ./...

help: ## Show This Help
	@for line in $$(cat Makefile | grep "##" | grep -v "grep" | sed  "s/:.*##/:/g" | sed "s/\ /!/g"); do verb=$$(echo $$line | cut -d ":" -f 1); desc=$$(echo $$line | cut -d ":" -f 2 | sed "s/!/\ /g"); printf "%-30s--%s\n" "$$verb" "$$desc"; done

config: ## Display build configuration
	@echo "OS:           $(OS)"
	@echo "ARCH:         $(ARCH)"
	@echo ""
	@echo "CC:           $${CC:-<not set>}"
	@echo "CXX:          $${CXX:-<not set>}"
	@echo "GO:           $(GO)"
	@echo ""
	@echo "GOROOT:       $${GOROOT:-<not set>}"
	@echo ""
	@echo "CGO_CXXFLAGS: $(CGO_CXXFLAGS)"
	@echo "CGO_LDFLAGS:  $(CGO_LDFLAGS)"

clean: ## Clean up transient (generated) files
	$(GO) clean
	rm -f $(COVERAGE_OUTPUT)
