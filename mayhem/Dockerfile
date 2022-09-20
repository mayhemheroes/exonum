# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Dependencies
RUN apt update && apt install -y protobuf-compiler

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd exonum/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv exonum/fuzz/target/x86_64-unknown-linux-gnu/release/raw_message /raw_message && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd exonum/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv exonum/fuzz/target/release/raw_message /raw_message_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /raw_message /raw_message_no_inst /
