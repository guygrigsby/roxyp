# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.40.0

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo install --path .

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 3000

# Run the web service on container startup.
CMD ["roxyp"]
