# How to Run

## Prerequisites

Ensure you have [`dfx`](https://internetcomputer.org/docs/current/developer-docs/setup/install/) installed. If not, you can install it using the following command:

```sh
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

## Deployment and Setup (Local)

To deploy and set up the project locally, run:

```sh
./scripts/deploy_all_local.sh <identity>
```

> **Note:** You can use the default identity, as it doesn't matter for local deployment.

## Run local preview

To run the start script:

```sh
yarn start
```

This should expose the frontend on 

`localhost:3000`