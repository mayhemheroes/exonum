# Build Stage
FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

## Dependencies
RUN apt update && apt install -y protobuf-compiler libsodium-dev clang libclang-dev

## Add source code to the build stage.
ADD . /src
WORKDIR /src

ENV ROCKSDB_CXX_STD="c++17"
ENV CXXFLAGS="-include cstdint"

RUN echo building instrumented harnesses && \
    bash -c "pushd exonum/fuzz && cargo +nightly fuzz build && popd" && \
    mv exonum/fuzz/target/x86_64-unknown-linux-gnu/release/raw_message /raw_message && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd exonum/fuzz && cargo +nightly build --release && popd" && \
    mv exonum/fuzz/target/release/raw_message /raw_message_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /raw_message /raw_message_no_inst /
