# syscalls(2) - Linux manual page

- https://man7.org/linux/man-pages/man2/syscalls.2.html

# Unable to handle kernel paging request at X while intercepting the system call [duplicate]
- https://stackoverflow.com/questions/14425410/unable-to-handle-kernel-paging-request-at-x-while-intercepting-the-system-call

```c
// Source - https://stackoverflow.com/q/14425410
// Posted by PaulDaviesC, modified by community. See post 'Timeline' for change history
// Retrieved 2026-10-18, License - CC BY-SA 3.0

#include<linux/kernel.h>
#include<linux/module.h>
#include<linux/unistd.h>
#include<linux/semaphore.h>
#include<asm/cacheflush.h>
MODULE_LICENSE("GPL");
void **sys_call_table;
unsigned long addr;
asmlinkage int (*original_call)(struct pt_regs);
asmlinkage int our_call(struct pt_regs regs)
{
        printk("Intercepted sys_fork");
        return original_call(regs);
}
static int __init p_entry(void)
{
        struct page *pg;
        printk(KERN_ALERT "Module Intercept inserted");
        sys_call_table=(void *)0xc12c9e90;
        pg=virt_to_page(sys_call_table);
        addr=(unsigned long)page_address(pg);
        set_memory_rw(addr,1);
        original_call=sys_call_table[__NR_fork];
        sys_call_table[__NR_fork]=our_call;
        set_memory_ro(addr,1);
        return 0;
}
static void __exit p_exit(void)
{
        sys_call_table[__NR_fork]=original_call;
        set_memory_ro(addr,1);
        printk(KERN_ALERT "Module Intercept removed");
}
module_init(p_entry);
module_exit(p_exit);
```
