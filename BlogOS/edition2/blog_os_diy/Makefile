

target := target/x86_64-blog_os/debug/bootimage-blog_os_diy.bin

.PHONY : run clean
run : $(target)
	qemu-system-x86_64 -drive format=raw,file=$(target)

$(target) :
	bootimage build --target x86_64-blog_os.json

clean : 
	cargo clean