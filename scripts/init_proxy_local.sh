#!/bin/bash

MYID=$(dfx identity get-principal)
BACKEND=$(dfx canister id trendlens_backend)

dfx canister call idempotency-proxy admin_set_managers "(vec {principal \"$MYID\"})"
# add dfx identity to the list of callers
dfx canister call idempotency-proxy admin_add_caller "(principal \"$MYID\")"
# add trendlense-backed to the list of callers
dfx canister call idempotency-proxy admin_add_caller "(principal \"$BACKEND\")"
dfx canister call idempotency-proxy admin_set_agents '
  (vec {
    record {
      name = "Trendlenselocal";
      endpoint = "https://idempotent-proxy-cf-worker.vypeers.workers.dev";
      max_cycles = 100000000000;
      proxy_token = null;
    };
  })
'