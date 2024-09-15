from random import choice as kernel
import tkinter
import time
import sys
from tkinter import messagebox as msgbox
import settings


def FileExist(filename):
    try:
        f = open(filename, "r")
    except:
        return False
    else:
        f.close()
        return True

# 导入设置
if FileExist('settings.py'):
    import settings
else:
    msgbox.showerror("错误", '无法找到配置文件(settings.py) \n 请前往"随机抽选设置"进行配置。')
    quit()
# 介绍(传人introduce参数)
args = sys.argv
if len(args) == 2:
    if args[1] == "introduce":
        msgbox.showinfo("Random Choose Introduce", "欢迎使用Random Choose! \n"
                        "该程序可以随机抽选任意一个号数 \n"
                        "要改最大值和最小值，请将minNum和maxNum值改掉. \n"
                        "再次欢迎你的使用！ \n"
                        "                             版本v3.3, By zhangxuan@702/longer@706")
        quit()

# 创建窗口
root = tkinter.Tk()
root.title("随机抽选")

# 设置图标
#root.iconphoto(True, tkinter.PhotoImage(file="icon.png"))

screenWidth = root.winfo_screenwidth()  # 获取显示区域的宽度
screenHeight = root.winfo_screenheight()  # 获取显示区域的高度
width = 850  # 设定窗口宽度
height = 500  # 设定窗口高度
left = (screenWidth - width) / 2
top = (screenHeight - height) / 2

# 宽度x高度+x偏移+y偏移
# 在设定宽度和高度的基础上指定窗口相对于屏幕左上角的偏移位置
root.geometry("%dx%d+%d+%d" % (width, height, left, top))

# 背景设置
#background = tkinter.Label(image=tkinter.PhotoImage(file="IMAGEFILE")).place(x=0, y=0)

# 初始化
inftext = tkinter.StringVar()
timeandround = tkinter.StringVar()
minNum = settings.settings.minNumber
maxNum = settings.settings.maxNumber
numlist = list()
randomNum = None
times = 0
rounds = 0

#Setup
#Generate a number list
for i in range(minNum,maxNum + 1):
	numlist.append(i)
print(f'[RandomChoose] Debug : numlist={numlist}, times={times},rounds={rounds}')
inftext.set(f"欢迎使用随机抽选")
timeandround.set(f"你一共抽了{times}次,{rounds}轮\n注意:每{maxNum - minNum + 1}次为一轮")
info = tkinter.Label(root, textvariable=inftext, font=("微软雅黑", 40))
info.place(x=190,y=100)
info2=tkinter.Label(root, textvariable=timeandround, font=("汉仪文黑-85W", 15))
info2.place(x=320, y=420)

# 定义函数
def Change():
    # 声明变量
    global times
    global rounds
    global numlist
    global minNum
    global maxNum
    randomNum = kernel(numlist)
    inftext.set(f"选中了：{randomNum}号")
    info.place(x=230,y=100)
    numlist.remove(randomNum)
    print(f"[RandomChoose] Debug : Now numlist={numlist}, times={times},rounds={rounds}") #Debug message
    times += 1
    timeandround.set(f"你一共抽了{times}次,{rounds}轮\n注意:每{maxNum}次为一轮")
    if len(numlist) == 0:
    	rounds += 1
    	#Re-Generate numlist
    	for j in range(minNum,maxNum + 1):
    		numlist.append(j)

def quitTk():
    root.destroy()

onceAgain = tkinter.Button(root,text="抽一个", font=("微软雅黑",20), command=Change).place(x=175, y=350)
quitbutton = tkinter.Button(root, text="退出", font=("微软雅黑", 20), command=quitTk).place(x=525, y=350)
root.mainloop()
