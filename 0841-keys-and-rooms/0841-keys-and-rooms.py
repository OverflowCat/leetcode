class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        visited, stack = {0}, [0]
        while stack:
          for x in rooms[stack.pop()]:
            if x not in visited:
              stack.append(x)
              visited.add(x)
        return len(rooms) == len(visited)