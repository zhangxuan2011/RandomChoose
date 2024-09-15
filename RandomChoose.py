from random import randint as kernel
import tkinter


root = tkinter.Tk()
root.title("Random Choose Main Window")
root.geometry("850x500")
minNum = 1
maxNum = 45
info = tkinter.Label(root, text=f"选中了：{kernel(minNum, maxNum)}号", font=("微软雅黑", 40))
info.place(x=200, y=200)
root.mainloop()
