FROM rust:alpine as builder

RUN rustup install nightly
RUN rustup default nightly
RUN apk -U add gzip alpine-sdk

RUN USER=root cargo new --bin compression_service
WORKDIR /compression_service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD ./ ./

RUN cat Rocket.toml
RUN rm -r ./target/release/
RUN cargo build --release

FROM alpine:latest
ARG PORT=15707
ARG APP=/usr/src/app
ARG SHARED=/mnt/shared

RUN apk -U add gzip

EXPOSE ${PORT}
VOLUME [ ${SHARED} ]

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    ROCKET_PORT=${PORT} \
    PATH=${PATH}:${APP}

RUN addgroup -S $APP_USER \
    && adduser -S $APP_USER -G $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /compression_service/target/release/compression_service ${APP}/compression_service

RUN chown -R $APP_USER:$APP_USER ${APP}
RUN chown -R $APP_USER:$APP_USER ${SHARED}

USER $APP_USER
WORKDIR ${SHARED}

CMD [ "compression_service" ]
