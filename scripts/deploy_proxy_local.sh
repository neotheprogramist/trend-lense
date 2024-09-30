dfx deploy idempotency_proxy --argument "(opt variant {
    Init = record { 
        ecdsa_key_name = \"dfx_key_new\"; 
        proxy_token_refresh_interval = 3600; 
        subnet_size = 1; 
        service_fee = 10_000_000; 
    }
})"