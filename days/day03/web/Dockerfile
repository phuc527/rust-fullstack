FROM rust:1 AS build
RUN rustup target add wasm32-unknown-unknown \
 && curl -sL https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin
WORKDIR /app
COPY . .
RUN trunk build --release

FROM nginx:alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
