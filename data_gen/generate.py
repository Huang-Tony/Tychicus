# Tychicus/data_gen/generate.py
import os
import random
import string
import sys

def generate_dummy_file(filename, size_in_gb):
    print(f"Generating {filename} ({size_in_gb} GB)... This may take a minute.")
    chunk_size = 1024 * 1024  # 1 MB chunks
    # We use a fixed buffer to save RAM, repeating it is fine for volume tests
    chars = string.ascii_letters + string.digits + " "
    base_chunk = ''.join(random.choices(chars, k=chunk_size))
    
    with open(filename, 'w') as f:
        for i in range(int(size_in_gb * 1024)):
            # Mutate the chunk slightly so hashes aren't identical
            current_chunk = base_chunk[:-10] + f"{i:010d}"
            f.write(current_chunk)
            
            # Progress bar
            if i % 100 == 0:
                sys.stdout.write(f"\rProgress: {i // 1024 * 100 / size_in_gb:.1f}%")
                sys.stdout.flush()
    
    print(f"\n✅ Done! Created {filename}")

if __name__ == "__main__":
    # 1. Generate the Clean Data (Choose 1, 5, or 10)
    generate_dummy_file("dataset_10gb.txt", 10)
    
    # 2. Generate a small "Bad" file for the Compliance Test
    with open("suspicious_file.txt", "w") as f:
        f.write("This is a normal file but wait... here is the prohibited text: restricted_ip ...oops.")
    print("✅ Created suspicious_file.txt for Phase 3 testing.")