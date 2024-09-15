import tkinter
from tkinter import messagebox as msgbox


# 创建窗口
root = tkinter.Tk()
root.title("随机抽选设置")

screenWidth = root.winfo_screenwidth()  # 获取显示区域的宽度
screenHeight = root.winfo_screenheight()  # 获取显示区域的高度
width = 500  # 设定窗口宽度
height = 200  # 设定窗口高度
left = (screenWidth - width) / 2
top = (screenHeight - height) / 2

# 宽度x高度+x偏移+y偏移
# 在设定宽度和高度的基础上指定窗口相对于屏幕左上角的偏移位置
root.geometry("%dx%d+%d+%d" % (width, height, left, top))

#设置组件
minNum = tkinter.Entry(root, width=25)
minNum.place(x=120, y=20)
minNumText = tkinter.Label(root, text='请输入最小值：', font=('微软雅黑', 12)).place(x=0, y=20)
maxNum = tkinter.Entry(root, width=25)
maxNum.place(x=120, y=60)
maxNumText = tkinter.Label(root, text='请输入最大值：', font=('微软雅黑', 12)).place(x=0, y=60)
applyopt_exception = None

#定义变量
def applyopt():
    try:
        f = open("settings.py", "w")
        minNumValue = minNum.get()
        maxNumValue = maxNum.get()
        f.write(f"class settings:\n\tminNumber = {minNumValue}\n\tmaxNumber = {maxNumValue}")
    except Exception as applyopt_exception:
        msgbox.showerror("错误", f"在试图将配置写入settings.py时发生了一些问题\n错误代码:{applyopt_exception}")
    else:
        msgbox.showinfo("提示", "写入成功!")

def quitTk():
    root.destroy()
apply = tkinter.Button(root, text="应用更改", font=('微软雅黑', 12), command=applyopt).place(x=60, y=150)
exitTk = tkinter.Button(root, text="退出", font=('微软雅黑', 12), command=quitTk).place(x=300, y=150)
root.mainloop()
