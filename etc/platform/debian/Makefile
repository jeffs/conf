var/flag/image-built: Dockerfile
	docker image build -t conf-debian .
	@mkdir -p $(shell dirname $@) && touch $@

run: var/flag/image-built
	docker container run --rm -it conf-debian
