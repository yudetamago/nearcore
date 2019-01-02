var txflow = require('./txflow.js');
var txflow_def = require('./txflow_def.js');
var utils = require('./utils.js');
const fs = require('fs');
const path = require('path');

maxUsers = 7;
maxNodes = 100;

function getRandomInt(max) {
    return Math.floor((Math.random()) * Math.floor(max)) + 1;
}

common_for_tests = fs.readFileSync(path.resolve(__dirname, 'common_rust_code_for_tests.txt'), 'utf8');

failed_rust = false;

while (true) {
    totalUsers = getRandomInt(maxUsers - 1) + 1;
    maliciousUsers = getRandomInt(totalUsers / 3 - 1);
    totalNodes = getRandomInt(maxNodes);
    console.log("totalUsers: " + totalUsers + " maliciousUsers: " + maliciousUsers + " totalNodes: " + totalNodes);
    timeStamp = Math.floor(Date.now() / 1000);
    fn_name = "nightly_generated_test.rs";
    comment = "Test generated @" + timeStamp;
    serialized = false;
    beaconMode = false;
    graph = utils.generate_random_graph(totalUsers, maliciousUsers, totalNodes, beaconMode, show_html = false);
    var ctx = new txflow_def.Context(totalUsers);
    result = utils.stress_epoch_blocks(ctx, txflow.toposort(graph), totalUsers);
    console.log(result);

    /*var s = '\n\n\n\n\n';
    for (var key in ctx.predicate_memo) {
        if (key.startsWith('invoke_')) s += key.substr(7);
    }
    console.log(s + "\n\n\n");/**/

    if (!result.includes('PASSED!')) {
        console.error("totalUsers: " + totalUsers + " maliciousUsers: " + maliciousUsers + " totalNodes: " + totalNodes);
        console.error(result);
        console.error(graph);
        console.error(JSON.stringify({'s': txflow.serialize_txflow(graph, false), 'n': totalUsers}));
        break;
    }

    // test that five rounds of communication always adds a representative
    var last_node = null;
    for (var i = 0; i < 5; ++ i) {
        for (var j = 0; j < totalUsers; ++ j) {
            last_node = txflow.create_node(j, i == 0 ? graph : [last_node]);
        }
    }

    var ctx = new txflow_def.Context(totalUsers);
    result2 = utils.stress_epoch_blocks(ctx, txflow.toposort([last_node]), totalUsers);
    console.log("AFTER:", result2);

    if (!result2.includes('PASSED!') || result2.length <= result.length) {
        console.error("totalUsers: " + totalUsers + " maliciousUsers: " + maliciousUsers + " totalNodes: " + totalNodes);
        console.error(result);
        console.error(JSON.stringify({'s': txflow.serialize_txflow([last_node], false), 'n': totalUsers}));
        break;
    }

    if (!failed_rust) {
        rust_test = txflow.gen_rust(graph, totalUsers, timeStamp, comment, serialized)

        fs.writeFileSync(path.resolve(__dirname, '../../core/txflow/src/dag/message/' + fn_name), common_for_tests + '\n' + rust_test + '}');

        const execSync = require('child_process').execSync;
        try {
            code = execSync('cargo test -p txflow dag::message::nightly_generated_test').toString();
            console.log("Rust Passed!");

        } catch (ex) {
            failed_rust = true;
            console.error("totalUsers: " + totalUsers + " maliciousUsers: " + maliciousUsers + " totalNodes: " + totalNodes);
            console.error(ex.output.toString());
        }
    }
}
