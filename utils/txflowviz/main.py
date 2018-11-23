from flask import Flask, jsonify, render_template
from config import config
from core import DataHandler

import sys

app = Flask(__name__)
handler = DataHandler(config['TXFLOW_LOGS'])


@app.route('/api/node/uid/<int:uid>')
@app.route('/api/shard/id/<int:shard_id>')
@app.route('/api/shard/id/<int:shard_id>/node/uid/<int:uid>')
@app.route('/api/shard/id/<int:shard_id>/epoch/<int:epoch>')
def get_network(shard_id=None, uid=None, epoch=None):
    return jsonify(handler.get_graph_dag(shard_id=shard_id, uid=uid, epoch=epoch))


@app.route('/node/uid/<int:uid>')
@app.route('/shard/id/<int:shard_id>')
@app.route('/shard/id/<int:shard_id>/node/uid/<int:uid>')
def view_network(shard_id=None, uid=None):
    url = '/api' + \
                    ('' if shard_id is None else f'/shard/id/{shard_id}') + \
                    ('' if uid is None else f'/node/uid/{uid}')

    title = ''
    if shard_id is not None:
        title = f'Shard {shard_id}'

    if uid is not None:
        if title:
            title += f" (Node {uid})"
        else:
            title = f"Node {uid}"

    context = dict(
        title = title,
        uid=uid,
        shard_id=shard_id,
        api_url = url,
    )

    return render_template('node.html', **context)


@app.route('/shard/id/<int:shard_id>/epoch/<int:epoch>')
def view_epoch(shard_id, epoch):
    url = f'/api/shard/id/{shard_id}/epoch/{epoch}'
    return render_template("epoch.html", api_url=url)


@app.route('/message/hash/<string:mhash>')
def message(mhash):
    message = handler.get_message(mhash)
    return render_template("message.html", message=message)


@app.route('/message/dashboard')
def message_dashboard():
    info = handler.get_last_messages(100)
    return render_template("dashboard.html", messages=info)


@app.route('/')
def index():
    context = dict(
        ok=handler.ok,
        txflow=handler.config,
        shards=list(range(handler.config['num_shards'])),
        **config,
    )

    # Display only first 1000 nodes
    context['txflow']['users'] = context['txflow']['users'][:1000]

    return render_template('index.html', **context)


if __name__ == '__main__':
    debug = 'debug' in sys.argv

    app.run(debug=debug,
            host="localhost",
            port=5000)
