import random
import os


def kernel(minNum, maxNum):
    return random.randint(minNum, maxNum)
minNum = 1
maxNum = 45
print(f"选中了：{kernel(minNum, maxNum)}")
print("press any key to continue......")
os.system("pause > nul")
