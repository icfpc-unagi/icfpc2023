# DOCKER_ENVIRONMENT is set to "docker" if it may be inside a Docker container.
# Otherwise, it is set to "host".  If Docker is not installed, it presumes it
# is running inside a container (containerd does not prepare a /.dockerenv).
DOCKER_ENVIRONMENT = $(shell \
	if [ -f /.dockerenv ] || ! which docker >/dev/null 2>/dev/null; then \
		echo docker; \
	else \
		echo host; \
	fi)
# DOCKER_RUN is a command to trigger "docker run".  It may have extra
# command-line arguments.
# NOTE: -it option is added when Makefile is called from a tty.  It enables
# a programmer to kill a docker-run command by Ctrl+C.
DOCKER_RUN = docker run --rm $(shell [ -t 0 ] && echo -it)

DOCKER_REGISTRY = asia-docker.pkg.dev/icfpc-primary/asia

PORT = 8080

export DOCKER_BUILDKIT = 1

###############################################################################
# Basic rules
###############################################################################

.PHONY: test
test: test/rust test/rust/vis

.PHONY:
check:
	@bash ./scripts/check_unagi_password.sh --logtostderr
	@echo 'Successfully passed precondition check.' >&2

.PHONY:
rebase:
	-@rm -rf target
	git fetch
	git rebase origin/main

###############################################################################
# Test rules
###############################################################################

.PHONY: test/rust test/rust/vis
test/rust:
	cargo test
	cargo build --bins

test/rust/vis:
  # TODO: Consider using --chrome that requires chrome installed on CI.
	cd vis && wasm-pack test --node

.PHONY: test/secrets
test/secrets: secrets

.PHONY: test/go
test/go:
	cd go && go test ./...

.PHONY: test/server
test/server: docker/server

###############################################################################
# Rules for secrets
###############################################################################

secrets: secrets/service_account.json FORCE

secrets/%: configs/%.encrypted FORCE
	$(MAKE) secrets/$*@$(DOCKER_ENVIRONMENT)
secrets/%@host: docker/tools FORCE
	$(DOCKER_RUN) -v $(CURDIR):/work -w /work \
		 icfpc-unagi/tools make secrets/$*@docker
secrets/%@docker:
	./bin/decrypt < configs/$*.encrypted > secrets/$*

configs/%.encrypted@: FORCE
	$(MAKE) configs/$*.encrypted@$(DOCKER_ENVIRONMENT)
configs/%.encrypted@host: docker/tools FORCE
	$(DOCKER_RUN) -v $(CURDIR):/work -w /work \
		icfpc-unagi/tools make configs/$*.encrypted@docker
configs/%.encrypted@docker:
	./bin/encrypt < secrets/$* > configs/$*.encrypted

###############################################################################
# Docker rules
###############################################################################

docker/%: FORCE
	cd docker && make $*

push/%: docker/%
	docker tag icfpc-unagi/$* "$(DOCKER_REGISTRY)/$*"
	docker push "$(DOCKER_REGISTRY)/$*"

run/server: docker/server
	docker run -p 0.0.0.0:$(PORT):80 icfpc-unagi/server

###############################################################################
# Generic rules
###############################################################################

.PHONY: FORCE
FORCE:
