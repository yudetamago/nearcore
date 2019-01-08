const InMemoryKeyStore = require('../test-tools/in_memory_key_store.js');
const LocalNodeConnection = require('../local_node_connection')
const NearClient = require('../nearclient');
const Account = require('../account');
const Near = require('../near');
const fs = require('fs');


const aliceAccountName = 'alice.near';
const aliceKey = {
    public_key: "FTEov54o3JFxgnrouLNo2uferbvkU7fHDJvt7ohJNpZY",
    secret_key: "N3LfWXp5ag8eKSTu9yvksvN8VriNJqJT72StfE6471N8ef4qCfXT668jkuBdchMJVcrcUysriM8vN1ShfS8bJRY"
};
const test_key_store = new InMemoryKeyStore();
test_key_store.setKey(aliceAccountName, aliceKey);
const localNodeConnection = new LocalNodeConnection();
const nearClient = new NearClient(test_key_store, localNodeConnection);
const account = new Account(nearClient);
const nearjs = new Near(nearClient);
const TEST_MAX_RETRIES = 10;


test('create account with a name that is too long should not succeed', async () => {
    const newAccountName = await generateUniqueString("create.account.test.verylongnamewhichshouldnotwork");
    const newAccountPublicKey = '9AhWenZ3JddamBoyMqnTbp7yVbRuvqAv3zwfrWgfVRJE';
    const createAccountResponse = await account.createAccount(newAccountName, newAccountPublicKey, 1, aliceAccountName);
    const expctedAccount = {
        nonce: 0,
        account_id: newAccountName,
        amount: 1,
        code_hash: 'GKot5hBsd81kMupNCXHaqbhv3huEbxAFMLnpcX2hniwn',
        stake: 0,
    };
    fail("the calls to create an invalid account should not return 200");
});

const callUntilConditionIsMet = async (functToPoll, condition, description) => {
    for (let i = 0; i < TEST_MAX_RETRIES; i++) {
        try {
            const response = await functToPoll();
            if (condition(response)) {
                console.log("Success " + description + " in " + (i + 1) + " attempts.");
                return response;
            }
        } catch (_) {
            // Errors are expected, not logging to avoid spam
        }
    }
    fail('exceeded number of retries for ' + description);
};

const waitForNonceToIncrease = async (initialAccount) => {
    callUntilConditionIsMet(
        async () => { return await account.viewAccount(initialAccount['account_id']); },
        (response) => { return response['nonce'] != initialAccount['nonce'] },
        "Call view account until nonce increases"
    );
};

// Generate some unique string with a given prefix using the alice nonce. 
const generateUniqueString = async (prefix) => {
    const viewAccountResponse = await account.viewAccount(aliceAccountName);
    return prefix + viewAccountResponse.nonce;
};
