FROM rust:latest as builder

RUN rustup install nightly
RUN rustup default nightly
RUN apt install gcc

RUN USER=root cargo new --bin s3_service
WORKDIR /s3_service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN cat Rocket.toml
RUN rm -r ./target/release/
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 31415

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /s3_service/target/release/s3_service ${APP}/s3_service
COPY --from=builder /s3_service/Rocket.toml ${APP}/Rocket.toml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./s3_service"]
