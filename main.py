"""
RandomChoose v4.2
Written By zhangxuan, MDTeam Corp.

==========
在4.0版本中，我们使用Qt库重写了随机抽选(RandomChoose)代码
在此，我们开源了此程序。希望大家提出更好的建议!
"""

# 导入模块
from PyQt6.QtWidgets import QApplication, QPushButton, QLabel, QMessageBox
import sys
from random import choice
from PyQt6 import uic
import json


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

if not FileExist('config.json'):
    settingsnotfound()
    quit()

## 1.2:初始化
ui = uic.loadUi("./ui/main.ui")
ui.show()
# 加载QObject
onceagain: QPushButton = ui.onceagain
info: QLabel = ui.info
timeandroundtips: QLabel = ui.timeandroundtips

# Init
with open('config.json', 'r') as file:
	data = json.load(file)
minNum = data['basic']['minNum']
maxNum = data['basic']['maxNum']
chooseMode = data['basic']['chooseMode']
blacklist = data['optional']['blacklist']
nickname = data['optional']['nickname']
showMode = data['optional']['showMode']
numlist = list()
randomNum = None
times = 0
rounds = 0
timeandround = None
inftext = "欢迎使用随机抽选"

# Do timeandroind
if chooseMode == 'listDel':
    timeandround = f"你一共抽了{times}次,{rounds}轮\n注意:每{maxNum - minNum + 1}次为一轮"
else:
    timeandround = f'你一共抽了{times}次\n注:这是经典抽选方式'

# Generate NumList
for i in range(minNum, maxNum + 1):
    numlist.append(i)
for k in blacklist:
    numlist.remove(k)

#Func set
info.setText(inftext)
timeandroundtips.setText(timeandround)

# Defind Func Change:
def Change():
    # 声明变量
    global times
    global rounds
    global numlist
    global minNum
    global maxNum
    global inftext
    global timeandround
    global blacklist
    global chooseMode
    global nnindex
    global nicknameChosen
    global showMode
    randomNum = choice(numlist)
    if chooseMode == 'listDel':
    	times += 1
    	numlist.remove(randomNum)
    	timeandround = f"你一共抽了{times}次,{rounds}轮\n注意:每{maxNum}次为一轮"
    	print(f"[RandomChoose] Debug : Now numlist={numlist}, times={times},rounds={rounds}")  # Debug message
    elif chooseMode == 'classic':
        times += 1
        timeandround = f'你一共抽了{times}次\n注:这是经典抽选方式'
    else:
    	QMessageBox.critical(None, 'Error while starting choose', 'Got an exception: In config.json/optional/chooseMode,\n\nError:invaild chooseMode and only support "classic" and "listDel".\n\nAsk developers for more information.')
    if not len(nickname) == len(numlist):
        result = QMessageBox.warning(None,"警告", "检测到显示名称长度与抽选范围长度不相同\n继续运行可能导致代码报错而意外退出\n仍要运行吗??", button=QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No)
        if result == QMessageBox.StandardButton.No:
            print("User choosed to quit to reset func")
            sys.exit(0)
    nnindex = randomNum - 1
    nicknameChosen = nickname[nnindex]
    if showMode == 'nickname':
    	inftext = f'选中了:{nicknameChosen}同学'
    elif showMode == 'both':
    	inftext = f'选中了:{randomNum}号({nicknameChosen}同学)'
    elif showMode == 'number':
    	inftext = f'选中了:{randomNum}号'
    else:
    	QMessageBox.critical(None, 'Error while starting choose', 'Got an exception: In config.json/optional/showMode,\n\nError:invaild showMode and only support "number", "nickname" and "both".\n\nAsk developers for more information.')
    info.setText(inftext)
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
