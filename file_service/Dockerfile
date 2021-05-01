FROM rust:alpine as builder

RUN rustup install nightly
RUN rustup default nightly
RUN apk -U add openssl-dev alpine-sdk

RUN USER=root cargo new --bin file_service
WORKDIR /file_service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD ./ ./

RUN cat Rocket.toml
RUN rm -r ./target/release/
RUN cargo build --release


FROM alpine:latest
ARG PORT=31415
ARG APP=/usr/src/app
ARG SHARED=/mnt/shared

RUN apk -U add openssl

EXPOSE ${PORT}
VOLUME [ ${SHARED} ]

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    ROCKET_PORT=${PORT} \
    PATH=${PATH}:${APP}

RUN addgroup -S $APP_USER \
    && adduser -S $APP_USER -G $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /file_service/target/release/file_service ${APP}/file_service

RUN chown -R $APP_USER:$APP_USER ${APP}
RUN chown -R $APP_USER:$APP_USER ${SHARED}

USER $APP_USER
WORKDIR ${SHARED}

CMD [ "file_service" ]
