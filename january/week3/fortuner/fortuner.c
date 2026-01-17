#include <linux/module.h>  /* We are making a Linux kernel module */
#include <linux/init.h>    /* Needed for init macros */
#include <linux/printk.h>  /* Needed for printk() macros */
#include <linux/proc_fs.h> /* For creating file under /proc */
#include <linux/random.h>  /* For generating a random fortune */

#include "fortunes.h"

#define procfs_name "fortuner"

static struct proc_dir_entry *our_proc_file;

/* State maintained per file descriptor.
 * fortune_idx: Random fortune index selected at open time.
 * Storing state in file->private_data ensures consistent fortune
 * across multiple read() calls for the same file descriptor.
 */
struct fortuner_state
{
    int fortune_idx;
};

/* Open the proc file. Generates a random index into the fortunes array
 * and stores in file private data */
static int
fortuner_open(struct inode *inode, struct file *file)
{
    struct fortuner_state *state;

    state = kmalloc(sizeof(*state), GFP_KERNEL);
    if (!state)
        return -ENOMEM;

    state->fortune_idx = get_random_u32_below(ARRAY_SIZE(fortunes));

    file->private_data = state;
    return 0;
}

/* Read the fortune at fortune_idx in fortunes array */
static ssize_t fortuner_read(struct file *file, char __user *buffer,
                             size_t buffer_length, loff_t *offset)
{

    struct fortuner_state *state = file->private_data;

    const char *fortune = fortunes[state->fortune_idx];
    size_t len = strlen(fortune);
    ssize_t retval;

    /* Log once per logical read */
    if (*offset == 0)
    {
        pr_info("fortuner: procfile read (fortune %u)\n",
                state->fortune_idx);
    }

    retval = simple_read_from_buffer(buffer, buffer_length, offset, fortune, len);

    if (retval < 0)
    {
        pr_warn("fortuner: read failed (%zd)\n", retval);
    }

    return retval;
}

static int fortuner_release(struct inode *inode, struct file *file)
{
    kfree(file->private_data);
    return 0;
}

/* Struct that stores function pointers to relevant module functions */
static const struct proc_ops proc_file_fops = {
    .proc_open = fortuner_open,
    .proc_read = fortuner_read,
    .proc_release = fortuner_release,
};

/* Module Initialization. Create the file /proc/fortuner*/
static int __init
fortuner_fs_init(void)
{
    our_proc_file = proc_create(procfs_name, 0644, NULL, &proc_file_fops);
    if (NULL == our_proc_file)
    {
        pr_alert("Error:Could not initialize /proc/%s\n", procfs_name);
        return -ENOMEM;
    }

    pr_info("/proc/%s created\n", procfs_name);
    return 0;
}

/* Module Cleanup. Remove the file /proc/fortuner*/
static void __exit fortuner_fs_exit(void)
{
    proc_remove(our_proc_file);
    pr_info("/proc/%s removed\n", procfs_name);
}

module_init(fortuner_fs_init);
module_exit(fortuner_fs_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("build52");
MODULE_DESCRIPTION("A fortune telling kernel module");