# for this to work on local machine you need to replace call to local ones
# same with ids

MYID=$(dfx identity get-principal)
dfx canister call --ic idempotency-proxy admin_set_managers "(vec {principal \"$MYID\"})"
# add dfx identity to the list of callers
dfx canister call --ic idempotency-proxy admin_add_caller "(principal \"$MYID\")"
# add trendlense-backed to the list of callers
dfx canister call --ic idempotency-proxy admin_add_caller "(principal \"ajbii-niaaa-aaaal-ajpaa-cai\")"
dfx canister call --ic idempotency-proxy admin_set_agents '
  (vec {
    record {
      name = "Trendlenselocal";
      endpoint = "https://idempotent-proxy-cf-worker.vypeers.workers.dev";
      max_cycles = 100000000000;
      proxy_token = null;
    };
  })
'