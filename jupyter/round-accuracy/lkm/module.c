#include <linux/module.h>
#include <linux/ktime.h>
#include <linux/sort.h>
#include <linux/kthread.h>
#include <linux/slab.h>
#include <linux/vmalloc.h>


MODULE_LICENSE("GPL");
MODULE_AUTHOR("daviderovell0");
MODULE_DESCRIPTION("Test round (timer + memory access) accuracy for sync CXL replication");
MODULE_VERSION("0.1");

// params
static uint64_t num_iterations = 1000000;
static uint64_t interval_ns = 1000;

static struct task_struct *loop_task;

int is_larger(const void *a, const void *b)
{
    return (*(uint64_t *)a - *(uint64_t *)b);
}

int round_test(void *data)
{
	uint64_t prev_time, cur_time;
    uint64_t *times = vzalloc(num_iterations * sizeof(uint64_t));
    // uint64_t total_time = 0;
    uint64_t p0, p50, p90, p99, p999, max;

    if (times == NULL) {
        pr_err("Memory allocation failed for times array\n");
        return -ENOMEM;
    }

	pr_info("Starting with interval: %llu\n", interval_ns);

	prev_time = ktime_get_ns();
    cur_time = ktime_get_ns();

	for (int i = 0; i < num_iterations; i++) {
        cur_time = ktime_get_ns();
		while ((cur_time - prev_time) < interval_ns) {
            // busy loop
			cur_time = ktime_get_ns();
		}
		times[i] = cur_time - prev_time;
        prev_time = cur_time;

        // Optional: Print progress every 10000 iterations
        if (i % 10000 == 0) {
            pr_info("Iteration %d/%llu completed\n", i, num_iterations);
        }
	} // end of loop


    // sort the measured times in ascending order
    sort(times, num_iterations, sizeof(uint64_t), is_larger, NULL);
    // calculate and print the percentiles
    p0 = times[0];
    p50 = times[num_iterations / 2];
    p90 = times[(90 * num_iterations) / 100];
    p99 = times[(99 * num_iterations) / 100];
    p999 = times[(999 * num_iterations) / 1000];
    max = times[num_iterations - 1];

    pr_info("Timer test results:\n");
    pr_info("Min: %llu ns\n", p0);
    pr_info("P50: %llu ns\n", p50);
    pr_info("P90: %llu ns\n", p90);
    pr_info("P99: %llu ns\n", p99);
    pr_info("P999: %llu ns\n", p999);
    pr_info("Max: %llu ns\n", max);

    vfree(times);
    return 0;
}

static int __init lkm_init(void)
{
    pr_info("Round accuracy module loaded\n");

    loop_task = kthread_create(&round_test, NULL, "roundtest_thread");
    kthread_bind(loop_task, 8); // Bind to CPU 8
    
    if (IS_ERR(loop_task)) {
		pr_err("Task Error. %s\n", __func__);
		return -EINVAL;
	}

	wake_up_process(loop_task);

    return 0;
}

static void __exit lkm_exit(void)
{
    pr_info("Round accuracy module unloaded\n");

    // kthread_stop(loop_task);

}

module_init(lkm_init);
module_exit(lkm_exit);
