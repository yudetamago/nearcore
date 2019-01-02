var use_short_circuit = true;

function apply_txflow_definitions(ctx, node) {
    var max_epoch = compute_max_epoch(ctx, node);
    var ret = [];
    for (var def of ctx.definitions) {
        ctx.invocations = [];
        if (def.args.length != 2 || def.args[0] != 'm') {
            console.error(`Unexpected args: ${def.args}`);
        }
        if (def.args[1] == 'e') {
            for (var epoch = 0; epoch <= max_epoch + 1; ++ epoch) {
                var r = def.cond(ctx, node, epoch);
                if (typeof r === 'boolean') {
                    r = {'evidence': [], 'result': r};
                }

                if (r.result) {
                    ret.push({'ltr': def.letter, 'def': def.def_name, 'epoch': epoch, 'evidence': r.evidence})
                }
            }
        }
        else if (def.args[1] == 'm') {
            var visited = {};
            var dfs = function(n) {
                if (visited[n.uid] !== undefined) return;
                visited[n.uid] = true;

                var r = def.cond(ctx, node, n);
                if (typeof r === 'boolean') {
                    r = {'evidence': [], 'result': r};
                }

                if (r.result) {
                    ret.push({'ltr': def.letter, 'def': def.def_name, 'msg': n, 'evidence': r.evidence})
                }

                for (var parent_ of n.parents) {
                    dfs(parent_);
                }
            }
            dfs(node);
        }
        else {
            console.error(`Unexpected arg type: ${def.args[1]}`);
        }
    }

    return ret;
}

var representative_conditions = AND([
    C('L(y) = w', (ctx, m, y) => ctx.leader_of(y) == m.owner),
    C('y <= x', (ctx, m, y) => y <= ctx.message_epoch(m)),
    C('nexists m\' in D(m), y\': y\' >= y and R(m\', y\')', (ctx, m, y) => nexists_m_e(ctx, m, false, [y], (ctx, m_, y_) => (y_ >= y && ctx.R(m_, y_)))),
    OR([
        C('x = 0', (ctx, m, y) => ctx.message_epoch(m) == 0),
        C('exists r in D(m) : R(r, y-1)', (ctx, m, y) => exists_m(ctx, m, false, [y], (ctx, m_) => ctx.R(m_, y - 1))),
        C('exists k_w^y in D(m_w^x): SP(m_w^x, k)', (ctx, m, y) => exists_m(ctx, m, false, [m, y], (ctx, m_) => ctx.SP(m, m_) && ctx.message_epoch(m_) == y && m_.owner == m.owner)),
    ]),
    OR([
        C('|{m\' in D(m)| Pr(m, m\') }| == 0', (ctx, m, y) => nexists_m(ctx, m, false, [m], (ctx, m_) => ctx.Pr(m, m_))),
        C('|{m\' in D(m)| Pr(m, m\') }| == 1', (ctx, m, y) => exists_unique_m(ctx, m, false, [m], (ctx, m_) => ctx.Pr(m, m_))),
    ]),
]);

var kickout_conditions = AND([
    C('L(x) = w', (ctx, m, y) => ctx.leader_of(ctx.message_epoch(m)) == m.owner),
    C('y < x', (ctx, m, y) => y < ctx.message_epoch(m)),
    C('nexists m\' in D(m), y\': y\' >= y and R(m\', y\')', (ctx, m, y) => nexists_m_e(ctx, m, false, [y], (ctx, m_, y_) => (y_ >= y && ctx.R(m_, y_)))),
    C('nexists k\' in D(m), K(k\', y)', (ctx, m, y) => nexists_m(ctx, m, false, [y], (ctx, m_) => (ctx.K(m_, y)))),
]);

