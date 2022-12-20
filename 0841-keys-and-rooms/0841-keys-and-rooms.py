class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        count = len(rooms)
        visited, stack = {0}, [0]
        while stack:
          for x in rooms[stack.pop()]:
            if x not in visited:
              stack.append(x)
              visited.add(x)
          if len(visited) == count: break
        return len(visited) == count