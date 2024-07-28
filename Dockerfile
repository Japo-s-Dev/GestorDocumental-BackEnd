# Utiliza la imagen oficial de Ubuntu 22.04 como base
FROM ubuntu:22.04

# Instala curl y build-essential (para compilación)
RUN apt-get update && apt-get install -y curl build-essential

# Instala Rust utilizando rustup, el gestor de instalación de Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Configura el entorno para que Rust esté disponible
ENV PATH="/root/.cargo/bin:${PATH}"

# Establece el directorio de trabajo
WORKDIR /src/server

# Copia el código fuente
#COPY ../server/ .

RUN cargo install cargo-watch

# Mantén el contenedor en ejecución
#CMD ["cargo", "watch", "-q","-c","-w","src/","-w",".cargo/","-x","run"]
CMD ["cargo", "run", "-q"]
