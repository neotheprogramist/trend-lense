#!/bin/bash

dfx canister call trendlens_backend set_proxy_canister_id "(\"$(dfx canister id idempotency-proxy)\")"