class Solution:
    def minNumberOfHours(self, initialEnergy: int, initialExperience: int, energy, experience) -> int:
        energyToIncrease = 0
        expToIncrease = 0
        currEnergy = initialEnergy
        currExp = initialExperience
        for i, (eng, exp) in enumerate(zip(energy, experience)):
            print(i, eng, exp, currEnergy, currExp)
            if currEnergy < eng:
                energyToIncrease += eng - currEnergy
                currEnergy = eng
            currEnergy -= eng
            if currExp < exp:
                expToIncrease += exp - currExp
                currExp = exp
            currExp += exp
            print(energyToIncrease, expToIncrease)
        return energyToIncrease + expToIncrease
