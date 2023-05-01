package main

func average(salary []int) float64 {
  var maxi int = salary[0]
  var mini int = salary[0]
  var sumi int = 0
  for _, val := range salary {
    if val > maxi {
      maxi = val
    }
    if val < mini {
      mini = val
    }
    sumi += val
  }
  sumi -= maxi + mini
  return float64(sumi) / float64(len(salary) - 2)
}