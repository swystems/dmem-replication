/* Copyright (C) 2024 Canonical Ltd

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License version 3 as
   published by the Free Software Foundation.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

/* Build with gcc edf.c -o edf */

#define _GNU_SOURCE

#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <syscall.h>
#include <time.h>
#include <unistd.h>
#include <sched.h>
#include <string.h>

#define FIRST_CORE 1


// params
static const uint64_t num_iterations = 10000000;
static const uint64_t interval_ns = 1000;

struct sched_attr
{
    uint32_t size;
    uint32_t sched_policy;
    uint64_t sched_flags;
    int32_t sched_nice;
    uint32_t sched_priority;
    uint64_t sched_runtime;
    uint64_t sched_deadline;
    uint64_t sched_period;
};

struct sched_attr attr2 = {
    .size = sizeof(struct sched_attr),
    .sched_policy = SCHED_DEADLINE,
    .sched_runtime = 1, //us
    .sched_deadline = 1, //us
    .sched_period = interval_ns / 1000, // watch out integer division
};



uint64_t get_time_ns(void)
{
    struct timespec ts;
    if (0 != clock_gettime(CLOCK_MONOTONIC, &ts))
    {
        perror("clock_gettime");
        exit(EXIT_FAILURE);
    }
    return (uint64_t)ts.tv_sec * 1000000000 + ts.tv_nsec;
}

int is_larger(const void *a, const void *b)
{
    return (*(uint64_t *)a - *(uint64_t *)b);
}

int first_index_of(uint64_t *array, uint64_t value, int size)
{
    for (int i = 0; i < size; i++)
    {
        if (array[i] == value)
        {
            return i;
        }
    }
    return -1; // Not found
}

// void *thread_start(void *arg)
// {
//     int thread_num = (intptr_t)arg;
//     static bool set_thread1_attr = true;
//     static bool set_thread2_attr = true;

//     static int calls_remaining = 100000;
//     do
//     {
//         if (1 == thread_num)
//         {
//             if (set_thread1_attr)
//             {
//                 syscall(SYS_sched_setattr, 0, &attr1, 0);
//                 set_thread1_attr = false;
//             }
//             calls_on_thread1++;
//         }
//         else
//         {
//             if (set_thread2_attr)
//             {
//                 syscall(SYS_sched_setattr, 0, &attr2, 0);
//                 set_thread2_attr = false;
//             }
//             calls_on_thread2++;
//         }

//         /* Simulate doing some work */
//         for (int i = 0; i < 20000; i++)
//             ;

//     } while (--calls_remaining > 0);

//     return (void *)NULL;
// }

int main(void)
{
    uint64_t *times, *times_sorted;
    times = malloc(num_iterations * sizeof(uint64_t));
    times_sorted = malloc(num_iterations * sizeof(uint64_t));
    if (!times || !times_sorted)
    {
        perror("Failed to allocate memory for times");
        exit(EXIT_FAILURE);
    }

    /*-- Run on first core only --*/

    cpu_set_t cpu_set;
    CPU_ZERO(&cpu_set);
    CPU_SET(FIRST_CORE, &cpu_set);
    if (0 != sched_setaffinity(0, sizeof(cpu_set), &cpu_set))
    {
        printf("Failed to set CPU affinity\n");
        exit(EXIT_FAILURE);
    }

    //set the scheduling policy to SCHED_DEADLINE
    syscall(SYS_sched_setattr, 0, &attr2, 0);

    for (int i = 0; i < num_iterations; i++)
    {
        times[i] = get_time_ns();
        while (get_time_ns() - times[i] < interval_ns); // wait for the next interval
        // if (i > 0)
        times[i] = get_time_ns() - times[i];
        // printf("time[%d]: %lu ns\n", i, times[i]);
    }

    // sort the measured times in ascending order
    memcpy(times_sorted, times, num_iterations * sizeof(uint64_t));
    qsort(times_sorted, num_iterations, sizeof(uint64_t), is_larger);
    // calculate and print the percentiles
    uint64_t p0 = times_sorted[0];
    uint64_t p50 = times_sorted[num_iterations / 2];
    uint64_t p90 = times_sorted[(90 * num_iterations) / 100];
    uint64_t p99 = times_sorted[(99 * num_iterations) / 100];
    uint64_t p999 = times_sorted[(999 * num_iterations) / 1000];
    uint64_t p9999 = times_sorted[(9999 * num_iterations) / 10000];
    uint64_t max = times_sorted[num_iterations - 1];

    printf("Timer test results:\n");
    printf("Min: %lu ns, index: %d\n", p0, first_index_of(times, p0, num_iterations));
    printf("P50: %lu ns, index: %d\n", p50, first_index_of(times, p50, num_iterations));
    printf("P90: %lu ns, index: %d\n", p90, first_index_of(times, p90, num_iterations));
    printf("P99: %lu ns, index: %d\n", p99, first_index_of(times, p99, num_iterations));
    printf("P999: %lu ns, index: %d\n", p999, first_index_of(times, p999, num_iterations));
    printf("P9999: %lu ns, index: %d\n", p9999, first_index_of(times, p9999, num_iterations));
    printf("Max: %lu ns, index: %d\n", max, first_index_of(times, max, num_iterations));

    free(times);
    free(times_sorted);

    exit(EXIT_SUCCESS);
}
