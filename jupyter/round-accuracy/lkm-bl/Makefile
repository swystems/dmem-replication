kerneldir ?= /lib/modules/$(shell uname -r)/build

obj-m := round_accuracy.o
round_accuracy-objs := module.o
ccflags-y := -std=gnu11

.PHONY: all clean
all:
	make -C $(kerneldir) M=$(PWD) modules

clean:
	make -C $(kerneldir)  M=$(PWD) clean