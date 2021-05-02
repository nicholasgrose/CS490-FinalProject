FROM rust:alpine as builder

RUN apk -U add openssl-dev alpine-sdk

RUN USER=root cargo new --bin file_service
WORKDIR /file_service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build
RUN rm src/*.rs

ADD ./ ./

RUN cat Rocket.toml
RUN rm -r ./target/release/
RUN cargo build


FROM alpine:latest
ARG PORT=31415
ARG APP=/usr/src/app
ARG SHARED=/mnt/shared

RUN apk add -U --no-cache openssl ca-certificates gzip \
    && update-ca-certificates

EXPOSE ${PORT}
VOLUME [ ${SHARED} ]

ENV TZ=Etc/UTC \
    # APP_USER=appuser \
    ROCKET_PORT=${PORT} \
    PATH=${PATH}:${APP} \
    RUST_BACKTRACE=1

# RUN addgroup -S $APP_USER \
    # && adduser -S $APP_USER -G $APP_USER \
RUN mkdir -p ${APP} \
    && mkdir -p ${SHARED}

COPY --from=builder /file_service/target/debug/file_service ${APP}/file_service

# RUN chown -R $APP_USER:$APP_USER ${SHARED} \
#     && chmod -R 777 ${SHARED}

# USER $APP_USER
WORKDIR ${SHARED}

CMD [ "file_service" ]
