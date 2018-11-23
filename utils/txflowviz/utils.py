import random

def randomcolor(rnd):
    return '#' + ''.join(
        rnd.choice('0123456789abcdef') for _ in range(3)
    )

# TODO: Implement iterator
# TODO: Implement reverse iterator
class Treap:
    class Node:
        def __init__(self, key, value):
            self.key = key
            self.value = value
            self.rndkey = random.randint(0, 2**32)
            self.left = None
            self.right = None
            self.size = 1

    @staticmethod
    def merge(u, v):
        if u is None:
            return v
        if v is None:
            return u

        if u.rndkey > v.rndkey:
            u.right = Treap.merge(u.right, v)
            return Treap.update(u)
        else:
            v.left = Treap.merge(u, v.left)
            return Treap.update(v)

    @staticmethod
    def split(u, size):
        if u is None:
            return (None, None)

        if Treap.size(u.left) >= size:
            l, r = Treap.split(u.left, size)
            u.left = r
            u = Treap.update(u)
            return (l, u)
        else:
            l, r = Treap.split(u.right, size - 1 - Treap.size(u.left))
            u.right = l
            u = Treap.update(u)
            return (u, r)

    @staticmethod
    def size(u):
        return 0 if u is None else u.size

    @staticmethod
    def update(u):
        u.size = Treap.size(u.left) + Treap.size(u.right) + 1
        return u

    # TODO: Implement this functions
    def __setitem__(self, key, value):
        pass

    def __getitem__(self, index):
        pass

    def get(self, key, default=None):
        return default