var endorsement_conditions = AND([
    C('n in D_(m)', (ctx, m, n) => approves(ctx, m, n, true)),
    C('exists e: R(n, e)', (ctx, m, n) => exists_e(ctx, n, [n], (ctx, e) => ctx.R(n, e))),
    C('nexists m\'_w in D(m_w): E_(m\'_w, n)', (ctx, m, n) => nexists_m(ctx, m, false, [m, n], (ctx, m_) => (m.owner == m_.owner && ctx.E_(m_, n)))),
    C('nexists e; p_w, k in D(m_w): K(k, e) and P(p_w, k) and R(n, e)', (ctx, m, n) => nexists_m_m_e(ctx, m, false, [m, n], (ctx, k, e) => ctx.K(k, e) && ctx.R(n, e), (ctx, p_w, k) => p_w.owner == m.owner && ctx.P(p_w, k))),
    C('nexists e;n\' in D(m_w): n\' != n and R(n, e) and R(n\', e)', (ctx, m, n) => nexists_m_e(ctx, m, false, [n], (ctx, n_, e) => n_.uid != n.uid && ctx.R(n, e) && ctx.R(n_, e))),
    C('nexists n\',m\'_w in D(m_w): E_(m\'_w, n\') and n\' notin D_(n) and n notin D_(n\')', (ctx, m, n) => nexists_m_m(ctx, m, false, [m, n], (ctx, n_) => !approves(ctx, n_, n, true) && !approves(ctx, n, n_, true), (ctx, m_) => m_.owner == m.owner, (ctx, n_, m_) => ctx.E_(m_, n_))),
]);

var finalization_conditions = AND([
    C('same owner', (ctx, m, n) => m.owner == n.owner),
    C('exists x: L(x) = w and R(n, x)', (ctx, m, n) => exists_e(ctx, n, [n], (ctx, e) => ctx.leader_of(e) == n.owner && ctx.R(n, e))),
    C('SE(m, n)', (ctx, m, n) => ctx.SE(m, n)),
    C('nexists m\'_w in D(m_w): F(m\'_w, n, x)', (ctx, m, n) => nexists_m(ctx, m, false, [n], (ctx, m_) => ctx.F_(m_, n))),
]);

var finalization_endorsement_conditions = AND([
    C('exists f in D_(m): F_(f, n)', (ctx, m, n) => exists_m(ctx, m, true, [n], (ctx, f) => ctx.F_(f, n))),
    C('nexists e;n\'_w in D(m): n\'_w != n_w and R(n, e) and R(n\'_w, e)', (ctx, m, n) => nexists_m_e(ctx, m, false, [n], (ctx, n_, e) => n.uid != n_.uid && ctx.R(n, e) && ctx.R(n_, e))),
]);

var promise_conditions = AND([
    C('k in D_(p)', (ctx, p, k) => approves(ctx, p, k, true)),
    C('exists e: K(k, e)', (ctx, p, k) => exists_e(ctx, k, [k], (ctx, e) => ctx.K(k, e))),
    C('nexists p\'_w in D(p_w): P(p\'_w, k)', (ctx, p, k) => nexists_m(ctx, p, false, [p, k], (ctx, p_) => (p.owner == p_.owner && ctx.P(p_, k)))),
    C('nexists e; m_w in D_(p_w), r in D(p_w): R(r, e) and E_(m_w, r) and K(k, e)', (ctx, p, k) => nexists_m_m_e(ctx, p, true, [p, k], (ctx, r, e) => ctx.R(r, e) && ctx.K(k, e), (ctx, m_w, r) => ctx.E_(m_w, r))),
]);

var sufficiently_approved_conditions = C(
    '|{w | exists m_w in D_(m): n in D_(m_w) }| > 2N/3',
    (ctx, m, n) => exists_supermajority_m(ctx, m, true, [n], (ctx, m_) => approves(ctx, m_, n, true))
);

var sufficiently_endorsed_conditions = C(
    '|{w | exists m_w in D_(m): E_(m_w,n) }| > 2N/3',
    (ctx, m, n) => exists_supermajority_m(ctx, m, true, [n], (ctx, m_) => ctx.E_(m_, n))
);

var sufficiently_promised_conditions = C(
    '|{w | exists p_w in D_(p): P(p_w,k) }| > 2N/3',
    (ctx, p, k) => exists_supermajority_m(ctx, p, false, [k], (ctx, p_) => ctx.P(p_, k))
);

var sufficiently_finalized_conditions = C(
    '|{w | exists m_w in D_(m): Ef(m_w,n) }| > 2N/3',
    (ctx, m, n) => exists_supermajority_m(ctx, m, true, [n], (ctx, m_) => ctx.Ef(m_, n))
);

