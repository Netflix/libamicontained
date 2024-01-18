#ifndef __CPUINFO_H
#define __CPUINFO_H

/*
 * numcpus returns the number of CPUs this task has access to, or a negative
 * error number on failure.
 */
extern int num_cpus(void);

/*
 * If the scheduler configuration may allow a substantially different number of
 * cpus to be accessed at different times (e.g. via quotas, shares, etc. vs.
 * strict cpusets), it may be desirable to configure a number of threads that
 * is not exactly the number of CPUs the application has access to at this
 * moment.
 *
 * Returns the recommended number of threads, or a negative error number on
 * failure.
 */
extern int recommended_threads(void);

#endif
