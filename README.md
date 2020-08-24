# Omics

## What is Omics?

Omics emerges as a solution for independent authors who do not have a clear way
of distributing comics of their own authorship. Moreover, it is important for
them to get a retribution for their works. This platform aims to create an
interactive space to bring authors closer to potential readers in the same
place. Anyone can upload and publish their work after going through a selection
process. The platform will distribute high-quality content, supporting new
authors and those who already have experience in the field. 

An author will be rewarded with monthly payments according to the impact of
their work and the interaction achieved with the readers. On the other hand,
the readers can subscribe to a plan to access premium content.  This content
represents the works whose author has a contract with the platform. It is
important to emphasize the closeness between authors and readers. 

Omics has a rating system to promote its catalogue. Any reader can rate, review
and follow his favorite comics. Following the works is an easy way to keep them
at hand.

Nobody has to invest in paper, protecting the environment, and each reader will
get personalized readings according to their preferences. A reader can access
his favorite publications by author, category and style, among other features.

The process is simple: an author uploads and publishes a comic, a content
manager approves his work. After generating enough engagement from the
readings, the author can apply for a contract to earn money. At the same time,
a reader can access all the content of the platform, except those publications
that have a contract. If a reader wants to access premium content, they must
pay a monthly subscription. The Omics income will be distributed between the
authors.

### Tools
- NodeJS
- Angular CLI
- Docker and docker-compose
- Make (commands)

### Configuration
- Para windows:
  - Descargar el installador Docker de escritorio (versión estable) de: https://www.docker.com/get-started
  - Habilitar hyperV (verificar que este habilitado container y hyperV en Caractrerística de Windows)
  - Verificar que no se inicie siempre con Windows al iniciar (reiniciar la computadora una vez instalado)
  - Para verificar que funciona
    * Ir al administrador Hyper-V
    * Ver que DockerDesktop este ejecutandose (sino doble click y ejecutarlo)
    * En caso de tener el problema de que "Hyper-V no se esta ejectuando"
      - Abrir un CMD (o Powershell) con Permisos de Administrador
      - Para comprobar que Hyper-V este funcionando ejecutar: bcdedit  (ver si hypervisorlaunchtype este en auto)
      - Para cambiarlo ejecutar lo siguiente: bcdedit /set hypervisorlaunchtype auto
      - Reiniciar la máquina

## Docker

Build backend and frontend:

```
docker-compose build
```

Run backend and frontend:

```
docker-compose up
```

- Build and run frontend image: `docker-compose up web`
- Build and run backend image: `docker-compose up server` (test in localhost:3000/api)

**Recreate backend**:

```
# Delete server container
docker-compose down

# Recreate server image and run this image as container
docker-compose up server
```

Only rebuild server and web images:

```
docker-compose build
```

## Rust

Install rustup: [rustup.rs](https://rustup.rs/). Support for GNU/Linux and Windows (64bit and 32bit: *rustup-init.exe*).

Run backend:

```
cargo run
```

Test backend:

```
cargo test
```

Build backend (for production):

```
cargo build --release
```

## Angular

See [web/README.md](./web/README.md).

Install dependencies:

```
npm install
```

Run:

```
npm start
```

## Make

Some commands have been added to a Makefile to use them easier.

Backend:

```
# Run server on PORT=3000
make run

# Build server for production
make build

# Build docker image
make docker

# Deploy
make heroku
```

Frontend:

```
# Install dependencies
make web-dependencies

# Run frontend
make web-run

# Build frontend for production
make web-build

# Deploy frontend
make web-deploy
```