var finalizable_conditions = AND([
    C('exists y: R(n, y)', (ctx, m, n) => exists_e(ctx, n, [n], (ctx, y) => ctx.R(n, y))),
    C('n \in D(m)', (ctx, m, n) => approves(ctx, m, n, false)),
    C(
        'nexists n\' in D(m): [n\' notin D_(n) and n notin D_(n\') and SE(m, n\'))]',
        (ctx, m, n) => nexists_m(ctx, m, false, [m, n], (ctx, n_) => !approves(ctx, n, n_, true) && !approves(ctx, n_, n, true) && (ctx.SE(m, n_)))
    ),
    C(
        'nexists n\' in D(m), y: [SP(m, n\') and K(n\', y) and R(n, y)]',
        (ctx, m, n) => nexists_m_e(ctx, m, false, [m, n], (ctx, n_, y) => (ctx.SP(m, n_) && ctx.K(n_, y) && ctx.R(n, y)))
    ),
    C(
        'nexists n\' in D(m), x: [!SE(m, n) and SA(m, n\') and R(n, x) and R(n\', x)]',
        (ctx, m, n) => nexists_m_e(ctx, m, false, [m, n], (ctx, n_, x) => n.uid != n_.uid && !ctx.SE(m, n) && ctx.SA(m, n_) && ctx.R(n, x) && ctx.R(n_, x))
    ),
]);

var predecessor_conditions = AND([
    C('F(m, n)', (ctx, m, n) => ctx.F(m, n)),
    C('nexists n_ in D(m): n in D(n_) and F(m, n_)', (ctx, m, n) => nexists_m(ctx, m, false, [m, n], (ctx, n_) => approves(ctx, n_, n, false) && ctx.F(m, n_)))
]);

var publishable_conditions = OR([
    AND([
        C('SF(m, n)', (ctx, m, n) => ctx.SF(m, n)),
        C('nexists n\' in D_(m): n in D(n\') and SF(m, n\')', (ctx, m, n) => nexists_m(ctx, m, false, [m, n], (ctx, n_) => approves(ctx, n_, n, false) && ctx.SF(m, n_))),
    ]),
    C('exists v in D(m): [Z(m, v) and Pr(v, n)]', (ctx, m, n) => exists_m(ctx, m, false, [m, n], (ctx, v) => ctx.Pr(v, n) && ctx.Z(m, v)))
]);

// publishable

function create_definitions(ctx) {
    create_definition(ctx, 'R', 'representative', ['m', 'e'], representative_conditions);
    create_definition(ctx, 'K', 'kickout', ['m', 'e'], kickout_conditions);
    create_definition(ctx, 'E_', 'endorsement', ['m', 'm'], endorsement_conditions);
    create_definition(ctx, 'F_', 'finalization', ['m', 'm'], finalization_conditions);
    create_definition(ctx, 'Ef', 'finalization_endorsement', ['m', 'm'], finalization_endorsement_conditions);
    create_definition(ctx, 'P', 'promise', ['m', 'm'], promise_conditions);
    create_definition(ctx, 'SA', 'sufficiently_approved', ['m', 'm'], sufficiently_approved_conditions);
    create_definition(ctx, 'SE', 'sufficiently_endorsed', ['m', 'm'], sufficiently_endorsed_conditions);
    create_definition(ctx, 'SP', 'sufficiently_promised', ['m', 'm'], sufficiently_promised_conditions);
    create_definition(ctx, 'SF', 'sufficiently_finalized', ['m', 'm'], sufficiently_finalized_conditions);
    create_definition(ctx, 'F', 'finalizable', ['m', 'm'], finalizable_conditions);
    create_definition(ctx, 'Pr', 'predecessor', ['m', 'm'], predecessor_conditions);
    create_definition(ctx, 'Z', 'publishable', ['m', 'm'], publishable_conditions);
}

function Context(num_participants) {
    var self = {};

    self.num_participants = num_participants;
    self.stack_depth_indent = '';
    self.invocations = []; // to detect SOs

    self.predicate_memo = {};
    self.definitions = [];

    self.leader_of = epoch => epoch % self.num_participants;
    self.set_num_participants = function(num_participants) {
        self.num_participants = num_participants;
        self.predicate_memo = {};
    }

    create_definitions(self);

    return self;
}

