
FROM rust:1.82-slim
RUN apt update && apt install -y librust-openssl-dev libssl-dev curl procps
RUN mkdir /app_build
COPY . /app_build
RUN cd /app_build && cargo build --release

FROM rust:1.82-slim
RUN mkdir /app_prod_deploy
COPY --from=0 /app_build/target/release/simple-restaurant-api /app_prod_deploy/simple-restaurant-api
COPY .env /app_prod_deploy/.env
WORKDIR /app_prod_deploy
CMD ["./simple-restaurant-api"]