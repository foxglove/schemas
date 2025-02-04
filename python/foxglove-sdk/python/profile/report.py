import pstats


head = 20

try:
    pstats.Stats("pb2_wrappers.prof").sort_stats("time").print_stats(head)
except FileNotFoundError:
    print("pb2_wrappers.prof not found")


try:
    pstats.Stats("pyo3_wrappers.prof").sort_stats("time").print_stats(head)
except FileNotFoundError:
    print("pyo3_wrappers.prof not found")
