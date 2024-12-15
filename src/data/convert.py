import pandas as pd

# Replace 'your_file.csv' with the path to your CSV file
input_file = 'C:/Users/dlisz/Desktop/Rust Projects/First Rust Project/my_project/src/data/2018-09-01-2024-Bitfinex_Spot-4h.csv'
output_file = 'C:/Users/dlisz/Desktop/Rust Projects/First Rust Project/my_project/src/data/2018-09-01-2024-Bitfinex_Spot-4h-puta.csv'

# Read the CSV file
df = pd.read_csv(input_file)

# Drop the first column and shift the data to the left
df = df.iloc[:, 1:]

# Save the updated DataFrame back to a CSV file
df.to_csv(output_file, index=False)

print(f"The first column has been removed and the updated file is saved as '{output_file}'.")
