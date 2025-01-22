import matplotlib.pyplot as plt

def parse_results(file_path):
    with open(file_path, 'r') as f:
        message = f.read()

    results_start = message.find("Results:")
    if results_start == -1:
        raise ValueError("No 'Results:' section found in the input message.")
    results_section = message[results_start:].strip()
    table_lines = results_section.splitlines()

    data_lines = [line for line in table_lines if "|" in line and not line.startswith("|---")]
    
    if len(data_lines) < 2:
        raise ValueError("Invalid table format: Unable to parse header or data rows.")

    headers = [header.strip() for header in data_lines[0].split("|")[1:-1]]

    results = {}
    for row in data_lines[1:]:
        cells = [cell.strip() for cell in row.split("|")[1:-1]]
        if len(cells) != len(headers):
            continue
        engine = cells[0]
        try:
            result = float(cells[1])
            results[engine] = result
        except ValueError:
            continue
    return headers, results

def generate_chart(results, output_file="chart.png"):
    engines = list(results.keys())
    times = list(results.values())

    plt.figure(figsize=(12, 8))
    bars = plt.bar(engines, times)

    plt.xlabel("Engine", fontsize=14)
    plt.ylabel("Result (ms)", fontsize=14)
    plt.title("Coremark Benchmark Results", fontsize=16, fontweight="bold")

    for bar, time in zip(bars, times):
        height = bar.get_height()
        plt.text(bar.get_x() + bar.get_width() / 2, height, f"{time:.2f}", ha="center", va="bottom", fontsize=10)

    plt.tight_layout()
    plt.savefig(output_file)
    print(f"Chart saved as {output_file}")

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 3:
        print("Usage: python benchmark_chart.py <input_file> <output_file>")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2]

    try:
        _, results = parse_results(input_file)
        generate_chart(results, output_file)
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)
