import os
import json

import time
import logging
import datetime

import random
import utils
from config import config
from heapq import heappush as push, heappop as pop

logger = logging.Logger("vizlogger")
FORMAT = "%Y-%m-%d %H:%M:%S.%f"


class DataHandler:
    def __init__(self, path):
        self.path = path
        self.ok = os.path.exists(path) and os.path.exists(os.path.join(path, 'config.json'))

        if self.ok:
            # Load configuration
            with open(os.path.join(path, 'config.json')) as fd:
                self.config = json.load(fd)

            self.uid2idx = {user['uid'] : idx for idx, user in enumerate(self.config['users'])}
            self.num_verifiers = self.config.get("num_nodes")

            self._messages = {}
            self._verifier_data = [set() for _ in range(self.num_verifiers)]

            self._shard_epochs = [0] * self.config['num_shards']

            self._last_check = time.time()
            self._wait_time = 0.1
            self._update()


    def add_message(self, message):
        m_hash = message['signed_message']['body']['hash']

        idx = self.uid2idx.get(message['uid'])
        shard_id = message['shard_id']

        self._verifier_data[idx].add(m_hash)
        self._shard_epochs[shard_id] = max(self._shard_epochs[shard_id], message['signed_message']['body']['epoch'])

        if not m_hash in self._messages and \
            message['signed_message']['body']['owner'] == message['uid']:
            # Store version of message from the original creator.
            # The only difference should be the timestamp

            self._messages[m_hash] = {
                'shard_id' : message['shard_id'],
                'timestamp' : datetime.datetime.strptime(message['timestamp'], FORMAT),
                'signed_message' : message['signed_message'],
            }


    def _update(self):
        print("Updating network.")
        updated = False

        for idx, curdata in enumerate(self._verifier_data):
            path = os.path.join(self.path, f'{idx:06}')
            newdata = os.listdir(path)

            if len(newdata) > len(curdata):
                updated = True

                for m_name in newdata:
                    m_hash, _ = os.path.splitext(m_name)

                    if m_hash in curdata:
                        continue

                    with open(os.path.join(path, m_name)) as fd:
                        message = json.load(fd)

                    self.add_message(message)


        return updated


    def get_node_info(self, uid):
        idx = self.uid2idx.get(uid)

        num_messages = 0

        return {
            "num_messages" : len(self._verifier_data[idx])
        }


    def update(self):
        if config['WATCH_REAL_TIME'] and \
            time.time() > self._last_check + self._wait_time:

            new_data = self._update()

            if new_data:
                self._wait_time = 0.1
            else:
                self._wait_time = min(2 * self._wait_time, 5.)

            self._last_check = time.time()


    def get_message(self, m_hash):
        return self._messages[m_hash]


    def get_last_messages(self, k):
        # TODO: Use treap to avoid running over every message to retrieve last `k` messages
        heap_messages = []

        for m_hash in self._messages:
            m = self.get_message(m_hash)

            push(heap_messages, (datetime.datetime.now() - m['timestamp'], m_hash))

            while len(heap_messages) > k:
                pop(heap_messages)

        messages = []

        while heap_messages:
            _, m_hash = pop(heap_messages)
            messages.append(self.get_message(m_hash))

        return messages

    def get_filtered_dag(self, shard_id=None, uid=None, epoch=None):
        self.update()

        idx = self.uid2idx.get(uid)

        # Prune graph, and leave only messages that belong to last k epochs
        k = config['DISPLAY_EPOCHS_BLOCKS']

        def is_ok(m_hash):
            message = self._messages[m_hash]

            if epoch is not None:
                if message['signed_message']['body']['epoch'] != epoch:
                    return False
            else:
                top_epoch = max(self._shard_epochs) if shard_id is None else self._shard_epochs[shard_id]
                if message['signed_message']['body']['epoch'] < top_epoch - k:
                    return False

            if shard_id is not None:
                if message['shard_id'] != shard_id:
                    return False

            if uid is not None:
                if not message['signed_message']['body']['hash'] in self._verifier_data[idx]:
                    return False

            return True

        messages = sorted(filter(is_ok, self._messages), key=lambda m_hash : datetime.datetime.now() - self._messages[m_hash]['timestamp'])

        return messages

    def get_graph_dag(self, **kwargs):
        messages = self.get_filtered_dag(**kwargs)
        valid = set(messages) # TODO: Use BST here

        epoch = kwargs.get('epoch')

        rnd = random.Random(x="color")
        colors = [utils.randomcolor(rnd) for _ in range(self.config['num_shards'])]
        xinfo = {}

        EPOCH_GAP = 100
        MESSAGE_GAP = 5 if epoch is None else 40
        VERIFIER_GAP = 10

        nodes = []
        edges = []

        self._dag_information = {
            "num_messages" : 0,
            "num_tx" : 0,
            "message_rate" : 0,
            "tx_rate" : 0,
        }

        min_time, max_time = None, None

        for m_hash in messages:
            m = self._messages[m_hash]

            if m_hash in valid:
                self._dag_information["num_messages"] += 1
                self._dag_information["num_tx"] += len(m['signed_message']['body']['transactions'])
                tm = m['timestamp']
                if min_time is None:
                    min_time = tm
                    max_time = tm
                else:
                    min_time = min(min_time, tm)
                    max_time = max(max_time, tm)

                epoch = m['signed_message']['body']['epoch']
                owner_idx = self.uid2idx.get(m['signed_message']['body']['owner'])
                last_epoch, position = xinfo.get(owner_idx, (-1, 0))


                if last_epoch == epoch:
                    position += MESSAGE_GAP
                else:
                    last_epoch = epoch
                    position = epoch * EPOCH_GAP

                xinfo[owner_idx] = (last_epoch, position)

                nodes.append({
                    'id' : m_hash,
                    'x' : position,
                    'y' : owner_idx * VERIFIER_GAP,
                    'size' : 3 if m['signed_message']['body']['isCommit'] else 1,
                    'color' : colors[m['shard_id']],
                    'label' : m_hash[:4],

                    # Extra information
                    'owner' : m['signed_message']['body']['owner'],
                    'shardid' : m['shard_id'],
                    'epoch' : epoch,
                    'timestamp' : m['timestamp'],
                    'num_parents' : len(m['signed_message']['body']['parent']),
                    'num_transactions' : len(m['signed_message']['body']['transactions']),
                    'is_commit' : m['signed_message']['body']['isCommit'],
                })

                for par in self._messages[m_hash]['signed_message']['body']['parent']:
                    if par in valid:
                        edges.append({
                            'id' : f'{m_hash}|{par}',
                            'source' : m_hash,
                            'target' : par,
                            'color' : '#ccc'
                        })

        if max_time is not None:
            delta = max_time - min_time
            sec = delta.total_seconds()

            self._dag_information['message_rate'] = self._dag_information['num_messages'] / sec
            self._dag_information['tx_rate'] = self._dag_information['num_tx'] / sec

            from pprint import pprint
            pprint(self._dag_information)

        answer = {
            'nodes' : nodes,
            'edges' : edges,
        }

        return answer
