rpc
===




Variables
----------

* `alphabet`: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz

Functions
----------

### def `b58encode(v)`

Encode a string using Base58
### def `b58encode_int(i, default_one=True)`

Encode an integer using Base58
### def `urlopen(url, data=None, timeout=<object object at 0x1002f30e0>, cafile=None, capath=None, cadefault=False, context=None)`

No documentation for this function

Classes
----------

### class `HTTPError()`

Raised when HTTP error occurs, but also acts like non-error return

Methods:

#### def `__init__(url, code, msg, hdrs, fp)`



#### def `close()`



#### def `getcode()`



#### def `geturl()`



#### def `info()`



### class `MultiCommandParser()`

No documentation for this class

Methods:

#### def `__init__()`



#### def `call_view_function()`

Call a view function on a smart contract

#### def `create_account()`

Create an account

#### def `deploy()`

Deploy a smart contract

#### def `get_beacon_block_by_hash()`

Get beacon block by hash.

#### def `get_shard_block_by_hash()`

Get shard block by hash.

#### def `schedule_function_call()`

Schedule a function call on a smart contract

#### def `send_money()`

Send money from one account to another

#### def `stake()`

Stake money for validation

#### def `swap_key()`

Swap key for an account

#### def `view_account()`

View an account

#### def `view_latest_beacon_block()`

View latest beacon block.

#### def `view_latest_shard_block()`

View latest shard block.

#### def `view_state()`

View state of the contract.

### class `NearRPC()`

No documentation for this class

Methods:

#### def `__init__(server_url, keystore_binary=None, keystore_path=None, public_key=None, debug=False)`



#### def `call_view_function(originator, contract_name, function_name, args=None)`



#### def `create_account(sender, account_alias, amount, account_public_key)`



#### def `deploy_contract(sender, contract_name, wasm_file)`



#### def `get_beacon_block_by_hash(_hash)`



#### def `get_shard_block_by_hash(_hash)`



#### def `schedule_function_call(sender, contract_name, method_name, amount, args=None)`



#### def `send_money(sender, receiver, amount)`



#### def `stake(sender, amount)`



#### def `swap_key(account, current_key, new_key)`



#### def `view_account(account_alias)`



#### def `view_latest_beacon_block()`



#### def `view_latest_shard_block()`



#### def `view_state(contract_name)`



### class `Request()`

No documentation for this class

Methods:

#### def `__init__(url, data=None, headers={}, origin_req_host=None, unverifiable=False)`



#### def `add_data(data)`



#### def `add_header(key, val)`



#### def `add_unredirected_header(key, val)`



#### def `get_data()`



#### def `get_full_url()`



#### def `get_header(header_name, default=None)`



#### def `get_host()`



#### def `get_method()`



#### def `get_origin_req_host()`



#### def `get_selector()`



#### def `get_type()`



#### def `has_data()`



#### def `has_header(header_name)`



#### def `has_proxy()`



#### def `header_items()`



#### def `is_unverifiable()`



#### def `set_proxy(host, type)`



### class `URLError()`

No documentation for this class

Methods:

#### def `__init__(reason)`



