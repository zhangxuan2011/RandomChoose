from random import randint as kernel
import tkinter
import time
import sys
from tkinter import messagebox as msgbox


args = sys.argv
if len(args) == 2:
    if args[1] == "introduce":
        msgbox.showinfo("Random Choose Introduce", "欢迎使用Random Choose! \n"
                        "该程序可以随机抽选任意一个号数 \n"
                        "要改最大值和最小值，请将minNum和maxNum值改掉. \n"
                        "再次欢迎你的使用！ \n"
                        "                             版本v3.1, By zhangxuan@702/longer@706")
        quit()
root = tkinter.Tk()
root.title("随机抽选")

# 设置图标
root.iconphoto(True, tkinter.PhotoImage(file="icon.png"))

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

inftext = tkinter.StringVar()
minNum = 1
maxNum = 45
randomNum = kernel(minNum, maxNum)
inftext.set(f"欢迎使用随机抽选")
info = tkinter.Label(root, textvariable=inftext, font=("汉仪文黑-85W", 40))
info.place(x=190,y=100)
producer=tkinter.Label(root, text="制作者：\n周龙（bilibili：龙ger_longer）\n张烜（bilibili：Windows11-New）", font=("汉仪文黑-85W", 10))
producer.place(x=300, y=420)
def Change():
    randomNum = kernel(minNum, maxNum)
    inftext.set(f"选中了：{randomNum}号")
    info.place(x=250, y=100)


def quitTk():
    root.destroy()
    
onceAgain = tkinter.Button(root,text="抽一个", font=("汉仪文黑-85W",20), command=Change).place(x=175, y=350)
quitbutton = tkinter.Button(root, text="退出", font=("汉仪文黑-85W", 20), command=quitTk).place(x=525, y=350)
root.mainloop()
