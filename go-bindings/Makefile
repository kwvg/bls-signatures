GO="go"
CGO_ENABLED := 1

OS := $(shell uname -s)
ARCH := $(shell uname -m)

COVERAGE_OUTPUT ?= coverage.out

MAKEFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))

BUILD_DIR=$(PWD)/../build
CURR_DIR := $(dir $(MAKEFILE_PATH))
SRC_DIR=$(PWD)/../src

DASHBLS_PREFIX := $(CURR_DIR)../build/src
GMP_PREFIX := /usr/local
MIMALLOC_PREFIX := $(CURR_DIR)../build/depends/mimalloc
RELIC_PREFIX := $(CURR_DIR)../build/depends/relic

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

CGO_LDFLAGS ?= "\
-L$(DASHBLS_PREFIX) \
-L$(GMP_PREFIX)/lib \
-L$(MIMALLOC_PREFIX) \
-L$(RELIC_PREFIX)/lib \
-ldashbls \
-lgmp \
-l$(MIMALLOC_LIB) \
-lrelic_s"

CGO_CXXFLAGS ?= "\
-I$(CURR_DIR)../include \
-I$(DASHBLS_PREFIX)/include \
-I$(GMP_PREFIX)/include \
-I$(MIMALLOC_PREFIX)/include \
-I$(RELIC_PREFIX)/include"

.PHONY: default vet test clean

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

clean: ## Clean up transient (generated) files
	$(GO) clean
	rm -f $(COVERAGE_OUTPUT)
