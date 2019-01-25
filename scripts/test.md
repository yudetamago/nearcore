Help on module rpc:

NAME
    rpc

FILE
    /Users/evgueniadegtiareva/near/nearcore/scripts/rpc.py

CLASSES
    __builtin__.object
        MultiCommandParser
        NearRPC
    
    class MultiCommandParser(__builtin__.object)
     |  Methods defined here:
     |  
     |  __init__(self)
     |  
     |  call_view_function(self)
     |      Call a view function on a smart contract
     |  
     |  create_account(self)
     |      Create an account
     |  
     |  deploy(self)
     |      Deploy a smart contract
     |  
     |  get_beacon_block_by_hash(self)
     |      Get beacon block by hash.
     |  
     |  get_shard_block_by_hash(self)
     |      Get shard block by hash.
     |  
     |  schedule_function_call(self)
     |      Schedule a function call on a smart contract
     |  
     |  send_money(self)
     |      Send money from one account to another
     |  
     |  stake(self)
     |      Stake money for validation
     |  
     |  swap_key(self)
     |      Swap key for an account
     |  
     |  view_account(self)
     |      View an account
     |  
     |  view_latest_beacon_block(self)
     |      View latest beacon block.
     |  
     |  view_latest_shard_block(self)
     |      View latest shard block.
     |  
     |  view_state(self)
     |      View state of the contract.
     |  
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |  
     |  __dict__
     |      dictionary for instance variables (if defined)
     |  
     |  __weakref__
     |      list of weak references to the object (if defined)
    
    class NearRPC(__builtin__.object)
     |  Methods defined here:
     |  
     |  __init__(self, server_url, keystore_binary=None, keystore_path=None, public_key=None, debug=False)
     |  
     |  call_view_function(self, originator, contract_name, function_name, args=None)
     |  
     |  create_account(self, sender, account_alias, amount, account_public_key)
     |  
     |  deploy_contract(self, sender, contract_name, wasm_file)
     |  
     |  get_beacon_block_by_hash(self, _hash)
     |  
     |  get_shard_block_by_hash(self, _hash)
     |  
     |  schedule_function_call(self, sender, contract_name, method_name, amount, args=None)
     |  
     |  send_money(self, sender, receiver, amount)
     |  
     |  stake(self, sender, amount)
     |  
     |  swap_key(self, account, current_key, new_key)
     |  
     |  view_account(self, account_alias)
     |  
     |  view_latest_beacon_block(self)
     |  
     |  view_latest_shard_block(self)
     |  
     |  view_state(self, contract_name)
     |  
     |  ----------------------------------------------------------------------
     |  Data descriptors defined here:
     |  
     |  __dict__
     |      dictionary for instance variables (if defined)
     |  
     |  __weakref__
     |      list of weak references to the object (if defined)

FUNCTIONS
    b58encode(v)
        Encode a string using Base58
    
    b58encode_int(i, default_one=True)
        Encode an integer using Base58

DATA
    alphabet = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz...