function create_definition(ctx, letter, name, args, cond) {
    function ret() {
        var args_str = ''; for (var i = 0; i < args.length; ++ i) args_str += ', ' + (args[i] == 'e' ? arguments[i] : 'v' + arguments[i].uid);
        var key = `${letter}(${args_str.substr(2)})`;

        if (ctx.predicate_memo['invoke_' + key] !== undefined) {
            return ctx.predicate_memo['invoke_' + key]
        }
        if (ctx.invocations.length > 1000000) {
            console.error("Too many invocations");
            console.error(ctx.invocations.join('\n'))
            return;
        }

        ctx.invocations.push(ctx.stack_depth_indent + key);
        ctx.stack_depth_indent = ctx.stack_depth_indent + ' ';
        if (arguments.length != args.length) {
            console.error(`Calling ${name} with ${arguments.length} arguments, while expected ${args.length}`);
        }
        var new_args = [ctx];
        for (var arg of arguments) { new_args.push(arg); }
        var ret = cond.apply(this, new_args);
        if (typeof ret !== 'boolean') {
            ret = ret.result;
        }
        if (typeof ret !== 'boolean') {
            console.error(`Calling ${name}, return type is ${typeof ret}, expected boolean`);
        }
        ctx.stack_depth_indent = ctx.stack_depth_indent.slice(0, -1);
        ctx.predicate_memo['invoke_' + key] = ret;
        return ret;
    }

    ret.letter = letter;
    ret.def_name = name;
    ret.args = args;
    ret.cond = cond;

    ctx[letter] = ret;
    ctx.definitions.push(ret);

    return ret;
}

function AND_OR(lst, is_and) {
    function ret() {
        var evidence = [];
        var result = is_and ? true : false;

        for (var item of lst) {
            if (((!is_and && result) || (is_and && !result)) && use_short_circuit) {
                // we don't do a short-circuit for `and`, so that we have evidence for all branches.
                // evidence for `or` is less needed, and not having a short-circuit 
                //    breaks `R` (x == 0 || R(, x-1) invokes `R` for -1)
                //
                continue;
            }

            var res = item.apply(this, arguments);
            if (typeof res === 'boolean') {
                evidence.push({'result': res, 'evidence': []});
                result = is_and ? (result && res) : (result || res);
            }
            else {
                evidence.push({'result': res.result, 'evidence': res.evidence});
                result = is_and ? (result && res.result) : (result || res.result);
            }
        }

        return {'evidence': evidence, 'result': result};
    }
    ret.kind = (is_and ? 'and' : 'or');
    ret.lst = lst;
    return ret;
}

function AND(lst) { return AND_OR(lst, true); }
function OR(lst) { return AND_OR(lst, false); }

function C(descr, func) {
    func.kind = 'c';
    func.descr = descr;
    return func;
}

function run_predicate(ctx, predicate, node) {
    var key = predicate.pred_name + "_" + node.uid;
    var memo = ctx.predicate_memo;
    if (memo[key] === undefined) {
        memo[key] = predicate(ctx, node);
    }

    return memo[key];
}

function find_all(ctx, from, inclusive, predicate) {
    var visited = {};
    var ret = [];

    var key = "global_" + predicate.pred_name + "_" + inclusive + "_" + from.uid;
    var memo = ctx.predicate_memo;
    if (memo[key] !== undefined) {
        return memo[key];
    }

    function dfs(node, is_root) {
        if (visited[node.uid] !== undefined) {
            return;
        }
        visited[node.uid] = true;

        if ((!is_root || inclusive) && run_predicate(ctx, predicate, node)) {
            ret.push(node);
        }

        for (var idx = 0; idx < node.parents.length; ++ idx) {
            dfs(node.parents[idx], false);
        }
    }

    if (Object.prototype.toString.call(from) !== '[object Array]') {
        from = [from];
    }

    for (var node of from) {
        dfs(node, true);
    }

    memo[key] = ret;
    return ret;
}

function approves(ctx, a, b, allow_self_approve) {
    var predicate = (ctx, n) => n.uid == b.uid;
    predicate.pred_name = 'approves_' + b.uid;

    var matching = find_all(ctx, a, allow_self_approve, predicate);
    return matching.length > 0;
}

function unique_closure_name(predicate, captured) {
    var ret = "{" + predicate.toString() + "} ";
    for (var v of captured) {
        if (typeof v.uid !== "undefined") {
            ret += "_" + v.uid;
        }
        else {
            if (typeof v !== "number") {
                console.error(`Type of ${v} inside unique_fidn_all_name is ${typeof v}, expected number. Ret: ${ret}`);
            }
            ret += "_" + v;
        }
    }
    return ret;
}

