import pickle
import pandas as pd
import subprocess

def unit_to_ns(unit):
    unit_type = unit[-2:]
    val = float(unit[:-2])
    if unit_type == "µs" or unit_type == "us":
        return val *1000        
    if unit_type == "ms":
        return val * 1000000
    if unit_type == "ns":
        return val

def interval_to_unit(interval):
    if interval < 1000:
        return f"{interval} ns"
    if interval < 1000000:
        return f"{interval/1000} µs"
    if interval < 1000000000:
        return f"{interval/1000000} ms"
        return val
        
def parse_result(result):
    stdout_lines = result.stdout.splitlines()
    stderr_lines = result.stderr.splitlines()
    stderr_lines = [l.strip() for l in stderr_lines]
    data = {}
    data["avg"] = stdout_lines[1].split(": ")[1]
    data["std"] = stdout_lines[2].split(": ")[1]
    data["max"] = stdout_lines[3].split(": ")[1]
    data["min"] = stdout_lines[4].split(": ")[1]
    data["jit"] = stdout_lines[5].split(": ")[1]

    data_ns = {k: unit_to_ns(v) for k,v in data.items()}
    # print(f"Parsed data: {stderr_lines}")
    data["cpu_cycles"] = data_ns["cpu_cycles"] = stderr_lines[3].split(" ")[0]
    
    return data, data_ns
    
program = "target/release/bpsleep"
# intervals = [10, 15, 100, 150, 1000, 1500, 10000, 
#                  15000,30000,50000, 70000, 80000, 90000, 100000,
#                  150000, 1000000, 1500000, 10000000, 15000000]
# intervals = [10, 100, 1000, 10000, 100000, 1000000]
intervals = [10, 100, 1000, 10000, 100000, 1000000]
# intervals = [10]

# sleep to busypoll ratios: 0 -> only busy-poll, 1 -> only sleep
ratios = [0, 0.001, 0.01, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
attempts = 10000
interval_to_err_to_threshold = {}

timer_accuracy_ns = {}

for interval in intervals:
    interval_results_pretty = []
    interval_results_ns = []
    for r in ratios:
        # perf stat -- target/release/performance -s 10000 -a 100000 -r 0
        result = subprocess.run(["perf", "stat", "-e", "cycles", "--", program, "-r", f"{r}",  "-s", f"{interval}", "-a", f"{attempts}"], capture_output=True, text=True)
        out,out_ns = parse_result(result)
        interval_results_pretty.append(out)
        interval_results_ns.append(out_ns)

    print(f"The results for an interval of {interval_to_unit(interval)} are: ")
    print(pd.DataFrame(interval_results_pretty, index=ratios))
    timer_accuracy_ns[interval] = pd.DataFrame(interval_results_ns, index=ratios)

# Save the results to a pickle file
with open("timer_accuracy_ns.pkl", "wb") as f:
    pickle.dump(timer_accuracy_ns, f)