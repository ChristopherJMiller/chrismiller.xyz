FROM docker.io/node:19-alpine as CSSBUILDER

WORKDIR /app

ADD . .

RUN --mount=type=cache,target=/app/node_modules yarn install && NODE_ENV=production yarn build

FROM docker.io/rust:alpine as BUILDER

WORKDIR /app

RUN apk add --update build-base

ADD . .

RUN --mount=type=cache,target=/app/target cargo install --locked --root install --path .

FROM gcr.io/distroless/cc

COPY --from=BUILDER /usr/lib /usr/lib
COPY --from=BUILDER /lib /lib
COPY --from=BUILDER /app/install /app/
COPY --from=BUILDER /app/public /app/public
COPY --from=CSSBUILDER /app/dist /app/dist

CMD ["/app/install/bin/chrismiller-xyz"]

