# Project 

*spin-sample-leptos-rs* is a sample app based on [leptos-rs](https://leptos.dev/) + [Fermyon Spin](https://www.fermyon.com/).

It uses [leptos_oidc](https://docs.rs/leptos_oidc/latest/leptos_oidc/) to connect with an [auth0](https://auth0.com/) OIDC Provider. 

## Configuration

Create the .env file
```
cp .env-sample .env
```

Edit the .env file:
* **SPIN_VARIABLE_AUTH0_CLIENT_ID**: OIDC client_id token
* **SPIN_VARIABLE_AUTH0_DOMAIN**: Auth0 Domain

## Setup

```bash
# install tooling
cargo install trunk just
```

## Start app locally with Spin on watch mode

To start the application
```bash
just watch
```

## Start app locally with Spin

To start the application
```bash
just up
```

## Start frontend locally with Trunk

To start the frontend
```bash
just serve
```
