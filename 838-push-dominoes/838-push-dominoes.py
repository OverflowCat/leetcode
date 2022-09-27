import queue

class Solution:
    def pushDominoes(self, state: str) -> str:
        last_i = len(state) - 1
        state = [*state]
        events = defaultdict(lambda: '')
        def add_event(i, x):
            if x == 'L':
                if i > 0 and state[i - 1] == '.':
                    events[i - 1] += 'L'
            elif i < last_i and state[i + 1] == '.':
                events[i + 1] += 'R'
        for i, x in enumerate(state):
            if x != '.': add_event(i, x)
        while len(events) != 0:
            oldevents = events.copy()
            events = defaultdict(lambda: '')
            for i, e in oldevents.items():
                if len(e) == 1: 
                    if state[i] == '.': state[i] = e
                    add_event(i, e)
        return "".join(state)
