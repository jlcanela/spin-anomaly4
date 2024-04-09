# Project 

*spin-sample-leptos-rs* is a sample app based on [leptos-rs](https://leptos.dev/) + [Fermyon Spin](https://www.fermyon.com/).

It uses [leptos_oidc](https://docs.rs/leptos_oidc/latest/leptos_oidc/) to connect with an [auth0](https://auth0.com/) OIDC Provider. 

## Setup

```bash
# install tooling
cargo install trunk just
```

## Configuration for local deployment

Create the .env file
```
cp .env-sample .env
```

Edit the .env file:
* **SPIN_VARIABLE_AUTH0_CLIENT_ID**: OIDC client_id token
* **SPIN_VARIABLE_AUTH0_DOMAIN**: Auth0 Domain

## Configuration for github action

*Deploy* github action automates deployment of anomaly4 to spin/fermyon hosting. 

Setup the environment secrets:
* **FERMYON_CLOUD_TOKEN**: [Fermyon Cloud Token](https://developer.fermyon.com/cloud/github-actions#save-the-personal-access-token-as-a-repository-secret) used to authenticate before spin deployment
* **AUTH0_CLIENT_ID**: value of the auth0_client_id parameter
* **AUTH0_DOMAIN**: value of the auth0_domain parameter

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
