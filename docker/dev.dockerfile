FROM nim65s/cargo-binstall AS installer

# Whether prebuilt binaries should be fetched
# over building dependencies from source.
ARG NO_BINSTALL="0"

# Download all the dependencies from cargo.
RUN \
	if [ "${NO_BINSTALL}" = "0" ]; then \
		cargo binstall -y --locked \
			trunk \
			cargo-watch \
			just; \
	elif [ "${NO_BINSTALL}" = "1" ]; then \
		cargo install -q --locked \
			trunk \
			cargo-watch \
			just; \
	fi

# Download atlasgo separately for migrations.
RUN curl -sSfL https://atlasgo.sh | sh




FROM rust:1.89-bookworm

ENV DEBIAN_FRONTEND=nointeractive

# Setup the dev user.
RUN useradd -m dev \
	&& chown -R dev:dev /home/dev \
	&& chmod -R u+rwx /home/dev
WORKDIR /home/dev/reddyt
USER dev

# Add cargo bin to the path.
ENV PATH="/home/dev/.cargo/bin:$PATH"
ENV PATH="/usr/local/bin/:$PATH"

# Copy all the dependencies from the installer step.
COPY --from=installer /usr/local/cargo/bin/cargo-watch /home/dev/.cargo/bin/cargo-watch
COPY --from=installer /usr/local/cargo/bin/trunk /home/dev/.cargo/bin/trunk
COPY --from=installer /usr/local/cargo/bin/just /home/dev/.cargo/bin/just
COPY --from=installer /usr/local/bin/atlas /usr/local/bin/atlas

# Install the WASM target for rust.
RUN rustup target add wasm32-unknown-unknown

# Run application.
EXPOSE 8080
CMD ["just", "dev"]
