class HashErrors:
    def __hash__(self):
        raise TypeError
