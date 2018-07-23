## API

### Block Level:
**1. fn current_block_len() -> u64**

usage:

Return current block length. 

---------------------

### Account Level:
**1. fn account_balance(addr: u256) -> u64**

usage:

Return balance according to the address. 

---------------------

### Storage Level:
**1. fn save(key: u256, value: u256)**

usage:

Save a value to the current account's storage 

**2. fn load(key: u256) -> u256**

usage:

Load a value from the current account's storage 

**3. fn delete(key: u256) -> u256**

usage:

Delete a value from the current account's storage 

**4. fn update(key: u256, value: u256) -> bool**

usage:

Update a value to the current account's storage, if the key doen't exist. return false.

**5. fn capacity() -> u64**

usage:

Current capacity of the storage.


---------------------

### Call:
**1. fn call(addr: u256, input_balnace: u64, selector: ptr) -> u256**

usage:

Call another contract. 

---------------------

### Transaction:
**1. fn transfer(to_addr: u256, balance: u64) -> bool**

usage:

Send balance to another account. If have no enough balance, return false.

**2. fn send_action(addr: u256, input_balnace: u64, selector: ptr) -> bool**

usage:

Send an action to another contract. If have no enough balance, return false.

**3. fn transaction_result(addr: u256) -> u256**

usage:

Get the result of previous executed transaction.

---------------------

### Utility:
**1. fn timestamp() -> u64**

usage:

Return current *nix timestamp.