function compute_max_epoch(ctx, msg, cache) {
    if (cache === undefined) {
        cache = {};
    }

    if (cache[msg.uid] !== undefined) {
        return cache[msg.uid];
    }

    var ret = ctx.message_epoch(msg);
    for (var parent_ of msg.parents) {
        var cand = compute_max_epoch(ctx, parent_, cache);
        if (cand > ret) ret = cand;
    }
    cache[msg.uid] = ret;
    return ret;
}

function exists_e(ctx, m, captured, predicate) {
    var max_epoch = compute_max_epoch(ctx, m);
    for (var epoch = 0; epoch <= max_epoch + 1; ++ epoch) {
        if (predicate(ctx, epoch)) {
            return true;
        }
    }

    return false;
}

function nexists_m_m(ctx, m, allow_self, captured, p1, p2, pc) {
    // p1 filters out first messages
    // p2 filters out second messages
    // pc is applied to two messages

    p1.pred_name = "m1_" + unique_closure_name(p1, captured);
    p2.pred_name = "m2_" + unique_closure_name(p2, captured);
    
    var m1 = find_all(ctx, m, allow_self, p1);
    var m2 = find_all(ctx, m, allow_self, p2);

    // TODO : cache
    for (var a1 of m1) {
        for (var a2 of m2) {
            if (pc(ctx, a1, a2)) {
                return {'evidence': [a1, a2], 'result': false};
            }
        }
    }

    return {'evidence': [], 'result': true};
}

function nexists_m_m_e(ctx, m, allow_self, captured, pme, pmm) {
    // if the arguments are (m, n, e)
    //   pme filters out on (n, e)
    //   pmm filters out on (m, n)
    // it works differently compared to nexists_m_m, since in both appearances
    //   the first filter will return very few messages

    var max_epoch = compute_max_epoch(ctx, m);
    combined_pme = function(ctx, msg) {
        for (var epoch = 0; epoch <= max_epoch + 1; ++ epoch) {
            if (pme(ctx, msg, epoch)) {
                return true;
            }
        }
        return false;
    }

    combined_pme.pred_name = "combined_pme_" + unique_closure_name(pme, captured);

    var me_filtered = find_all(ctx, m, allow_self, combined_pme);

    combined_pmm = function(ctx, msg) {
        for (var msg2 of me_filtered) {
            if (pmm(ctx, msg, msg2)) {
                return true;
            }
        }
        return false;
    }

    combined_pmm.pred_name = "combined_pmm_" + unique_closure_name(pmm, captured);

    var matched = find_all(ctx, m, allow_self, combined_pmm);

    return {'evidence': matched, 'result': matched.length == 0};
}

function nexists_m_e(ctx, m, allow_self, captured, predicate) {
    var max_epoch = compute_max_epoch(ctx, m);
    combined_predicate = function(ctx, msg) {
        for (var epoch = 0; epoch <= max_epoch + 1; ++ epoch) {
            if (predicate(ctx, msg, epoch)) {
                return true;
            }
        }
        return false;
    }

    combined_predicate.pred_name = "combined_" + unique_closure_name(predicate, captured);

    var matching = find_all(ctx, m, allow_self, combined_predicate);
    return {'evidence': matching, 'result': matching.length == 0}
}

function exists_m(ctx, m, allow_self, captured, predicate) {
    predicate.pred_name = unique_closure_name(predicate, captured);

    var matching = find_all(ctx, m, allow_self, predicate);
    return {'evidence': matching, 'result': matching.length > 0}
}

function exists_supermajority_m(ctx, m, allow_self, captured, predicate) {
    predicate.pred_name = unique_closure_name(predicate, captured);

    var matching = find_all(ctx, m, allow_self, predicate);
    var witnesses = {};
    for (var msg of matching) {
        witnesses[msg.owner] = true;
    }

    return {'evidence': matching, 'result': Object.keys(witnesses).length > ctx.num_participants * 2 / 3};
}

function exists_unique_m(ctx, m, allow_self, captured, predicate) {
    predicate.pred_name = unique_closure_name(predicate, captured);

    var matching = find_all(ctx, m, allow_self, predicate);
    return {'evidence': matching, 'result': matching.length == 1}
}

function nexists_m(ctx, m, allow_self, captured, predicate) {
    predicate.pred_name = unique_closure_name(predicate, captured);

    var matching = find_all(ctx, m, allow_self, predicate);
    return {'evidence': matching, 'result': matching.length == 0}
}

if (typeof module !== 'undefined') {
    module.exports.Context = Context;
    module.exports.apply_txflow_definitions = apply_txflow_definitions;
}
