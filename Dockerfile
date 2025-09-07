# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copiar archivos de dependencias
COPY Cargo.toml Cargo.lock ./

# Trick para compilar dependencias primero
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar el código fuente real
COPY src ./src

# Compilar la aplicación
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Instalar certificados CA para HTTPS
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copiar el binario
COPY --from=builder /app/target/release/health-dashboard /usr/local/bin/

# Usuario no-root
RUN useradd -m -u 1000 appuser
USER appuser

EXPOSE 3000

CMD ["health-dashboard"]