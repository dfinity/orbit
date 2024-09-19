PRINCIPAL=$1
dfx canister call control_panel update_waiting_list 'record { users = vec { principal "'$PRINCIPAL'" }; new_status = variant {Approved} }'
