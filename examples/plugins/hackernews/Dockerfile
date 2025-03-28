FROM tinygo/tinygo:0.37.0 AS builder

WORKDIR /workspace
COPY go.mod .
COPY go.sum .
RUN go mod download
COPY . .

# # Add memory limit for TinyGo build
# ENV GOMEMLIMIT=8G
RUN tinygo build -target wasi -o plugin.wasm .

FROM scratch
WORKDIR /
COPY --from=builder /workspace/plugin.wasm /plugin.wasm
