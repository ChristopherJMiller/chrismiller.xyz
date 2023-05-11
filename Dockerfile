FROM docker.io/node:19-alpine as CSSBUILDER

WORKDIR /app

ADD . .

RUN --mount=type=cache,target=/app/node_modules yarn install && NODE_ENV=production yarn build

FROM docker.io/rust:alpine as BUILDER

WORKDIR /app

RUN apk add --update build-base postgresql-dev

ADD . .

RUN --mount=type=cache,target=/app/target cargo install --locked --root install --path .

RUN ldd /app/install/bin/chrismiller-xyz

FROM gcr.io/distroless/cc

COPY --from=BUILDER /app/install /app/
COPY --from=BUILDER /usr/lib/libpq.so.5 /app/
COPY --from=BUILDER /lib/libssl.so.3 /app/
COPY --from=BUILDER /lib/ld-musl-x86_64.so.1 /app/
COPY --from=BUILDER /lib/libcrypto.so.3 /app/
COPY --from=BUILDER /lib/ld-musl-x86_64.so.1  => /app/libc.musl-x86_64.so.1 

COPY --from=BUILDER /app/public /app/public
COPY --from=CSSBUILDER /app/dist /app/dist

CMD ["/app/install/bin/chrismiller-xyz"]

