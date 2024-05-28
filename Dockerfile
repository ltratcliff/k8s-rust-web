FROM --platform=linux/amd64 rust AS build
WORKDIR /usr/src/myapp
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM --platform=linux/amd64 alpine
COPY --from=build /usr/local/cargo/bin/web_env_k8s /usr/local/bin/myapp
EXPOSE 3000
CMD ["myapp"]