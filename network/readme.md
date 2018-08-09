### Establish Session:

**1.bootstrap (Client Unconnected)**
- BOOTSTRAP Version Account Timestamp String(host) ...
- Init -> WaitGosship

**2-1. gossip (Server Unconnected)**
- GOSSIP Version Account Timestamp Int(block height) String(host) ...
- Init -> WaitingRequestBlockInfo

**2-2. reject (Server Unconnected)**
- P2P_REJECT Version Account Timestamp String(reason)
- Init -> ConnectionReject

**3. request_block_info (Client Connected)**
- REQUEST_BLOCK_INFO Version Account Timestamp Int(block length) Hash(last block hash)
- WaitGosship -> WaitBlockInfo

**4-1. block_info (Server Connected)**
- BLOCK_INFO Version Account Timestamp Int(block length) Hash(last block hash)
- WaitingRequestBlockInfo -> Idle

**4-2. reject (Server Unconnected)**
- P2P_REJECT Version Account Timestamp String(reason)
- WaitingRequestBlockInfo -> ConnectionReject

**5. (Client Connected/Failed)**
- WaitBlockInfo -> Idle/ConnectionReject

### Sync Block:

**1. request_sync_info (Client Connected)**
- REQUEST_SYNC_INFO Version Account Timestamp Int(my last block) Hash(my last block hash)
- Idle -> WaitSyncInfo

**2-1. sync_info (Server Connected)**
- SYNC_INFO Version Account Timestamp Int(fork block) Hash(fork block hash) Int(last block) Hash(last block hash)
- Idle -> WaitTransmissionRequest

**2-2. unsuccess_sync_info (Server Connected)**
- UNSECCESS_SYNC_INFO Version Account Timestamp Int(fork block) Hash(fork block hash)
- Idle -> WaitSyncInfoRequest

**3. request_transmission (Client Connected)**
- REQUEST_TRANS Version Account Timestamp Int(init block num) Int(end block num)
- WaitSyncInfo -> WaitTransmission

**4-1 transmission_prepared (Server Connected)**
- TRANS_PREPARED Version Account Timestamp Int(size)
- WaitTransmissionRequest -> WaitTransmissionAccept

**4-2 transmission_not_ready (Server Connected)**
- TRANS_NOT_READY Version Account Timestamp String(reason)
- WaitTransmissionRequest -> Idle

**5. transmission_accept (Client Transmission)**
- TRANS_ACCEPT Version Account Timestamp
- WaitTransmission -> Transmission

**6. (Server Transmission)**
- TRANS_ACCEPT Version Account Timestamp
- WaitTransmissionAccept -> Transmission

### Transmission:


### End Session: