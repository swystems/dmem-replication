#include <linux/module.h>
#include <linux/ktime.h>
#include <linux/sort.h>
#include <linux/kthread.h>
#include <linux/slab.h>
#include <linux/vmalloc.h>
#include <linux/sched.h>
// #include <linux/sched/types.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("daviderovell0");
MODULE_DESCRIPTION("Test round (timer + memory access) accuracy for sync CXL replication");
MODULE_VERSION("0.1");

// params
static uint64_t num_iterations = 100000000;
static uint64_t interval_ns = 1000;

static struct task_struct *loop_task;
//struct sched_param {
//    int sched_priority;
//} param;


int is_larger(const void *a, const void *b)
{
    return (*(uint64_t *)a - *(uint64_t *)b);
}

int first_index_of(uint64_t *array, uint64_t value, int size)
{
    for (int i = 0; i < size; i++) {
        if (array[i] == value) {
            return i;
        }
    }
    return -1; // Not found
}

int round_test(void *data)
{
	uint64_t prev_time, cur_time;
    uint64_t *times = vzalloc(num_iterations * sizeof(uint64_t));
    uint64_t *times_sorted = vzalloc(num_iterations * sizeof(uint64_t));
    uint64_t p0, p50, p90, p99, p999, p9999, max;

    if (times == NULL) {
        pr_err("Memory allocation failed for times array\n");
        return -ENOMEM;
    }

	pr_info("Starting with interval: %llu\n", interval_ns);

	prev_time = ktime_get_ns();
    cur_time = ktime_get_ns();

	for (int i = 0; i < num_iterations; i++) {
        prev_time = cur_time = ktime_get_ns();
		while ((cur_time - prev_time) < interval_ns) {
            // busy loop
			cur_time = ktime_get_ns();
		}
		times[i] = cur_time - prev_time;

        // Optional: Print progress every 10000 iterations
        if (i % 10000 == 0) {
            pr_info("Iteration %d/%llu completed\n", i, num_iterations);
        }
	} // end of loop


    // sort the measured times in ascending order
    memcpy(times_sorted, times, num_iterations * sizeof(uint64_t));
    sort(times_sorted, num_iterations, sizeof(uint64_t), is_larger, NULL);
    // calculate and print the percentiles
    p0 = times_sorted[0];
    p50 = times_sorted[num_iterations / 2];
    p90 = times_sorted[(90 * num_iterations) / 100];
    p99 = times_sorted[(99 * num_iterations) / 100];
    p999 = times_sorted[(999 * num_iterations) / 1000];
    p9999 = times_sorted[(9999 * num_iterations) / 10000];
    max = times_sorted[num_iterations - 1];

    pr_info("Timer test results:\n");
    pr_info("Min: %llu ns, index: %d\n", p0, first_index_of(times, p0, num_iterations));
    pr_info("P50: %llu ns, index: %d\n", p50, first_index_of(times, p50, num_iterations));
    pr_info("P90: %llu ns, index: %d\n", p90, first_index_of(times, p90, num_iterations));
    pr_info("P99: %llu ns, index: %d\n", p99, first_index_of(times, p99, num_iterations));
    pr_info("P999: %llu ns, index: %d\n", p999, first_index_of(times, p999, num_iterations));
    pr_info("P9999: %llu ns, index: %d\n", p9999, first_index_of(times, p9999, num_iterations));
    pr_info("Max: %llu ns, index: %d\n", max, first_index_of(times, max, num_iterations));

    vfree(times);
    return 0;
}

static int __init lkm_init(void)
{
    pr_info("Round accuracy module loaded\n");

    // param.sched_priority = 80 ; // Priority 1-99
    loop_task = kthread_create(&round_test, NULL, "roundtest_thread");
    
    // sched_setscheduler(loop_task, SCHED_FIFO, &param);
    sched_set_fifo(loop_task);
    kthread_bind(loop_task, 1);
    
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
