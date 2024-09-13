"""
RandomChoose v4.0
Written By zhangxuan, MDTeam Corp.

==========
在4.0版本中，我们使用Qt库重写了随机抽选(RandomChoose)代码
在此，我们开源了此程序。希望大家提出更好的建议!
"""

# 导入模块
from PyQt6.QtWidgets import QApplication, QPushButton, QLabel, QMessageBox
import sys
from random import choice as kernel
from PyQt6 import uic


# 1.正片开始!
app = QApplication(sys.argv)

## 1.1:定义函数
def FileExist(filename):
    try:
        f = open(filename, "r")
    except:
        return False
    else:
        f.close()
        return True


def settingsnotfound():
    QMessageBox.critical(None, "错误", "找不到RandomChoose的配置文件\n请前往\"随机抽选设置\"进行设置", buttons=QMessageBox.StandardButton.Ok)

if FileExist('settings.py'):
    import settings
else:
    settingsnotfound()
    quit()

## 1.2:初始化
ui = uic.loadUi("./ui/main.ui")
ui.show()
# 加载QObject
onceagain: QPushButton = ui.onceagain
info: QLabel = ui.info
timeandroundtips: QLabel = ui.timeandroundtips

minNum = settings.settings.minNumber
maxNum = settings.settings.maxNumber
numlist = list()
randomNum = None
times = 0
rounds = 0
timeandround = f"你一共抽了{times}次,{rounds}轮\n注意:每{maxNum - minNum + 1}次为一轮"
inftext = "欢迎使用随机抽选"

# Generate NumList
for i in range(minNum, maxNum + 1):
    numlist.append(i)

#Func set
info.setText(inftext)
timeandroundtips.setText(timeandround)

# Defind Change:
def Change():
    # 声明变量
    global times
    global rounds
    global numlist
    global minNum
    global maxNum
    global inftext
    global timeandround
    randomNum = kernel(numlist)
    inftext = f'选中了:{randomNum}号'
    info.setText(inftext)
    numlist.remove(randomNum)
    print(f"[RandomChoose] Debug : Now numlist={numlist}, times={times},rounds={rounds}")  # Debug message
    times += 1
    timeandround = f"你一共抽了{times}次,{rounds}轮\n注意:每{maxNum}次为一轮"
    timeandroundtips.setText(timeandround)
    if len(numlist) == 0:
        rounds += 1
        # Re-Generate numlist
        for j in range(minNum, maxNum + 1):
            numlist.append(j)


# 定义OnceAgain按钮被点击后的发生事件
onceagain.clicked.connect(Change)

# 进入事件循环
sys.exit(app.exec())