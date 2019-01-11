

image := target/x86_64-blog_os/debug/bootimage-blog_os_diy.bin

.PHONY : run target clean
run : target
#	qemu-system-x86_64 -drive format=raw,file=$(image)
	qemu-system-x86_64 \
    -drive format=raw,file=$(image) \
    -serial mon:stdio

target :
	bootimage build --target x86_64-blog_os.json

clean : 
	cargo clean