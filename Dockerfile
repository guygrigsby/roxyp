FROM rust:1.31

WORKDIR /usr/src/roxyp
COPY . .

RUN cargo install --path .

CMD ["roxyp"]
