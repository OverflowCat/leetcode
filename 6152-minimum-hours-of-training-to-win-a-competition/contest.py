class Solution:
    def minNumberOfHours(self, initialEnergy: int, initialExperience: int, energy, experience) -> int:
        energyToIncrease = 0
        expToIncrease = 0
        currEnergy = initialEnergy
        currExp = initialExperience
        for i, (eng, exp) in enumerate(zip(energy, experience)):
            # print("i:", i, "currEnergy:", currEnergy, "currExp:", currExp)
            if currEnergy <= eng:
                d = eng - currEnergy + 1
                # print("currEnergy:", currEnergy, "eng:", eng, "d:", d)
                # print("d:", energyToIncrease)
                energyToIncrease += d
                currEnergy += d
                # print("currEnergy:", currEnergy)
            currEnergy -= eng
            if currExp <= exp:
                d = exp - currExp + 1
                expToIncrease += d
                currExp += d
            currExp += exp
            # print(energyToIncrease, expToIncrease)
        return energyToIncrease + expToIncrease
