FROM docker.io/node:19-alpine as CSSBUILDER

WORKDIR /app

ADD . .

RUN --mount=type=cache,target=/app/node_modules yarn install && NODE_ENV=production yarn build

FROM docker.io/rust:1-slim as BUILDER

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libpq-dev openssl libssl-dev libclang-dev llvm

ADD . .

RUN --mount=type=cache,target=/app/target cargo install --locked --root install --path .

RUN ldd /app/install/bin/chrismiller-xyz
RUN ldd /app/install/bin/chrismiller-xyz | grep "/" | cut -d '>' -f 2 | cut -d '(' -f 1 | while read -r line ; do cp $line /app/install/bin/; echo "$line"; done;

FROM gcr.io/distroless/cc

COPY --from=BUILDER /app/install/bin /app/
COPY --from=CSSBUILDER /app/public /public
COPY --from=CSSBUILDER /app/dist /dist


ENV LD_LIBRARY_PATH="/app/"

CMD ["/app/chrismiller-xyz"]
